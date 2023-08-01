//! Utilities for working with pg_query.rs
//! This file is generated from the libg_query proto
use pg_query::{NodeEnum, NodeRef};

pub fn get_location(node: NodeEnum) -> Option<i32> {
    match &node {
        NodeEnum::Alias(_) => None,
        NodeEnum::RangeVar(n) => Some(n.location),
        NodeEnum::TableFunc(n) => Some(n.location),
        NodeEnum::Var(n) => Some(n.location),
        NodeEnum::Param(n) => Some(n.location),
        NodeEnum::Aggref(n) => Some(n.location),
        NodeEnum::GroupingFunc(n) => Some(n.location),
        NodeEnum::WindowFunc(n) => Some(n.location),
        NodeEnum::SubscriptingRef(_) => None,
        NodeEnum::FuncExpr(n) => Some(n.location),
        NodeEnum::NamedArgExpr(n) => Some(n.location),
        NodeEnum::OpExpr(n) => Some(n.location),
        NodeEnum::DistinctExpr(n) => Some(n.location),
        NodeEnum::NullIfExpr(n) => Some(n.location),
        NodeEnum::ScalarArrayOpExpr(n) => Some(n.location),
        NodeEnum::BoolExpr(n) => Some(n.location),
        NodeEnum::SubLink(n) => Some(n.location),
        NodeEnum::SubPlan(_) => None,
        NodeEnum::AlternativeSubPlan(_) => None,
        NodeEnum::FieldSelect(_) => None,
        NodeEnum::FieldStore(_) => None,
        NodeEnum::RelabelType(n) => Some(n.location),
        NodeEnum::CoerceViaIo(n) => Some(n.location),
        NodeEnum::ArrayCoerceExpr(n) => Some(n.location),
        NodeEnum::ConvertRowtypeExpr(n) => Some(n.location),
        NodeEnum::CollateExpr(n) => Some(n.location),
        NodeEnum::CaseExpr(n) => Some(n.location),
        NodeEnum::CaseWhen(n) => Some(n.location),
        NodeEnum::CaseTestExpr(_) => None,
        NodeEnum::ArrayExpr(n) => Some(n.location),
        NodeEnum::RowExpr(n) => Some(n.location),
        NodeEnum::RowCompareExpr(_) => None,
        NodeEnum::CoalesceExpr(n) => Some(n.location),
        NodeEnum::MinMaxExpr(n) => Some(n.location),
        NodeEnum::SqlvalueFunction(n) => Some(n.location),
        NodeEnum::XmlExpr(n) => Some(n.location),
        NodeEnum::NullTest(n) => Some(n.location),
        NodeEnum::BooleanTest(n) => Some(n.location),
        NodeEnum::CoerceToDomain(n) => Some(n.location),
        NodeEnum::CoerceToDomainValue(n) => Some(n.location),
        NodeEnum::SetToDefault(n) => Some(n.location),
        NodeEnum::CurrentOfExpr(_) => None,
        NodeEnum::NextValueExpr(_) => None,
        NodeEnum::InferenceElem(_) => None,
        NodeEnum::TargetEntry(_) => None,
        NodeEnum::RangeTblRef(_) => None,
        NodeEnum::JoinExpr(_) => None,
        NodeEnum::FromExpr(_) => None,
        NodeEnum::OnConflictExpr(_) => None,
        NodeEnum::IntoClause(_) => None,
        NodeEnum::MergeAction(_) => None,
        NodeEnum::RawStmt(_) => None,
        NodeEnum::Query(_) => None,
        NodeEnum::InsertStmt(_) => None,
        NodeEnum::DeleteStmt(_) => None,
        NodeEnum::UpdateStmt(_) => None,
        NodeEnum::MergeStmt(_) => None,
        NodeEnum::SelectStmt(_) => None,
        NodeEnum::ReturnStmt(_) => None,
        NodeEnum::PlassignStmt(n) => Some(n.location),
        NodeEnum::AlterTableStmt(_) => None,
        NodeEnum::AlterTableCmd(_) => None,
        NodeEnum::AlterDomainStmt(_) => None,
        NodeEnum::SetOperationStmt(_) => None,
        NodeEnum::GrantStmt(_) => None,
        NodeEnum::GrantRoleStmt(_) => None,
        NodeEnum::AlterDefaultPrivilegesStmt(_) => None,
        NodeEnum::ClosePortalStmt(_) => None,
        NodeEnum::ClusterStmt(_) => None,
        NodeEnum::CopyStmt(_) => None,
        NodeEnum::CreateStmt(_) => None,
        NodeEnum::DefineStmt(_) => None,
        NodeEnum::DropStmt(_) => None,
        NodeEnum::TruncateStmt(_) => None,
        NodeEnum::CommentStmt(_) => None,
        NodeEnum::FetchStmt(_) => None,
        NodeEnum::IndexStmt(_) => None,
        NodeEnum::CreateFunctionStmt(_) => None,
        NodeEnum::AlterFunctionStmt(_) => None,
        NodeEnum::DoStmt(_) => None,
        NodeEnum::RenameStmt(_) => None,
        NodeEnum::RuleStmt(_) => None,
        NodeEnum::NotifyStmt(_) => None,
        NodeEnum::ListenStmt(_) => None,
        NodeEnum::UnlistenStmt(_) => None,
        NodeEnum::TransactionStmt(_) => None,
        NodeEnum::ViewStmt(_) => None,
        NodeEnum::LoadStmt(_) => None,
        NodeEnum::CreateDomainStmt(_) => None,
        NodeEnum::CreatedbStmt(_) => None,
        NodeEnum::DropdbStmt(_) => None,
        NodeEnum::VacuumStmt(_) => None,
        NodeEnum::ExplainStmt(_) => None,
        NodeEnum::CreateTableAsStmt(_) => None,
        NodeEnum::CreateSeqStmt(_) => None,
        NodeEnum::AlterSeqStmt(_) => None,
        NodeEnum::VariableSetStmt(_) => None,
        NodeEnum::VariableShowStmt(_) => None,
        NodeEnum::DiscardStmt(_) => None,
        NodeEnum::CreateTrigStmt(_) => None,
        NodeEnum::CreatePlangStmt(_) => None,
        NodeEnum::CreateRoleStmt(_) => None,
        NodeEnum::AlterRoleStmt(_) => None,
        NodeEnum::DropRoleStmt(_) => None,
        NodeEnum::LockStmt(_) => None,
        NodeEnum::ConstraintsSetStmt(_) => None,
        NodeEnum::ReindexStmt(_) => None,
        NodeEnum::CheckPointStmt(_) => None,
        NodeEnum::CreateSchemaStmt(_) => None,
        NodeEnum::AlterDatabaseStmt(_) => None,
        NodeEnum::AlterDatabaseRefreshCollStmt(_) => None,
        NodeEnum::AlterDatabaseSetStmt(_) => None,
        NodeEnum::AlterRoleSetStmt(_) => None,
        NodeEnum::CreateConversionStmt(_) => None,
        NodeEnum::CreateCastStmt(_) => None,
        NodeEnum::CreateOpClassStmt(_) => None,
        NodeEnum::CreateOpFamilyStmt(_) => None,
        NodeEnum::AlterOpFamilyStmt(_) => None,
        NodeEnum::PrepareStmt(_) => None,
        NodeEnum::ExecuteStmt(_) => None,
        NodeEnum::DeallocateStmt(_) => None,
        NodeEnum::DeclareCursorStmt(_) => None,
        NodeEnum::CreateTableSpaceStmt(_) => None,
        NodeEnum::DropTableSpaceStmt(_) => None,
        NodeEnum::AlterObjectDependsStmt(_) => None,
        NodeEnum::AlterObjectSchemaStmt(_) => None,
        NodeEnum::AlterOwnerStmt(_) => None,
        NodeEnum::AlterOperatorStmt(_) => None,
        NodeEnum::AlterTypeStmt(_) => None,
        NodeEnum::DropOwnedStmt(_) => None,
        NodeEnum::ReassignOwnedStmt(_) => None,
        NodeEnum::CompositeTypeStmt(_) => None,
        NodeEnum::CreateEnumStmt(_) => None,
        NodeEnum::CreateRangeStmt(_) => None,
        NodeEnum::AlterEnumStmt(_) => None,
        NodeEnum::AlterTsdictionaryStmt(_) => None,
        NodeEnum::AlterTsconfigurationStmt(_) => None,
        NodeEnum::CreateFdwStmt(_) => None,
        NodeEnum::AlterFdwStmt(_) => None,
        NodeEnum::CreateForeignServerStmt(_) => None,
        NodeEnum::AlterForeignServerStmt(_) => None,
        NodeEnum::CreateUserMappingStmt(_) => None,
        NodeEnum::AlterUserMappingStmt(_) => None,
        NodeEnum::DropUserMappingStmt(_) => None,
        NodeEnum::AlterTableSpaceOptionsStmt(_) => None,
        NodeEnum::AlterTableMoveAllStmt(_) => None,
        NodeEnum::SecLabelStmt(_) => None,
        NodeEnum::CreateForeignTableStmt(_) => None,
        NodeEnum::ImportForeignSchemaStmt(_) => None,
        NodeEnum::CreateExtensionStmt(_) => None,
        NodeEnum::AlterExtensionStmt(_) => None,
        NodeEnum::AlterExtensionContentsStmt(_) => None,
        NodeEnum::CreateEventTrigStmt(_) => None,
        NodeEnum::AlterEventTrigStmt(_) => None,
        NodeEnum::RefreshMatViewStmt(_) => None,
        NodeEnum::ReplicaIdentityStmt(_) => None,
        NodeEnum::AlterSystemStmt(_) => None,
        NodeEnum::CreatePolicyStmt(_) => None,
        NodeEnum::AlterPolicyStmt(_) => None,
        NodeEnum::CreateTransformStmt(_) => None,
        NodeEnum::CreateAmStmt(_) => None,
        NodeEnum::CreatePublicationStmt(_) => None,
        NodeEnum::AlterPublicationStmt(_) => None,
        NodeEnum::CreateSubscriptionStmt(_) => None,
        NodeEnum::AlterSubscriptionStmt(_) => None,
        NodeEnum::DropSubscriptionStmt(_) => None,
        NodeEnum::CreateStatsStmt(_) => None,
        NodeEnum::AlterCollationStmt(_) => None,
        NodeEnum::CallStmt(_) => None,
        NodeEnum::AlterStatsStmt(_) => None,
        NodeEnum::AExpr(n) => Some(n.location),
        NodeEnum::ColumnRef(n) => Some(n.location),
        NodeEnum::ParamRef(n) => Some(n.location),
        NodeEnum::FuncCall(n) => Some(n.location),
        NodeEnum::AStar(_) => None,
        NodeEnum::AIndices(_) => None,
        NodeEnum::AIndirection(_) => None,
        NodeEnum::AArrayExpr(n) => Some(n.location),
        NodeEnum::ResTarget(n) => Some(n.location),
        NodeEnum::MultiAssignRef(_) => None,
        NodeEnum::TypeCast(n) => Some(n.location),
        NodeEnum::CollateClause(n) => Some(n.location),
        NodeEnum::SortBy(n) => Some(n.location),
        NodeEnum::WindowDef(n) => Some(n.location),
        NodeEnum::RangeSubselect(_) => None,
        NodeEnum::RangeFunction(_) => None,
        NodeEnum::RangeTableSample(n) => Some(n.location),
        NodeEnum::RangeTableFunc(n) => Some(n.location),
        NodeEnum::RangeTableFuncCol(n) => Some(n.location),
        NodeEnum::TypeName(n) => Some(n.location),
        NodeEnum::ColumnDef(n) => Some(n.location),
        NodeEnum::IndexElem(_) => None,
        NodeEnum::StatsElem(_) => None,
        NodeEnum::Constraint(n) => Some(n.location),
        NodeEnum::DefElem(n) => Some(n.location),
        NodeEnum::RangeTblEntry(_) => None,
        NodeEnum::RangeTblFunction(_) => None,
        NodeEnum::TableSampleClause(_) => None,
        NodeEnum::WithCheckOption(_) => None,
        NodeEnum::SortGroupClause(_) => None,
        NodeEnum::GroupingSet(n) => Some(n.location),
        NodeEnum::WindowClause(_) => None,
        NodeEnum::ObjectWithArgs(_) => None,
        NodeEnum::AccessPriv(_) => None,
        NodeEnum::CreateOpClassItem(_) => None,
        NodeEnum::TableLikeClause(_) => None,
        NodeEnum::FunctionParameter(_) => None,
        NodeEnum::LockingClause(_) => None,
        NodeEnum::RowMarkClause(_) => None,
        NodeEnum::XmlSerialize(n) => Some(n.location),
        NodeEnum::WithClause(n) => Some(n.location),
        NodeEnum::InferClause(n) => Some(n.location),
        NodeEnum::OnConflictClause(n) => Some(n.location),
        NodeEnum::CtesearchClause(n) => Some(n.location),
        NodeEnum::CtecycleClause(n) => Some(n.location),
        NodeEnum::CommonTableExpr(n) => Some(n.location),
        NodeEnum::MergeWhenClause(_) => None,
        NodeEnum::RoleSpec(n) => Some(n.location),
        NodeEnum::TriggerTransition(_) => None,
        NodeEnum::PartitionElem(n) => Some(n.location),
        NodeEnum::PartitionSpec(n) => Some(n.location),
        NodeEnum::PartitionBoundSpec(n) => Some(n.location),
        NodeEnum::PartitionRangeDatum(n) => Some(n.location),
        NodeEnum::PartitionCmd(_) => None,
        NodeEnum::VacuumRelation(_) => None,
        NodeEnum::PublicationObjSpec(n) => Some(n.location),
        NodeEnum::PublicationTable(_) => None,
        NodeEnum::InlineCodeBlock(_) => None,
        NodeEnum::CallContext(_) => None,
        NodeEnum::Integer(_) => None,
        NodeEnum::Float(_) => None,
        NodeEnum::Boolean(_) => None,
        NodeEnum::String(_) => None,
        NodeEnum::BitString(_) => None,
        NodeEnum::List(_) => None,
        NodeEnum::IntList(_) => None,
        NodeEnum::OidList(_) => None,
        NodeEnum::AConst(n) => Some(n.location),
    }
}

