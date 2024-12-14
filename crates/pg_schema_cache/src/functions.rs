use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

/// `Behavior` describes the characteristics of the function. Is it deterministic? Does it changed due to side effects, and if so, when?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Behavior {
    /// The function is a pure function (same input leads to same output.)
    Immutable,

    /// The results of the function do not change within a scan.
    Stable,

    /// The results of the function might change at any time.
    #[default]
    Volatile,
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
    /// `in`, `out`, or `inout`.
    pub mode: String,

    pub name: String,

    /// Refers to the argument type's ID in the `pg_type` table.
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
    /// The Id (`oid`).
    pub id: i64,

    /// The name of the schema the function belongs to.
    pub schema: String,

    /// The name of the function.
    pub name: String,

    /// e.g. `plpgsql/sql` or `internal`.
    pub language: String,

    /// The body of the function – the `declare [..] begin [..] end [..]` block.` Not set for internal functions.
    pub body: Option<String>,

    /// The full definition of the function. Includes the full `CREATE OR REPLACE...` shenanigans. Not set for internal functions.
    pub definition: Option<String>,

    /// The Rust representation of the function's arguments.
    pub args: FunctionArgs,

    /// Comma-separated list of argument types, in the form required for a CREATE FUNCTION statement. For example, `"text, smallint"`. `None` if the function doesn't take any arguments.
    pub argument_types: Option<String>,

    /// Comma-separated list of argument types, in the form required to identify a function in an ALTER FUNCTION statement. For example, `"text, smallint"`. `None` if the function doesn't take any arguments.
    pub identity_argument_types: Option<String>,

    /// An ID identifying the return type. For example, `2275` refers to `cstring`. 2278 refers to `void`.
    pub return_type_id: i64,

    /// The return type, for example "text", "trigger", or "void".
    pub return_type: String,

    /// If the return type is a composite type, this will point the matching entry's `oid` column in the `pg_class` table. `None` if the function does not return a composite type.
    pub return_type_relation_id: Option<i64>,

    /// Does the function returns multiple values of a data type?
    pub is_set_returning_function: bool,

    /// See `Behavior`.
    pub behavior: Behavior,

    /// Is the function's security set to `Definer` (true) or `Invoker` (false)?
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
    oid,
    proname,
    prosrc,
    prorettype,
    proretset,
    provolatile,
    prosecdef,
    prolang,
    pronamespace,
    proconfig,

    -- proargmodes is null when all arg modes are IN
    coalesce(
      p.proargmodes,
      array_fill(
        'i' :: text,
        array [cardinality(coalesce(p.proallargtypes, p.proargtypes))]
      )
    ) as arg_modes,
    -- proargnames is null when all args are unnamed
    coalesce(
      p.proargnames,
      array_fill(
        '' :: text,
        array [cardinality(coalesce(p.proallargtypes, p.proargtypes))]
      )
    ) as arg_names,
    -- proallargtypes is null when all arg modes are IN
    coalesce(p.proallargtypes, p.proargtypes) as arg_types,
    array_cat(
      array_fill(false, array [pronargs - pronargdefaults]),
      array_fill(true, array [pronargdefaults])
    ) as arg_has_defaults
  from
    pg_proc as p
  where
    p.prokind = 'f'
)
select
  f.oid :: int8 as "id!",
  n.nspname as "schema!",
  f.proname as "name!",
  l.lanname as "language!",
  case
    when l.lanname = 'internal' then ''
    else f.prosrc
  end as body,
  case
    when l.lanname = 'internal' then ''
    else pg_get_functiondef(f.oid)
  end as definition,
  coalesce(f_args.args, '[]') as args,
  nullif(pg_get_function_arguments(f.oid), '') as argument_types,
  nullif(pg_get_function_identity_arguments(f.oid), '') as identity_argument_types,
  f.prorettype :: int8 as "return_type_id!",
  pg_get_function_result(f.oid) as "return_type!",
  nullif(rt.typrelid :: int8, 0) as return_type_relation_id,
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
      jsonb_object_agg(param, value) filter (
        where
          param is not null
      ) as config_params
    from
      (
        select
          oid,
          (string_to_array(unnest(proconfig), '=')) [1] as param,
          (string_to_array(unnest(proconfig), '=')) [2] as value
        from
          functions
      ) as t
    group by
      oid
  ) f_config on f_config.oid = f.oid
  left join (
    select
      oid,
      jsonb_agg(
        jsonb_build_object(
          'mode',
          t2.mode,
          'name',
          name,
          'type_id',
          type_id,
          'has_default',
          has_default
        )
      ) as args
    from
      (
        select
          oid,
          unnest(arg_modes) as mode,
          unnest(arg_names) as name,
          unnest(arg_types) :: int8 as type_id,
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
  ) f_args on f_args.oid = f.oid;
        "#
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }
}
