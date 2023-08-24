//! Utilities for working with pg_query.rs
//! This file is generated from the libg_query proto
use crate::pg_query_utils_manual::derive_location;
use pg_query::NodeEnum;
use std::collections::VecDeque;

pub fn get_location(node: &NodeEnum) -> Option<i32> {
    match node {
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
        NodeEnum::AExpr(n) => get_location(&n.lexpr.as_ref().unwrap().node.as_ref().unwrap()),
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

#[derive(Debug, Clone)]
pub struct NestedNode {
    pub node: NodeEnum,
    pub depth: i32,
    pub location: i32,
    pub path: String,
}

/// Returns all children of the node, recursively
pub fn get_children(node: &NodeEnum, text: String, current_depth: i32) -> Vec<NestedNode> {
    let mut nodes: Vec<NestedNode> = vec![];
    // Node, depth, path
    let mut stack: VecDeque<(NodeEnum, i32, String)> =
        VecDeque::from(vec![(node.to_owned(), current_depth, "0".to_string())]); // Node, depth, path
    let mut location_stack: VecDeque<(NodeEnum, i32, String)> = VecDeque::new();
    while !stack.is_empty() || !location_stack.is_empty() {
        if !stack.is_empty() {
            let (node, depth, path) = stack.pop_front().unwrap();
            let current_depth = depth + 1;
            let mut child_ctr: i32 = 0;
            let mut handle_child = |c: NodeEnum| {
                let location = get_location(&c);
                let path = path.clone() + "." + child_ctr.to_string().as_str();
                child_ctr = child_ctr + 1;
                stack.push_back((c.to_owned(), current_depth, path.clone()));
                if location.is_some() {
                    nodes.push(NestedNode {
                        node: c,
                        depth: current_depth,
                        location: location.unwrap(),
                        path: path.clone(),
                    });
                } else {
                    location_stack.push_back((c, current_depth, path));
                }
            };
            match &node {
                NodeEnum::Alias(n) => {
                    n.colnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RangeVar(n) => {
                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }
                }
                NodeEnum::TableFunc(n) => {
                    n.ns_uris
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.ns_names
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.docexpr.is_some() {
                        handle_child(n.docexpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.rowexpr.is_some() {
                        handle_child(n.rowexpr.to_owned().unwrap().node.unwrap());
                    }

                    n.colnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.coltypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.coltypmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.colcollations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.colexprs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.coldefexprs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::Var(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::Param(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::Aggref(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.aggargtypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.aggdirectargs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.aggorder
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.aggdistinct
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.aggfilter.is_some() {
                        handle_child(n.aggfilter.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::GroupingFunc(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.refs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.cols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::WindowFunc(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.aggfilter.is_some() {
                        handle_child(n.aggfilter.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::SubscriptingRef(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.refupperindexpr
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.reflowerindexpr
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.refexpr.is_some() {
                        handle_child(n.refexpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.refassgnexpr.is_some() {
                        handle_child(n.refassgnexpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::FuncExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::NamedArgExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::OpExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DistinctExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::NullIfExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ScalarArrayOpExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::BoolExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::SubLink(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.testexpr.is_some() {
                        handle_child(n.testexpr.to_owned().unwrap().node.unwrap());
                    }

                    n.oper_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.subselect.is_some() {
                        handle_child(n.subselect.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::SubPlan(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.testexpr.is_some() {
                        handle_child(n.testexpr.to_owned().unwrap().node.unwrap());
                    }

                    n.param_ids
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.set_param
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.par_param
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlternativeSubPlan(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.subplans
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::FieldSelect(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::FieldStore(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    n.newvals
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.fieldnums
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RelabelType(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CoerceViaIo(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::ArrayCoerceExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    if n.elemexpr.is_some() {
                        handle_child(n.elemexpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::ConvertRowtypeExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CollateExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CaseExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.defresult.is_some() {
                        handle_child(n.defresult.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CaseWhen(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }

                    if n.result.is_some() {
                        handle_child(n.result.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CaseTestExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::ArrayExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.elements
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RowExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.colnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RowCompareExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.opnos
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.opfamilies
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.inputcollids
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.largs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.rargs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CoalesceExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::MinMaxExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::SqlvalueFunction(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::XmlExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    n.named_args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.arg_names
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::NullTest(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::BooleanTest(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CoerceToDomain(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CoerceToDomainValue(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::SetToDefault(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CurrentOfExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::NextValueExpr(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::InferenceElem(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::TargetEntry(n) => {
                    if n.xpr.is_some() {
                        handle_child(n.xpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::RangeTblRef(n) => (),
                NodeEnum::JoinExpr(n) => {
                    if n.larg.is_some() {
                        handle_child(n.larg.to_owned().unwrap().node.unwrap());
                    }

                    if n.rarg.is_some() {
                        handle_child(n.rarg.to_owned().unwrap().node.unwrap());
                    }

                    n.using_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.join_using_alias.is_some() {
                        handle_child(NodeEnum::Alias(n.join_using_alias.to_owned().unwrap()));
                    }

                    if n.quals.is_some() {
                        handle_child(n.quals.to_owned().unwrap().node.unwrap());
                    }

                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }
                }
                NodeEnum::FromExpr(n) => {
                    n.fromlist
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.quals.is_some() {
                        handle_child(n.quals.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::OnConflictExpr(n) => {
                    n.arbiter_elems
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.arbiter_where.is_some() {
                        handle_child(n.arbiter_where.to_owned().unwrap().node.unwrap());
                    }

                    n.on_conflict_set
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.on_conflict_where.is_some() {
                        handle_child(n.on_conflict_where.to_owned().unwrap().node.unwrap());
                    }

                    n.excl_rel_tlist
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::IntoClause(n) => {
                    if n.rel.is_some() {
                        handle_child(NodeEnum::RangeVar(n.rel.to_owned().unwrap()));
                    }

                    n.col_names
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.view_query.is_some() {
                        handle_child(n.view_query.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::MergeAction(n) => {
                    if n.qual.is_some() {
                        handle_child(n.qual.to_owned().unwrap().node.unwrap());
                    }

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.update_colnos
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RawStmt(n) => {
                    if n.stmt.is_some() {
                        handle_child(n.stmt.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::Query(n) => {
                    if n.utility_stmt.is_some() {
                        handle_child(n.utility_stmt.to_owned().unwrap().node.unwrap());
                    }

                    n.cte_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.rtable
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.jointree.is_some() {
                        handle_child(NodeEnum::FromExpr(n.jointree.to_owned().unwrap()));
                    }

                    n.merge_action_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.on_conflict.is_some() {
                        handle_child(NodeEnum::OnConflictExpr(n.on_conflict.to_owned().unwrap()));
                    }

                    n.returning_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.group_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.grouping_sets
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.having_qual.is_some() {
                        handle_child(n.having_qual.to_owned().unwrap().node.unwrap());
                    }

                    n.window_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.distinct_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.sort_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.limit_offset.is_some() {
                        handle_child(n.limit_offset.to_owned().unwrap().node.unwrap());
                    }

                    if n.limit_count.is_some() {
                        handle_child(n.limit_count.to_owned().unwrap().node.unwrap());
                    }

                    n.row_marks
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.set_operations.is_some() {
                        handle_child(n.set_operations.to_owned().unwrap().node.unwrap());
                    }

                    n.constraint_deps
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.with_check_options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::InsertStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.cols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.select_stmt.is_some() {
                        handle_child(n.select_stmt.to_owned().unwrap().node.unwrap());
                    }

                    if n.on_conflict_clause.is_some() {
                        handle_child(NodeEnum::OnConflictClause(
                            n.on_conflict_clause.to_owned().unwrap(),
                        ));
                    }

                    n.returning_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.with_clause.is_some() {
                        handle_child(NodeEnum::WithClause(n.with_clause.to_owned().unwrap()));
                    }
                }
                NodeEnum::DeleteStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.using_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.returning_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.with_clause.is_some() {
                        handle_child(NodeEnum::WithClause(n.with_clause.to_owned().unwrap()));
                    }
                }
                NodeEnum::UpdateStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.from_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.returning_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.with_clause.is_some() {
                        handle_child(NodeEnum::WithClause(n.with_clause.to_owned().unwrap()));
                    }
                }
                NodeEnum::MergeStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.source_relation.is_some() {
                        handle_child(n.source_relation.to_owned().unwrap().node.unwrap());
                    }

                    if n.join_condition.is_some() {
                        handle_child(n.join_condition.to_owned().unwrap().node.unwrap());
                    }

                    n.merge_when_clauses
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.with_clause.is_some() {
                        handle_child(NodeEnum::WithClause(n.with_clause.to_owned().unwrap()));
                    }
                }
                NodeEnum::SelectStmt(n) => {
                    n.distinct_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.into_clause.is_some() {
                        handle_child(NodeEnum::IntoClause(n.into_clause.to_owned().unwrap()));
                    }

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.from_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.group_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.having_clause.is_some() {
                        handle_child(n.having_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.window_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.values_lists
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.sort_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.limit_offset.is_some() {
                        handle_child(n.limit_offset.to_owned().unwrap().node.unwrap());
                    }

                    if n.limit_count.is_some() {
                        handle_child(n.limit_count.to_owned().unwrap().node.unwrap());
                    }

                    n.locking_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.with_clause.is_some() {
                        handle_child(NodeEnum::WithClause(n.with_clause.to_owned().unwrap()));
                    }

                    if n.larg.is_some() {
                        handle_child(NodeEnum::SelectStmt(n.larg.to_owned().unwrap()));
                    }

                    if n.rarg.is_some() {
                        handle_child(NodeEnum::SelectStmt(n.rarg.to_owned().unwrap()));
                    }
                }
                NodeEnum::ReturnStmt(n) => {
                    if n.returnval.is_some() {
                        handle_child(n.returnval.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::PlassignStmt(n) => {
                    n.indirection
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.val.is_some() {
                        handle_child(NodeEnum::SelectStmt(n.val.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterTableStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.cmds
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterTableCmd(n) => {
                    if n.newowner.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.newowner.to_owned().unwrap()));
                    }

                    if n.def.is_some() {
                        handle_child(n.def.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::AlterDomainStmt(n) => {
                    n.type_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.def.is_some() {
                        handle_child(n.def.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::SetOperationStmt(n) => {
                    if n.larg.is_some() {
                        handle_child(n.larg.to_owned().unwrap().node.unwrap());
                    }

                    if n.rarg.is_some() {
                        handle_child(n.rarg.to_owned().unwrap().node.unwrap());
                    }

                    n.col_types
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.col_typmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.col_collations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.group_clauses
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::GrantStmt(n) => {
                    n.objects
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.privileges
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.grantees
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.grantor.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.grantor.to_owned().unwrap()));
                    }
                }
                NodeEnum::GrantRoleStmt(n) => {
                    n.granted_roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.grantee_roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.grantor.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.grantor.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterDefaultPrivilegesStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.action.is_some() {
                        handle_child(NodeEnum::GrantStmt(n.action.to_owned().unwrap()));
                    }
                }
                NodeEnum::ClosePortalStmt(n) => (),
                NodeEnum::ClusterStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CopyStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }

                    n.attlist
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CreateStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.table_elts
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.inh_relations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.partbound.is_some() {
                        handle_child(NodeEnum::PartitionBoundSpec(
                            n.partbound.to_owned().unwrap(),
                        ));
                    }

                    if n.partspec.is_some() {
                        handle_child(NodeEnum::PartitionSpec(n.partspec.to_owned().unwrap()));
                    }

                    if n.of_typename.is_some() {
                        handle_child(NodeEnum::TypeName(n.of_typename.to_owned().unwrap()));
                    }

                    n.constraints
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DefineStmt(n) => {
                    n.defnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.definition
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropStmt(n) => {
                    n.objects
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::TruncateStmt(n) => {
                    n.relations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CommentStmt(n) => {
                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::FetchStmt(n) => (),
                NodeEnum::IndexStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.index_params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.index_including_params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.exclude_op_names
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateFunctionStmt(n) => {
                    n.funcname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.parameters
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.return_type.is_some() {
                        handle_child(NodeEnum::TypeName(n.return_type.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.sql_body.is_some() {
                        handle_child(n.sql_body.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::AlterFunctionStmt(n) => {
                    if n.func.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.func.to_owned().unwrap()));
                    }

                    n.actions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DoStmt(n) => {
                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RenameStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::RuleStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.actions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::NotifyStmt(n) => (),
                NodeEnum::ListenStmt(n) => (),
                NodeEnum::UnlistenStmt(n) => (),
                NodeEnum::TransactionStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ViewStmt(n) => {
                    if n.view.is_some() {
                        handle_child(NodeEnum::RangeVar(n.view.to_owned().unwrap()));
                    }

                    n.aliases
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::LoadStmt(n) => (),
                NodeEnum::CreateDomainStmt(n) => {
                    n.domainname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }

                    if n.coll_clause.is_some() {
                        handle_child(NodeEnum::CollateClause(n.coll_clause.to_owned().unwrap()));
                    }

                    n.constraints
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreatedbStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropdbStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::VacuumStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.rels
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ExplainStmt(n) => {
                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateTableAsStmt(n) => {
                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }

                    if n.into.is_some() {
                        handle_child(NodeEnum::IntoClause(n.into.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreateSeqStmt(n) => {
                    if n.sequence.is_some() {
                        handle_child(NodeEnum::RangeVar(n.sequence.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterSeqStmt(n) => {
                    if n.sequence.is_some() {
                        handle_child(NodeEnum::RangeVar(n.sequence.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::VariableSetStmt(n) => {
                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::VariableShowStmt(n) => (),
                NodeEnum::DiscardStmt(n) => (),
                NodeEnum::CreateTrigStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.funcname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.columns
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.when_clause.is_some() {
                        handle_child(n.when_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.transition_rels
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.constrrel.is_some() {
                        handle_child(NodeEnum::RangeVar(n.constrrel.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreatePlangStmt(n) => {
                    n.plhandler
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.plinline
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.plvalidator
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateRoleStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterRoleStmt(n) => {
                    if n.role.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.role.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropRoleStmt(n) => {
                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::LockStmt(n) => {
                    n.relations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ConstraintsSetStmt(n) => {
                    n.constraints
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ReindexStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CheckPointStmt(n) => (),
                NodeEnum::CreateSchemaStmt(n) => {
                    if n.authrole.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.authrole.to_owned().unwrap()));
                    }

                    n.schema_elts
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterDatabaseStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterDatabaseRefreshCollStmt(n) => (),
                NodeEnum::AlterDatabaseSetStmt(n) => {
                    if n.setstmt.is_some() {
                        handle_child(NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterRoleSetStmt(n) => {
                    if n.role.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.role.to_owned().unwrap()));
                    }

                    if n.setstmt.is_some() {
                        handle_child(NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreateConversionStmt(n) => {
                    n.conversion_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.func_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateCastStmt(n) => {
                    if n.sourcetype.is_some() {
                        handle_child(NodeEnum::TypeName(n.sourcetype.to_owned().unwrap()));
                    }

                    if n.targettype.is_some() {
                        handle_child(NodeEnum::TypeName(n.targettype.to_owned().unwrap()));
                    }

                    if n.func.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.func.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreateOpClassStmt(n) => {
                    n.opclassname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.opfamilyname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.datatype.is_some() {
                        handle_child(NodeEnum::TypeName(n.datatype.to_owned().unwrap()));
                    }

                    n.items
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateOpFamilyStmt(n) => {
                    n.opfamilyname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterOpFamilyStmt(n) => {
                    n.opfamilyname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.items
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::PrepareStmt(n) => {
                    n.argtypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::ExecuteStmt(n) => {
                    n.params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DeallocateStmt(n) => (),
                NodeEnum::DeclareCursorStmt(n) => {
                    if n.query.is_some() {
                        handle_child(n.query.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CreateTableSpaceStmt(n) => {
                    if n.owner.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.owner.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropTableSpaceStmt(n) => (),
                NodeEnum::AlterObjectDependsStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }

                    if n.extname.is_some() {
                        handle_child(NodeEnum::String(n.extname.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterObjectSchemaStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::AlterOwnerStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }

                    if n.newowner.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.newowner.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterOperatorStmt(n) => {
                    if n.opername.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.opername.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterTypeStmt(n) => {
                    n.type_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropOwnedStmt(n) => {
                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ReassignOwnedStmt(n) => {
                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.newrole.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.newrole.to_owned().unwrap()));
                    }
                }
                NodeEnum::CompositeTypeStmt(n) => {
                    if n.typevar.is_some() {
                        handle_child(NodeEnum::RangeVar(n.typevar.to_owned().unwrap()));
                    }

                    n.coldeflist
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateEnumStmt(n) => {
                    n.type_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.vals
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateRangeStmt(n) => {
                    n.type_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterEnumStmt(n) => {
                    n.type_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterTsdictionaryStmt(n) => {
                    n.dictname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterTsconfigurationStmt(n) => {
                    n.cfgname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.tokentype
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.dicts
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateFdwStmt(n) => {
                    n.func_options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterFdwStmt(n) => {
                    n.func_options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateForeignServerStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterForeignServerStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateUserMappingStmt(n) => {
                    if n.user.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.user.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterUserMappingStmt(n) => {
                    if n.user.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.user.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropUserMappingStmt(n) => {
                    if n.user.is_some() {
                        handle_child(NodeEnum::RoleSpec(n.user.to_owned().unwrap()));
                    }
                }
                NodeEnum::AlterTableSpaceOptionsStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterTableMoveAllStmt(n) => {
                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::SecLabelStmt(n) => {
                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CreateForeignTableStmt(n) => {
                    if n.base_stmt.is_some() {
                        handle_child(NodeEnum::CreateStmt(n.base_stmt.to_owned().unwrap()));
                    }

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ImportForeignSchemaStmt(n) => {
                    n.table_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateExtensionStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterExtensionStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterExtensionContentsStmt(n) => {
                    if n.object.is_some() {
                        handle_child(n.object.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CreateEventTrigStmt(n) => {
                    n.whenclause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.funcname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterEventTrigStmt(n) => (),
                NodeEnum::RefreshMatViewStmt(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }
                }
                NodeEnum::ReplicaIdentityStmt(n) => (),
                NodeEnum::AlterSystemStmt(n) => {
                    if n.setstmt.is_some() {
                        handle_child(NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreatePolicyStmt(n) => {
                    if n.table.is_some() {
                        handle_child(NodeEnum::RangeVar(n.table.to_owned().unwrap()));
                    }

                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.qual.is_some() {
                        handle_child(n.qual.to_owned().unwrap().node.unwrap());
                    }

                    if n.with_check.is_some() {
                        handle_child(n.with_check.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::AlterPolicyStmt(n) => {
                    if n.table.is_some() {
                        handle_child(NodeEnum::RangeVar(n.table.to_owned().unwrap()));
                    }

                    n.roles
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.qual.is_some() {
                        handle_child(n.qual.to_owned().unwrap().node.unwrap());
                    }

                    if n.with_check.is_some() {
                        handle_child(n.with_check.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CreateTransformStmt(n) => {
                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }

                    if n.fromsql.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.fromsql.to_owned().unwrap()));
                    }

                    if n.tosql.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.tosql.to_owned().unwrap()));
                    }
                }
                NodeEnum::CreateAmStmt(n) => {
                    n.handler_name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreatePublicationStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.pubobjects
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterPublicationStmt(n) => {
                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.pubobjects
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateSubscriptionStmt(n) => {
                    n.publication
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterSubscriptionStmt(n) => {
                    n.publication
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DropSubscriptionStmt(n) => (),
                NodeEnum::CreateStatsStmt(n) => {
                    n.defnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.stat_types
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.exprs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.relations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterCollationStmt(n) => {
                    n.collname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CallStmt(n) => {
                    if n.funccall.is_some() {
                        handle_child(NodeEnum::FuncCall(n.funccall.to_owned().unwrap()));
                    }

                    if n.funcexpr.is_some() {
                        handle_child(NodeEnum::FuncExpr(n.funcexpr.to_owned().unwrap()));
                    }

                    n.outargs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AlterStatsStmt(n) => {
                    n.defnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AExpr(n) => {
                    n.name
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.lexpr.is_some() {
                        handle_child(n.lexpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.rexpr.is_some() {
                        handle_child(n.rexpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::ColumnRef(n) => {
                    n.fields
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ParamRef(n) => (),
                NodeEnum::FuncCall(n) => {
                    n.funcname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.agg_order
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.agg_filter.is_some() {
                        handle_child(n.agg_filter.to_owned().unwrap().node.unwrap());
                    }

                    if n.over.is_some() {
                        handle_child(NodeEnum::WindowDef(n.over.to_owned().unwrap()));
                    }
                }
                NodeEnum::AStar(n) => (),
                NodeEnum::AIndices(n) => {
                    if n.lidx.is_some() {
                        handle_child(n.lidx.to_owned().unwrap().node.unwrap());
                    }

                    if n.uidx.is_some() {
                        handle_child(n.uidx.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::AIndirection(n) => {
                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    n.indirection
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AArrayExpr(n) => {
                    n.elements
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ResTarget(n) => {
                    n.indirection
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.val.is_some() {
                        handle_child(n.val.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::MultiAssignRef(n) => {
                    if n.source.is_some() {
                        handle_child(n.source.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::TypeCast(n) => {
                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }
                }
                NodeEnum::CollateClause(n) => {
                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }

                    n.collname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::SortBy(n) => {
                    if n.node.is_some() {
                        handle_child(n.node.to_owned().unwrap().node.unwrap());
                    }

                    n.use_op
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::WindowDef(n) => {
                    n.partition_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.order_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.start_offset.is_some() {
                        handle_child(n.start_offset.to_owned().unwrap().node.unwrap());
                    }

                    if n.end_offset.is_some() {
                        handle_child(n.end_offset.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::RangeSubselect(n) => {
                    if n.subquery.is_some() {
                        handle_child(n.subquery.to_owned().unwrap().node.unwrap());
                    }

                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }
                }
                NodeEnum::RangeFunction(n) => {
                    n.functions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }

                    n.coldeflist
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RangeTableSample(n) => {
                    if n.relation.is_some() {
                        handle_child(n.relation.to_owned().unwrap().node.unwrap());
                    }

                    n.method
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.repeatable.is_some() {
                        handle_child(n.repeatable.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::RangeTableFunc(n) => {
                    if n.docexpr.is_some() {
                        handle_child(n.docexpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.rowexpr.is_some() {
                        handle_child(n.rowexpr.to_owned().unwrap().node.unwrap());
                    }

                    n.namespaces
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.columns
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }
                }
                NodeEnum::RangeTableFuncCol(n) => {
                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }

                    if n.colexpr.is_some() {
                        handle_child(n.colexpr.to_owned().unwrap().node.unwrap());
                    }

                    if n.coldefexpr.is_some() {
                        handle_child(n.coldefexpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::TypeName(n) => {
                    n.names
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.typmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.array_bounds
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ColumnDef(n) => {
                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }

                    if n.raw_default.is_some() {
                        handle_child(n.raw_default.to_owned().unwrap().node.unwrap());
                    }

                    if n.cooked_default.is_some() {
                        handle_child(n.cooked_default.to_owned().unwrap().node.unwrap());
                    }

                    if n.identity_sequence.is_some() {
                        handle_child(NodeEnum::RangeVar(n.identity_sequence.to_owned().unwrap()));
                    }

                    if n.coll_clause.is_some() {
                        handle_child(NodeEnum::CollateClause(n.coll_clause.to_owned().unwrap()));
                    }

                    n.constraints
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.fdwoptions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::IndexElem(n) => {
                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }

                    n.collation
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.opclass
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.opclassopts
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::StatsElem(n) => {
                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::Constraint(n) => {
                    if n.raw_expr.is_some() {
                        handle_child(n.raw_expr.to_owned().unwrap().node.unwrap());
                    }

                    n.keys
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.including
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.exclusions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.options
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    if n.pktable.is_some() {
                        handle_child(NodeEnum::RangeVar(n.pktable.to_owned().unwrap()));
                    }

                    n.fk_attrs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.pk_attrs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.fk_del_set_cols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.old_conpfeqop
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::DefElem(n) => {
                    if n.arg.is_some() {
                        handle_child(n.arg.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::RangeTblEntry(n) => {
                    if n.tablesample.is_some() {
                        handle_child(NodeEnum::TableSampleClause(
                            n.tablesample.to_owned().unwrap(),
                        ));
                    }

                    if n.subquery.is_some() {
                        handle_child(NodeEnum::Query(n.subquery.to_owned().unwrap()));
                    }

                    n.joinaliasvars
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.joinleftcols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.joinrightcols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.join_using_alias.is_some() {
                        handle_child(NodeEnum::Alias(n.join_using_alias.to_owned().unwrap()));
                    }

                    n.functions
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.tablefunc.is_some() {
                        handle_child(NodeEnum::TableFunc(n.tablefunc.to_owned().unwrap()));
                    }

                    n.values_lists
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.coltypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.coltypmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.colcollations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }

                    if n.eref.is_some() {
                        handle_child(NodeEnum::Alias(n.eref.to_owned().unwrap()));
                    }

                    n.security_quals
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RangeTblFunction(n) => {
                    if n.funcexpr.is_some() {
                        handle_child(n.funcexpr.to_owned().unwrap().node.unwrap());
                    }

                    n.funccolnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.funccoltypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.funccoltypmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.funccolcollations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::TableSampleClause(n) => {
                    n.args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.repeatable.is_some() {
                        handle_child(n.repeatable.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::WithCheckOption(n) => {
                    if n.qual.is_some() {
                        handle_child(n.qual.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::SortGroupClause(n) => (),
                NodeEnum::GroupingSet(n) => {
                    n.content
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::WindowClause(n) => {
                    n.partition_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.order_clause
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.start_offset.is_some() {
                        handle_child(n.start_offset.to_owned().unwrap().node.unwrap());
                    }

                    if n.end_offset.is_some() {
                        handle_child(n.end_offset.to_owned().unwrap().node.unwrap());
                    }

                    n.run_condition
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::ObjectWithArgs(n) => {
                    n.objname
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.objargs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.objfuncargs
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AccessPriv(n) => {
                    n.cols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CreateOpClassItem(n) => {
                    if n.name.is_some() {
                        handle_child(NodeEnum::ObjectWithArgs(n.name.to_owned().unwrap()));
                    }

                    n.order_family
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.class_args
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.storedtype.is_some() {
                        handle_child(NodeEnum::TypeName(n.storedtype.to_owned().unwrap()));
                    }
                }
                NodeEnum::TableLikeClause(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }
                }
                NodeEnum::FunctionParameter(n) => {
                    if n.arg_type.is_some() {
                        handle_child(NodeEnum::TypeName(n.arg_type.to_owned().unwrap()));
                    }

                    if n.defexpr.is_some() {
                        handle_child(n.defexpr.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::LockingClause(n) => {
                    n.locked_rels
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RowMarkClause(n) => (),
                NodeEnum::XmlSerialize(n) => {
                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }

                    if n.type_name.is_some() {
                        handle_child(NodeEnum::TypeName(n.type_name.to_owned().unwrap()));
                    }
                }
                NodeEnum::WithClause(n) => {
                    n.ctes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::InferClause(n) => {
                    n.index_elems
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::OnConflictClause(n) => {
                    if n.infer.is_some() {
                        handle_child(NodeEnum::InferClause(n.infer.to_owned().unwrap()));
                    }

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CtesearchClause(n) => {
                    n.search_col_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::CtecycleClause(n) => {
                    n.cycle_col_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.cycle_mark_value.is_some() {
                        handle_child(n.cycle_mark_value.to_owned().unwrap().node.unwrap());
                    }

                    if n.cycle_mark_default.is_some() {
                        handle_child(n.cycle_mark_default.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::CommonTableExpr(n) => {
                    n.aliascolnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    if n.ctequery.is_some() {
                        handle_child(n.ctequery.to_owned().unwrap().node.unwrap());
                    }

                    if n.search_clause.is_some() {
                        handle_child(NodeEnum::CtesearchClause(
                            n.search_clause.to_owned().unwrap(),
                        ));
                    }

                    if n.cycle_clause.is_some() {
                        handle_child(NodeEnum::CtecycleClause(n.cycle_clause.to_owned().unwrap()));
                    }

                    n.ctecolnames
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.ctecoltypes
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.ctecoltypmods
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.ctecolcollations
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::MergeWhenClause(n) => {
                    if n.condition.is_some() {
                        handle_child(n.condition.to_owned().unwrap().node.unwrap());
                    }

                    n.target_list
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.values
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RoleSpec(n) => (),
                NodeEnum::TriggerTransition(n) => (),
                NodeEnum::PartitionElem(n) => {
                    if n.expr.is_some() {
                        handle_child(n.expr.to_owned().unwrap().node.unwrap());
                    }

                    n.collation
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.opclass
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::PartitionSpec(n) => {
                    n.part_params
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::PartitionBoundSpec(n) => {
                    n.listdatums
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.lowerdatums
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));

                    n.upperdatums
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::PartitionRangeDatum(n) => {
                    if n.value.is_some() {
                        handle_child(n.value.to_owned().unwrap().node.unwrap());
                    }
                }
                NodeEnum::PartitionCmd(n) => {
                    if n.name.is_some() {
                        handle_child(NodeEnum::RangeVar(n.name.to_owned().unwrap()));
                    }

                    if n.bound.is_some() {
                        handle_child(NodeEnum::PartitionBoundSpec(n.bound.to_owned().unwrap()));
                    }
                }
                NodeEnum::VacuumRelation(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    n.va_cols
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::PublicationObjSpec(n) => {
                    if n.pubtable.is_some() {
                        handle_child(NodeEnum::PublicationTable(n.pubtable.to_owned().unwrap()));
                    }
                }
                NodeEnum::PublicationTable(n) => {
                    if n.relation.is_some() {
                        handle_child(NodeEnum::RangeVar(n.relation.to_owned().unwrap()));
                    }

                    if n.where_clause.is_some() {
                        handle_child(n.where_clause.to_owned().unwrap().node.unwrap());
                    }

                    n.columns
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::InlineCodeBlock(n) => (),
                NodeEnum::CallContext(n) => (),
                NodeEnum::Integer(n) => (),
                NodeEnum::Float(n) => (),
                NodeEnum::Boolean(n) => (),
                NodeEnum::String(n) => (),
                NodeEnum::BitString(n) => (),
                NodeEnum::List(n) => {
                    n.items
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::IntList(n) => {
                    n.items
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::OidList(n) => {
                    n.items
                        .iter()
                        .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::AConst(n) => {
                    if n.val.is_some() {
                        handle_child(match n.val.to_owned().unwrap() {
                            pg_query::protobuf::a_const::Val::Ival(v) => NodeEnum::Integer(v),
                            pg_query::protobuf::a_const::Val::Fval(v) => NodeEnum::Float(v),
                            pg_query::protobuf::a_const::Val::Boolval(v) => NodeEnum::Boolean(v),
                            pg_query::protobuf::a_const::Val::Sval(v) => NodeEnum::String(v),
                            pg_query::protobuf::a_const::Val::Bsval(v) => NodeEnum::BitString(v),
                        });
                    }
                }
            };
        } else if !location_stack.is_empty() {
            let (node, depth, path) = location_stack.pop_front().unwrap();
            let parent_node = nodes.iter().find(|n| {
                let mut path_elements = path.split(".").collect::<Vec<&str>>();
                path_elements.pop();
                let parent_path = path_elements.join(".");
                n.path == parent_path
            });
            let parent_location = if parent_node.is_some() {
                parent_node.unwrap().location
            } else {
                0
            };
            let earliest_child_location = nodes
                .iter()
                .filter(|n| n.path.starts_with(path.as_str()))
                .min_by(|a, b| a.location.cmp(&b.location))
                .map(|n| n.location);
            let location = derive_location(
                &node,
                text.clone(),
                parent_location,
                earliest_child_location,
            );
            if location.is_some() {
                nodes.push(NestedNode {
                    node,
                    depth,
                    location: location.unwrap(),
                    path: path.clone(),
                });
            }
        }
    }
    nodes.sort_by_key(|n| n.location);
    nodes
}