/// Returns the node and all its childrens, recursively
pub fn get_nested_nodes(node: NodeEnum) -> Vec<NodeEnum> {
    match &node {
        NodeEnum::Alias(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RangeVar(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::TableFunc(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .ns_uris
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .ns_names
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.docexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.rowexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .coltypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .coltypmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .colcollations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .colexprs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .coldefexprs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::Var(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::Param(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::Aggref(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .aggargtypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .aggdirectargs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .aggorder
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .aggdistinct
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.aggfilter.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::GroupingFunc(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .refs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::WindowFunc(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.aggfilter.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::SubscriptingRef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .refupperindexpr
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .reflowerindexpr
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.refexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.refassgnexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::FuncExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::NamedArgExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::OpExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DistinctExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::NullIfExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ScalarArrayOpExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::BoolExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::SubLink(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.testexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .oper_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.subselect.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::SubPlan(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.testexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .param_ids
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .set_param
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .par_param
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlternativeSubPlan(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .subplans
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::FieldSelect(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::FieldStore(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .newvals
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .fieldnums
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RelabelType(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CoerceViaIo(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::ArrayCoerceExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.elemexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::ConvertRowtypeExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CollateExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CaseExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.defresult.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CaseWhen(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.result.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CaseTestExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::ArrayExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .elements
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RowExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RowCompareExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .opnos
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .opfamilies
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .inputcollids
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .largs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .rargs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CoalesceExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::MinMaxExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::SqlvalueFunction(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::XmlExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .named_args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .arg_names
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::NullTest(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::BooleanTest(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CoerceToDomain(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CoerceToDomainValue(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::SetToDefault(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CurrentOfExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::NextValueExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::InferenceElem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::TargetEntry(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.xpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::RangeTblRef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::JoinExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.larg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.rarg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .using_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.join_using_alias.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.quals.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::FromExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .fromlist
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.quals.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::OnConflictExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .arbiter_elems
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.arbiter_where.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .on_conflict_set
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.on_conflict_where
                    .as_ref()
                    .unwrap()
                    .to_owned()
                    .node
                    .unwrap(),
            ));
            nodes.append(
                &mut n
                    .excl_rel_tlist
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::IntoClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.rel.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .col_names
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.view_query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::MergeAction(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.qual.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .update_colnos
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RawStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.stmt.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::Query(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.utility_stmt.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .cte_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .rtable
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::FromExpr(
                n.jointree.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .merge_action_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::OnConflictExpr(
                n.on_conflict.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .group_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .grouping_sets
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.having_qual.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .window_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .distinct_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .sort_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.limit_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.limit_count.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .row_marks
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.set_operations.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .constraint_deps
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .with_check_options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::InsertStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.select_stmt.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::OnConflictClause(
                n.on_conflict_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::WithClause(
                n.with_clause.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::DeleteStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .using_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::WithClause(
                n.with_clause.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::UpdateStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .from_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::WithClause(
                n.with_clause.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::MergeStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.source_relation.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.join_condition.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .merge_when_clauses
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::WithClause(
                n.with_clause.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::SelectStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .distinct_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::IntoClause(
                n.into_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .from_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .group_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.having_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .window_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .values_lists
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .sort_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.limit_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.limit_count.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .locking_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::WithClause(
                n.with_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::SelectStmt(
                n.larg.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::SelectStmt(
                n.rarg.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::ReturnStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.returnval.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::PlassignStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::SelectStmt(
                n.val.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterTableStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .cmds
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterTableCmd(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.newowner.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.def.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::AlterDomainStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.def.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::SetOperationStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.larg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.rarg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .col_types
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .col_typmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .col_collations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .group_clauses
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::GrantStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .objects
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .privileges
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .grantees
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.grantor.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::GrantRoleStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .granted_roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .grantee_roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.grantor.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterDefaultPrivilegesStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::GrantStmt(
                n.action.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::ClosePortalStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::ClusterStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CopyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .attlist
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CreateStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .table_elts
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .inh_relations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::PartitionBoundSpec(
                n.partbound.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::PartitionSpec(
                n.partspec.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.of_typename.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DefineStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .definition
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .objects
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::TruncateStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CommentStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::FetchStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::IndexStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .index_params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .index_including_params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .exclude_op_names
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateFunctionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .parameters
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.return_type.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.sql_body.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::AlterFunctionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.func.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .actions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DoStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RenameStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::RuleStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .actions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::NotifyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::ListenStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::UnlistenStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::TransactionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ViewStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.view.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .aliases
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::LoadStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::CreateDomainStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .domainname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::CollateClause(
                n.coll_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreatedbStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropdbStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::VacuumStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .rels
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ExplainStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateTableAsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::IntoClause(
                n.into.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreateSeqStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.sequence.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterSeqStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.sequence.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::VariableSetStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::VariableShowStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::DiscardStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::CreateTrigStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.when_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .transition_rels
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.constrrel.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreatePlangStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .plhandler
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .plinline
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .plvalidator
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateRoleStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterRoleStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.role.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropRoleStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::LockStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ConstraintsSetStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ReindexStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CheckPointStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::CreateSchemaStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.authrole.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .schema_elts
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterDatabaseStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterDatabaseRefreshCollStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::AlterDatabaseSetStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::VariableSetStmt(
                n.setstmt.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterRoleSetStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.role.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::VariableSetStmt(
                n.setstmt.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreateConversionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .conversion_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .func_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateCastStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.sourcetype.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.targettype.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.func.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreateOpClassStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .opclassname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.datatype.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateOpFamilyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterOpFamilyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::PrepareStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .argtypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::ExecuteStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DeallocateStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::DeclareCursorStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.query.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CreateTableSpaceStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.owner.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropTableSpaceStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::AlterObjectDependsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::String(
                n.extname.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterObjectSchemaStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::AlterOwnerStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.newowner.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterOperatorStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.opername.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterTypeStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropOwnedStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ReassignOwnedStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.newrole.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CompositeTypeStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.typevar.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .coldeflist
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateEnumStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .vals
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateRangeStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterEnumStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterTsdictionaryStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .dictname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterTsconfigurationStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .cfgname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .tokentype
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .dicts
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateFdwStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .func_options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterFdwStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .func_options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateForeignServerStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterForeignServerStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateUserMappingStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.user.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterUserMappingStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.user.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropUserMappingStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RoleSpec(
                n.user.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AlterTableSpaceOptionsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterTableMoveAllStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::SecLabelStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CreateForeignTableStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::CreateStmt(
                n.base_stmt.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ImportForeignSchemaStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .table_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateExtensionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterExtensionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterExtensionContentsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.object.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CreateEventTrigStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .whenclause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterEventTrigStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::RefreshMatViewStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::ReplicaIdentityStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::AlterSystemStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::VariableSetStmt(
                n.setstmt.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreatePolicyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.table.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.qual.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.with_check.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::AlterPolicyStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.table.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.qual.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.with_check.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CreateTransformStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.fromsql.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.tosql.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CreateAmStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .handler_name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreatePublicationStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .pubobjects
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterPublicationStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .pubobjects
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateSubscriptionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .publication
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterSubscriptionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .publication
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DropSubscriptionStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::CreateStatsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .stat_types
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .exprs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterCollationStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .collname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CallStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::FuncCall(
                n.funccall.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::FuncExpr(
                n.funcexpr.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .outargs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AlterStatsStmt(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .name
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.lexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.rexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::ColumnRef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .fields
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ParamRef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::FuncCall(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .agg_order
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.agg_filter.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::WindowDef(
                n.over.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::AStar(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::AIndices(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.lidx.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.uidx.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::AIndirection(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AArrayExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .elements
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ResTarget(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.val.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::MultiAssignRef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.source.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::TypeCast(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::CollateClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .collname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::SortBy(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.node.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .use_op
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::WindowDef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .partition_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .order_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.start_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.end_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::RangeSubselect(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.subquery.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::RangeFunction(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .functions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .coldeflist
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RangeTableSample(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.relation.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .method
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.repeatable.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::RangeTableFunc(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.docexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.rowexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .namespaces
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::RangeTableFuncCol(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.colexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.coldefexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::TypeName(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .names
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .typmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .array_bounds
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ColumnDef(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.raw_default.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.cooked_default.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.identity_sequence.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::CollateClause(
                n.coll_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .fdwoptions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::IndexElem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .collation
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .opclass
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .opclassopts
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::StatsElem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::Constraint(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.raw_expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .keys
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .including
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .exclusions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.pktable.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .fk_attrs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .pk_attrs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .fk_del_set_cols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .old_conpfeqop
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::DefElem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.arg.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::RangeTblEntry(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TableSampleClause(
                n.tablesample.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::Query(
                n.subquery.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .joinaliasvars
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .joinleftcols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .joinrightcols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.join_using_alias.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .functions
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::TableFunc(
                n.tablefunc.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .values_lists
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .coltypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .coltypmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .colcollations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.alias.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::Alias(
                n.eref.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .security_quals
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RangeTblFunction(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.funcexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .funccolnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .funccoltypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .funccoltypmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .funccolcollations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::TableSampleClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.repeatable.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::WithCheckOption(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.qual.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::SortGroupClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::GroupingSet(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .content
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::WindowClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .partition_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .order_clause
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.start_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.end_offset.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .run_condition
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::ObjectWithArgs(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .objname
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .objargs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .objfuncargs
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AccessPriv(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CreateOpClassItem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::ObjectWithArgs(
                n.name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .order_family
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .class_args
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.storedtype.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::TableLikeClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::FunctionParameter(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.arg_type.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.defexpr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::LockingClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .locked_rels
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RowMarkClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::XmlSerialize(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::TypeName(
                n.type_name.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::WithClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .ctes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::InferClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .index_elems
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::OnConflictClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::InferClause(
                n.infer.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::CtesearchClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .search_col_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::CtecycleClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .cycle_col_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.cycle_mark_value
                    .as_ref()
                    .unwrap()
                    .to_owned()
                    .node
                    .unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(
                n.cycle_mark_default
                    .as_ref()
                    .unwrap()
                    .to_owned()
                    .node
                    .unwrap(),
            ));
            nodes
        }
        NodeEnum::CommonTableExpr(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .aliascolnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(&mut get_nested_nodes(
                n.ctequery.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(&mut get_nested_nodes(NodeEnum::CtesearchClause(
                n.search_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::CtecycleClause(
                n.cycle_clause.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .ctecolnames
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .ctecoltypes
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .ctecoltypmods
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .ctecolcollations
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::MergeWhenClause(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.condition.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .values
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::RoleSpec(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::TriggerTransition(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::PartitionElem(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.expr.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .collation
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .opclass
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::PartitionSpec(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .part_params
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::PartitionBoundSpec(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .listdatums
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .lowerdatums
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes.append(
                &mut n
                    .upperdatums
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::PartitionRangeDatum(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(
                n.value.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes
        }
        NodeEnum::PartitionCmd(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.name.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(NodeEnum::PartitionBoundSpec(
                n.bound.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::VacuumRelation(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(
                &mut n
                    .va_cols
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::PublicationObjSpec(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::PublicationTable(
                n.pubtable.as_ref().unwrap().to_owned(),
            )));
            nodes
        }
        NodeEnum::PublicationTable(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(&mut get_nested_nodes(NodeEnum::RangeVar(
                n.relation.as_ref().unwrap().to_owned(),
            )));
            nodes.append(&mut get_nested_nodes(
                n.where_clause.as_ref().unwrap().to_owned().node.unwrap(),
            ));
            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::InlineCodeBlock(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::CallContext(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::Integer(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::Float(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::Boolean(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::String(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::BitString(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
        NodeEnum::List(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::IntList(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::OidList(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_nested_nodes(x.node.as_ref().unwrap().to_owned()))
                    .collect(),
            );
            nodes
        }
        NodeEnum::AConst(n) => {
            let mut nodes: Vec<NodeEnum> = vec![node.clone()];
            nodes
        }
    }
}
