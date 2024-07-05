use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Behavior {
    Immutable,
    Stable,
    Volatile,
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior::Volatile
    }
}

impl From<Option<String>> for Behavior {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => match s.as_str() {
                "IMMUTABLE" => Behavior::Immutable,
                "STABLE" => Behavior::Stable,
                "VOLATILE" => Behavior::Volatile,
                _ => panic!("Invalid behavior"),
            },
            None => Behavior::Volatile,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionArg {
    pub mode: String,
    pub name: String,
    pub type_id: i64,
    pub has_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionArgs {
    pub args: Vec<FunctionArg>,
}

impl From<Option<JsonValue>> for FunctionArgs {
    fn from(s: Option<JsonValue>) -> Self {
        let args: Vec<FunctionArg> =
            serde_json::from_value(s.unwrap_or(JsonValue::Array(vec![]))).unwrap();
        FunctionArgs { args }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Function {
    pub id: Option<i64>,
    pub schema: Option<String>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub definition: Option<String>,
    pub complete_statement: Option<String>,
    pub args: FunctionArgs,
    pub argument_types: Option<String>,
    pub identity_argument_types: Option<String>,
    pub return_type_id: Option<i64>,
    pub return_type: Option<String>,
    pub return_type_relation_id: Option<i64>,
    pub is_set_returning_function: bool,
    pub behavior: Behavior,
    pub security_definer: bool,
}

impl SchemaCacheItem for Function {
    type Item = Function;

    async fn load(pool: &PgPool) -> Vec<Function> {
        sqlx::query_as!(
            Function,
            r#"
with functions as (
  select
    *,
    -- proargmodes is null when all arg modes are IN
    coalesce(
      p.proargmodes,
      array_fill('i'::text, array[cardinality(coalesce(p.proallargtypes, p.proargtypes))])
    ) as arg_modes,
    -- proargnames is null when all args are unnamed
    coalesce(
      p.proargnames,
      array_fill(''::text, array[cardinality(coalesce(p.proallargtypes, p.proargtypes))])
    ) as arg_names,
    -- proallargtypes is null when all arg modes are IN
    coalesce(p.proallargtypes, p.proargtypes) as arg_types,
    array_cat(
      array_fill(false, array[pronargs - pronargdefaults]),
      array_fill(true, array[pronargdefaults])) as arg_has_defaults
  from
    pg_proc as p
  where
    p.prokind = 'f'
)
select
  f.oid::int8 as id,
  n.nspname as schema,
  f.proname as name,
  l.lanname as language,
  case
    when l.lanname = 'internal' then ''
    else f.prosrc
  end as definition,
  case
    when l.lanname = 'internal' then f.prosrc
    else pg_get_functiondef(f.oid)
  end as complete_statement,
  coalesce(f_args.args, '[]') as args,
  pg_get_function_arguments(f.oid) as argument_types,
  pg_get_function_identity_arguments(f.oid) as identity_argument_types,
  f.prorettype::int8 as return_type_id,
  pg_get_function_result(f.oid) as return_type,
  nullif(rt.typrelid::int8, 0) as return_type_relation_id,
  f.proretset as is_set_returning_function,
  case
    when f.provolatile = 'i' then 'IMMUTABLE'
    when f.provolatile = 's' then 'STABLE'
    when f.provolatile = 'v' then 'VOLATILE'
  end as behavior,
  f.prosecdef as security_definer
from
  functions f
  left join pg_namespace n on f.pronamespace = n.oid
  left join pg_language l on f.prolang = l.oid
  left join pg_type rt on rt.oid = f.prorettype
  left join (
    select
      oid,
      jsonb_object_agg(param, value) filter (where param is not null) as config_params
    from
      (
        select
          oid,
          (string_to_array(unnest(proconfig), '='))[1] as param,
          (string_to_array(unnest(proconfig), '='))[2] as value
        from
          functions
      ) as t
    group by
      oid
  ) f_config on f_config.oid = f.oid
  left join (
    select
      oid,
      jsonb_agg(jsonb_build_object(
        'mode', t2.mode,
        'name', name,
        'type_id', type_id,
        'has_default', has_default
      )) as args
    from
      (
        select
          oid,
          unnest(arg_modes) as mode,
          unnest(arg_names) as name,
          unnest(arg_types)::int8 as type_id,
          unnest(arg_has_defaults) as has_default
        from
          functions
      ) as t1,
      lateral (
        select
          case
            when t1.mode = 'i' then 'in'
            when t1.mode = 'o' then 'out'
            when t1.mode = 'b' then 'inout'
            when t1.mode = 'v' then 'variadic'
            else 'table'
          end as mode
      ) as t2
    group by
      t1.oid
  ) f_args on f_args.oid = f.oid"#
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }
}
