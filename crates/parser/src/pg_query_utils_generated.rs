//! Utilities for working with pg_query.rs
//! This file is generated from the libg_query proto
use pg_query::NodeEnum;

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

#[derive(Debug, Clone)]
pub struct NestedNode {
    node: NodeEnum,
    depth: i32,
}

/// Returns all children of the node, recursively
pub fn get_children(node: &NodeEnum, current_depth: i32) -> Vec<NestedNode> {
    let current_depth = current_depth + 1;
    match &node {
        NodeEnum::Alias(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RangeVar(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::TableFunc(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .ns_uris
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .ns_names
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let docexpr = n.docexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&docexpr, current_depth));
            nodes.push(NestedNode {
                node: docexpr,
                depth: current_depth,
            });

            let rowexpr = n.rowexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&rowexpr, current_depth));
            nodes.push(NestedNode {
                node: rowexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .coltypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .coltypmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .colcollations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .colexprs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .coldefexprs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::Var(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::Param(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::Aggref(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .aggargtypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .aggdirectargs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .aggorder
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .aggdistinct
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let aggfilter = n.aggfilter.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&aggfilter, current_depth));
            nodes.push(NestedNode {
                node: aggfilter,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::GroupingFunc(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .refs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::WindowFunc(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let aggfilter = n.aggfilter.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&aggfilter, current_depth));
            nodes.push(NestedNode {
                node: aggfilter,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SubscriptingRef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .refupperindexpr
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .reflowerindexpr
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let refexpr = n.refexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&refexpr, current_depth));
            nodes.push(NestedNode {
                node: refexpr,
                depth: current_depth,
            });

            let refassgnexpr = n.refassgnexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&refassgnexpr, current_depth));
            nodes.push(NestedNode {
                node: refassgnexpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::FuncExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::NamedArgExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::OpExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DistinctExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::NullIfExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ScalarArrayOpExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::BoolExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::SubLink(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let testexpr = n.testexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&testexpr, current_depth));
            nodes.push(NestedNode {
                node: testexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .oper_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let subselect = n.subselect.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&subselect, current_depth));
            nodes.push(NestedNode {
                node: subselect,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SubPlan(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let testexpr = n.testexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&testexpr, current_depth));
            nodes.push(NestedNode {
                node: testexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .param_ids
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .set_param
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .par_param
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlternativeSubPlan(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .subplans
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::FieldSelect(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::FieldStore(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .newvals
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .fieldnums
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RelabelType(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CoerceViaIo(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ArrayCoerceExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            let elemexpr = n.elemexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&elemexpr, current_depth));
            nodes.push(NestedNode {
                node: elemexpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ConvertRowtypeExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CollateExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CaseExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let defresult = n.defresult.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&defresult, current_depth));
            nodes.push(NestedNode {
                node: defresult,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CaseWhen(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            let result = n.result.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&result, current_depth));
            nodes.push(NestedNode {
                node: result,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CaseTestExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ArrayExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .elements
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RowExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .colnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RowCompareExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .opnos
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .opfamilies
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .inputcollids
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .largs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .rargs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CoalesceExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::MinMaxExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::SqlvalueFunction(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::XmlExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .named_args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .arg_names
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::NullTest(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::BooleanTest(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CoerceToDomain(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CoerceToDomainValue(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SetToDefault(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CurrentOfExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::NextValueExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::InferenceElem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::TargetEntry(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let xpr = n.xpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&xpr, current_depth));
            nodes.push(NestedNode {
                node: xpr,
                depth: current_depth,
            });

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeTblRef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::JoinExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let larg = n.larg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&larg, current_depth));
            nodes.push(NestedNode {
                node: larg,
                depth: current_depth,
            });

            let rarg = n.rarg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&rarg, current_depth));
            nodes.push(NestedNode {
                node: rarg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .using_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let join_using_alias = NodeEnum::Alias(n.join_using_alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&join_using_alias, current_depth));
            nodes.push(NestedNode {
                node: join_using_alias,
                depth: current_depth,
            });

            let quals = n.quals.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&quals, current_depth));
            nodes.push(NestedNode {
                node: quals,
                depth: current_depth,
            });

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::FromExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .fromlist
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let quals = n.quals.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&quals, current_depth));
            nodes.push(NestedNode {
                node: quals,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::OnConflictExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .arbiter_elems
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let arbiter_where = n.arbiter_where.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arbiter_where, current_depth));
            nodes.push(NestedNode {
                node: arbiter_where,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .on_conflict_set
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let on_conflict_where = n
                .on_conflict_where
                .as_ref()
                .unwrap()
                .to_owned()
                .node
                .unwrap();
            nodes.append(&mut get_children(&on_conflict_where, current_depth));
            nodes.push(NestedNode {
                node: on_conflict_where,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .excl_rel_tlist
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::IntoClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let rel = NodeEnum::RangeVar(n.rel.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&rel, current_depth));
            nodes.push(NestedNode {
                node: rel,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .col_names
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let view_query = n.view_query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&view_query, current_depth));
            nodes.push(NestedNode {
                node: view_query,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::MergeAction(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let qual = n.qual.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&qual, current_depth));
            nodes.push(NestedNode {
                node: qual,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .update_colnos
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RawStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let stmt = n.stmt.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&stmt, current_depth));
            nodes.push(NestedNode {
                node: stmt,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::Query(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let utility_stmt = n.utility_stmt.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&utility_stmt, current_depth));
            nodes.push(NestedNode {
                node: utility_stmt,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .cte_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .rtable
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let jointree = NodeEnum::FromExpr(n.jointree.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&jointree, current_depth));
            nodes.push(NestedNode {
                node: jointree,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .merge_action_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let on_conflict = NodeEnum::OnConflictExpr(n.on_conflict.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&on_conflict, current_depth));
            nodes.push(NestedNode {
                node: on_conflict,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .group_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .grouping_sets
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let having_qual = n.having_qual.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&having_qual, current_depth));
            nodes.push(NestedNode {
                node: having_qual,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .window_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .distinct_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .sort_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let limit_offset = n.limit_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&limit_offset, current_depth));
            nodes.push(NestedNode {
                node: limit_offset,
                depth: current_depth,
            });

            let limit_count = n.limit_count.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&limit_count, current_depth));
            nodes.push(NestedNode {
                node: limit_count,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .row_marks
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let set_operations = n.set_operations.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&set_operations, current_depth));
            nodes.push(NestedNode {
                node: set_operations,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .constraint_deps
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .with_check_options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::InsertStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let select_stmt = n.select_stmt.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&select_stmt, current_depth));
            nodes.push(NestedNode {
                node: select_stmt,
                depth: current_depth,
            });

            let on_conflict_clause =
                NodeEnum::OnConflictClause(n.on_conflict_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&on_conflict_clause, current_depth));
            nodes.push(NestedNode {
                node: on_conflict_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let with_clause = NodeEnum::WithClause(n.with_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&with_clause, current_depth));
            nodes.push(NestedNode {
                node: with_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::DeleteStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .using_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let with_clause = NodeEnum::WithClause(n.with_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&with_clause, current_depth));
            nodes.push(NestedNode {
                node: with_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::UpdateStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .from_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .returning_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let with_clause = NodeEnum::WithClause(n.with_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&with_clause, current_depth));
            nodes.push(NestedNode {
                node: with_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::MergeStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let source_relation = n.source_relation.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&source_relation, current_depth));
            nodes.push(NestedNode {
                node: source_relation,
                depth: current_depth,
            });

            let join_condition = n.join_condition.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&join_condition, current_depth));
            nodes.push(NestedNode {
                node: join_condition,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .merge_when_clauses
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let with_clause = NodeEnum::WithClause(n.with_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&with_clause, current_depth));
            nodes.push(NestedNode {
                node: with_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SelectStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .distinct_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let into_clause = NodeEnum::IntoClause(n.into_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&into_clause, current_depth));
            nodes.push(NestedNode {
                node: into_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .from_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .group_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let having_clause = n.having_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&having_clause, current_depth));
            nodes.push(NestedNode {
                node: having_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .window_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .values_lists
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .sort_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let limit_offset = n.limit_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&limit_offset, current_depth));
            nodes.push(NestedNode {
                node: limit_offset,
                depth: current_depth,
            });

            let limit_count = n.limit_count.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&limit_count, current_depth));
            nodes.push(NestedNode {
                node: limit_count,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .locking_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let with_clause = NodeEnum::WithClause(n.with_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&with_clause, current_depth));
            nodes.push(NestedNode {
                node: with_clause,
                depth: current_depth,
            });

            let larg = NodeEnum::SelectStmt(n.larg.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&larg, current_depth));
            nodes.push(NestedNode {
                node: larg,
                depth: current_depth,
            });

            let rarg = NodeEnum::SelectStmt(n.rarg.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&rarg, current_depth));
            nodes.push(NestedNode {
                node: rarg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ReturnStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let returnval = n.returnval.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&returnval, current_depth));
            nodes.push(NestedNode {
                node: returnval,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::PlassignStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let val = NodeEnum::SelectStmt(n.val.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&val, current_depth));
            nodes.push(NestedNode {
                node: val,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterTableStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .cmds
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterTableCmd(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let newowner = NodeEnum::RoleSpec(n.newowner.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&newowner, current_depth));
            nodes.push(NestedNode {
                node: newowner,
                depth: current_depth,
            });

            let def = n.def.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&def, current_depth));
            nodes.push(NestedNode {
                node: def,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterDomainStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let def = n.def.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&def, current_depth));
            nodes.push(NestedNode {
                node: def,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SetOperationStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let larg = n.larg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&larg, current_depth));
            nodes.push(NestedNode {
                node: larg,
                depth: current_depth,
            });

            let rarg = n.rarg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&rarg, current_depth));
            nodes.push(NestedNode {
                node: rarg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .col_types
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .col_typmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .col_collations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .group_clauses
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::GrantStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .objects
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .privileges
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .grantees
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let grantor = NodeEnum::RoleSpec(n.grantor.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&grantor, current_depth));
            nodes.push(NestedNode {
                node: grantor,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::GrantRoleStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .granted_roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .grantee_roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let grantor = NodeEnum::RoleSpec(n.grantor.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&grantor, current_depth));
            nodes.push(NestedNode {
                node: grantor,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterDefaultPrivilegesStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let action = NodeEnum::GrantStmt(n.action.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&action, current_depth));
            nodes.push(NestedNode {
                node: action,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ClosePortalStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::ClusterStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CopyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .attlist
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .table_elts
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .inh_relations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let partbound = NodeEnum::PartitionBoundSpec(n.partbound.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&partbound, current_depth));
            nodes.push(NestedNode {
                node: partbound,
                depth: current_depth,
            });

            let partspec = NodeEnum::PartitionSpec(n.partspec.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&partspec, current_depth));
            nodes.push(NestedNode {
                node: partspec,
                depth: current_depth,
            });

            let of_typename = NodeEnum::TypeName(n.of_typename.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&of_typename, current_depth));
            nodes.push(NestedNode {
                node: of_typename,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DefineStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .definition
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .objects
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::TruncateStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CommentStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::FetchStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::IndexStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .index_params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .index_including_params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .exclude_op_names
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateFunctionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .parameters
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let return_type = NodeEnum::TypeName(n.return_type.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&return_type, current_depth));
            nodes.push(NestedNode {
                node: return_type,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let sql_body = n.sql_body.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&sql_body, current_depth));
            nodes.push(NestedNode {
                node: sql_body,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterFunctionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let func = NodeEnum::ObjectWithArgs(n.func.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&func, current_depth));
            nodes.push(NestedNode {
                node: func,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .actions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DoStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RenameStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RuleStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .actions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::NotifyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::ListenStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::UnlistenStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::TransactionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ViewStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let view = NodeEnum::RangeVar(n.view.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&view, current_depth));
            nodes.push(NestedNode {
                node: view,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .aliases
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::LoadStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::CreateDomainStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .domainname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            let coll_clause = NodeEnum::CollateClause(n.coll_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&coll_clause, current_depth));
            nodes.push(NestedNode {
                node: coll_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreatedbStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropdbStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::VacuumStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .rels
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ExplainStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateTableAsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            let into = NodeEnum::IntoClause(n.into.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&into, current_depth));
            nodes.push(NestedNode {
                node: into,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateSeqStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let sequence = NodeEnum::RangeVar(n.sequence.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&sequence, current_depth));
            nodes.push(NestedNode {
                node: sequence,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterSeqStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let sequence = NodeEnum::RangeVar(n.sequence.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&sequence, current_depth));
            nodes.push(NestedNode {
                node: sequence,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::VariableSetStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::VariableShowStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::DiscardStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::CreateTrigStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let when_clause = n.when_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&when_clause, current_depth));
            nodes.push(NestedNode {
                node: when_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .transition_rels
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let constrrel = NodeEnum::RangeVar(n.constrrel.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&constrrel, current_depth));
            nodes.push(NestedNode {
                node: constrrel,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreatePlangStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .plhandler
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .plinline
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .plvalidator
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateRoleStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterRoleStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let role = NodeEnum::RoleSpec(n.role.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&role, current_depth));
            nodes.push(NestedNode {
                node: role,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropRoleStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::LockStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ConstraintsSetStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ReindexStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CheckPointStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes
        }
        NodeEnum::CreateSchemaStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let authrole = NodeEnum::RoleSpec(n.authrole.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&authrole, current_depth));
            nodes.push(NestedNode {
                node: authrole,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .schema_elts
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterDatabaseStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterDatabaseRefreshCollStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::AlterDatabaseSetStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let setstmt = NodeEnum::VariableSetStmt(n.setstmt.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&setstmt, current_depth));
            nodes.push(NestedNode {
                node: setstmt,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterRoleSetStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let role = NodeEnum::RoleSpec(n.role.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&role, current_depth));
            nodes.push(NestedNode {
                node: role,
                depth: current_depth,
            });

            let setstmt = NodeEnum::VariableSetStmt(n.setstmt.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&setstmt, current_depth));
            nodes.push(NestedNode {
                node: setstmt,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateConversionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .conversion_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .func_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateCastStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let sourcetype = NodeEnum::TypeName(n.sourcetype.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&sourcetype, current_depth));
            nodes.push(NestedNode {
                node: sourcetype,
                depth: current_depth,
            });

            let targettype = NodeEnum::TypeName(n.targettype.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&targettype, current_depth));
            nodes.push(NestedNode {
                node: targettype,
                depth: current_depth,
            });

            let func = NodeEnum::ObjectWithArgs(n.func.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&func, current_depth));
            nodes.push(NestedNode {
                node: func,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateOpClassStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .opclassname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let datatype = NodeEnum::TypeName(n.datatype.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&datatype, current_depth));
            nodes.push(NestedNode {
                node: datatype,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateOpFamilyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterOpFamilyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .opfamilyname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::PrepareStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .argtypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ExecuteStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DeallocateStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::DeclareCursorStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let query = n.query.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&query, current_depth));
            nodes.push(NestedNode {
                node: query,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateTableSpaceStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let owner = NodeEnum::RoleSpec(n.owner.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&owner, current_depth));
            nodes.push(NestedNode {
                node: owner,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropTableSpaceStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::AlterObjectDependsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            let extname = NodeEnum::String(n.extname.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&extname, current_depth));
            nodes.push(NestedNode {
                node: extname,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterObjectSchemaStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterOwnerStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            let newowner = NodeEnum::RoleSpec(n.newowner.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&newowner, current_depth));
            nodes.push(NestedNode {
                node: newowner,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterOperatorStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let opername = NodeEnum::ObjectWithArgs(n.opername.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&opername, current_depth));
            nodes.push(NestedNode {
                node: opername,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterTypeStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropOwnedStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ReassignOwnedStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let newrole = NodeEnum::RoleSpec(n.newrole.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&newrole, current_depth));
            nodes.push(NestedNode {
                node: newrole,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CompositeTypeStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let typevar = NodeEnum::RangeVar(n.typevar.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&typevar, current_depth));
            nodes.push(NestedNode {
                node: typevar,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .coldeflist
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateEnumStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .vals
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateRangeStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterEnumStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .type_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterTsdictionaryStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .dictname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterTsconfigurationStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .cfgname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .tokentype
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .dicts
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateFdwStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .func_options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterFdwStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .func_options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateForeignServerStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterForeignServerStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateUserMappingStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let user = NodeEnum::RoleSpec(n.user.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&user, current_depth));
            nodes.push(NestedNode {
                node: user,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterUserMappingStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let user = NodeEnum::RoleSpec(n.user.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&user, current_depth));
            nodes.push(NestedNode {
                node: user,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropUserMappingStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let user = NodeEnum::RoleSpec(n.user.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&user, current_depth));
            nodes.push(NestedNode {
                node: user,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterTableSpaceOptionsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterTableMoveAllStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::SecLabelStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateForeignTableStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let base_stmt = NodeEnum::CreateStmt(n.base_stmt.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&base_stmt, current_depth));
            nodes.push(NestedNode {
                node: base_stmt,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ImportForeignSchemaStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .table_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateExtensionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterExtensionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterExtensionContentsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let object = n.object.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&object, current_depth));
            nodes.push(NestedNode {
                node: object,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateEventTrigStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .whenclause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterEventTrigStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::RefreshMatViewStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ReplicaIdentityStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::AlterSystemStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let setstmt = NodeEnum::VariableSetStmt(n.setstmt.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&setstmt, current_depth));
            nodes.push(NestedNode {
                node: setstmt,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreatePolicyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let table = NodeEnum::RangeVar(n.table.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&table, current_depth));
            nodes.push(NestedNode {
                node: table,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let qual = n.qual.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&qual, current_depth));
            nodes.push(NestedNode {
                node: qual,
                depth: current_depth,
            });

            let with_check = n.with_check.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&with_check, current_depth));
            nodes.push(NestedNode {
                node: with_check,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AlterPolicyStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let table = NodeEnum::RangeVar(n.table.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&table, current_depth));
            nodes.push(NestedNode {
                node: table,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .roles
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let qual = n.qual.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&qual, current_depth));
            nodes.push(NestedNode {
                node: qual,
                depth: current_depth,
            });

            let with_check = n.with_check.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&with_check, current_depth));
            nodes.push(NestedNode {
                node: with_check,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateTransformStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            let fromsql = NodeEnum::ObjectWithArgs(n.fromsql.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&fromsql, current_depth));
            nodes.push(NestedNode {
                node: fromsql,
                depth: current_depth,
            });

            let tosql = NodeEnum::ObjectWithArgs(n.tosql.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&tosql, current_depth));
            nodes.push(NestedNode {
                node: tosql,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CreateAmStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .handler_name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreatePublicationStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .pubobjects
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterPublicationStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .pubobjects
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateSubscriptionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .publication
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterSubscriptionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .publication
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DropSubscriptionStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::CreateStatsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .stat_types
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .exprs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .relations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterCollationStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .collname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CallStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let funccall = NodeEnum::FuncCall(n.funccall.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&funccall, current_depth));
            nodes.push(NestedNode {
                node: funccall,
                depth: current_depth,
            });

            let funcexpr = NodeEnum::FuncExpr(n.funcexpr.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&funcexpr, current_depth));
            nodes.push(NestedNode {
                node: funcexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .outargs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AlterStatsStmt(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .defnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .name
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let lexpr = n.lexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&lexpr, current_depth));
            nodes.push(NestedNode {
                node: lexpr,
                depth: current_depth,
            });

            let rexpr = n.rexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&rexpr, current_depth));
            nodes.push(NestedNode {
                node: rexpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::ColumnRef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .fields
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ParamRef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::FuncCall(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .funcname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .agg_order
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let agg_filter = n.agg_filter.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&agg_filter, current_depth));
            nodes.push(NestedNode {
                node: agg_filter,
                depth: current_depth,
            });

            let over = NodeEnum::WindowDef(n.over.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&over, current_depth));
            nodes.push(NestedNode {
                node: over,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AStar(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes
        }
        NodeEnum::AIndices(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let lidx = n.lidx.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&lidx, current_depth));
            nodes.push(NestedNode {
                node: lidx,
                depth: current_depth,
            });

            let uidx = n.uidx.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&uidx, current_depth));
            nodes.push(NestedNode {
                node: uidx,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::AIndirection(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AArrayExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .elements
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ResTarget(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .indirection
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let val = n.val.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&val, current_depth));
            nodes.push(NestedNode {
                node: val,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::MultiAssignRef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let source = n.source.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&source, current_depth));
            nodes.push(NestedNode {
                node: source,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::TypeCast(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CollateClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .collname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::SortBy(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let node = n.node.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&node, current_depth));
            nodes.push(NestedNode {
                node: node,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .use_op
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::WindowDef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .partition_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .order_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let start_offset = n.start_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&start_offset, current_depth));
            nodes.push(NestedNode {
                node: start_offset,
                depth: current_depth,
            });

            let end_offset = n.end_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&end_offset, current_depth));
            nodes.push(NestedNode {
                node: end_offset,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeSubselect(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let subquery = n.subquery.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&subquery, current_depth));
            nodes.push(NestedNode {
                node: subquery,
                depth: current_depth,
            });

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeFunction(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .functions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .coldeflist
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RangeTableSample(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = n.relation.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .method
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let repeatable = n.repeatable.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&repeatable, current_depth));
            nodes.push(NestedNode {
                node: repeatable,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeTableFunc(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let docexpr = n.docexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&docexpr, current_depth));
            nodes.push(NestedNode {
                node: docexpr,
                depth: current_depth,
            });

            let rowexpr = n.rowexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&rowexpr, current_depth));
            nodes.push(NestedNode {
                node: rowexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .namespaces
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeTableFuncCol(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            let colexpr = n.colexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&colexpr, current_depth));
            nodes.push(NestedNode {
                node: colexpr,
                depth: current_depth,
            });

            let coldefexpr = n.coldefexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&coldefexpr, current_depth));
            nodes.push(NestedNode {
                node: coldefexpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::TypeName(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .names
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .typmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .array_bounds
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ColumnDef(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            let raw_default = n.raw_default.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&raw_default, current_depth));
            nodes.push(NestedNode {
                node: raw_default,
                depth: current_depth,
            });

            let cooked_default = n.cooked_default.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&cooked_default, current_depth));
            nodes.push(NestedNode {
                node: cooked_default,
                depth: current_depth,
            });

            let identity_sequence =
                NodeEnum::RangeVar(n.identity_sequence.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&identity_sequence, current_depth));
            nodes.push(NestedNode {
                node: identity_sequence,
                depth: current_depth,
            });

            let coll_clause = NodeEnum::CollateClause(n.coll_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&coll_clause, current_depth));
            nodes.push(NestedNode {
                node: coll_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .constraints
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .fdwoptions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::IndexElem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .collation
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .opclass
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .opclassopts
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::StatsElem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::Constraint(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let raw_expr = n.raw_expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&raw_expr, current_depth));
            nodes.push(NestedNode {
                node: raw_expr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .keys
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .including
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .exclusions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .options
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            let pktable = NodeEnum::RangeVar(n.pktable.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&pktable, current_depth));
            nodes.push(NestedNode {
                node: pktable,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .fk_attrs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .pk_attrs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .fk_del_set_cols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .old_conpfeqop
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::DefElem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let arg = n.arg.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&arg, current_depth));
            nodes.push(NestedNode {
                node: arg,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::RangeTblEntry(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let tablesample =
                NodeEnum::TableSampleClause(n.tablesample.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&tablesample, current_depth));
            nodes.push(NestedNode {
                node: tablesample,
                depth: current_depth,
            });

            let subquery = NodeEnum::Query(n.subquery.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&subquery, current_depth));
            nodes.push(NestedNode {
                node: subquery,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .joinaliasvars
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .joinleftcols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .joinrightcols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let join_using_alias = NodeEnum::Alias(n.join_using_alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&join_using_alias, current_depth));
            nodes.push(NestedNode {
                node: join_using_alias,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .functions
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let tablefunc = NodeEnum::TableFunc(n.tablefunc.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&tablefunc, current_depth));
            nodes.push(NestedNode {
                node: tablefunc,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .values_lists
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .coltypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .coltypmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .colcollations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let alias = NodeEnum::Alias(n.alias.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&alias, current_depth));
            nodes.push(NestedNode {
                node: alias,
                depth: current_depth,
            });

            let eref = NodeEnum::Alias(n.eref.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&eref, current_depth));
            nodes.push(NestedNode {
                node: eref,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .security_quals
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RangeTblFunction(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let funcexpr = n.funcexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&funcexpr, current_depth));
            nodes.push(NestedNode {
                node: funcexpr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .funccolnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .funccoltypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .funccoltypmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .funccolcollations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::TableSampleClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let repeatable = n.repeatable.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&repeatable, current_depth));
            nodes.push(NestedNode {
                node: repeatable,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::WithCheckOption(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let qual = n.qual.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&qual, current_depth));
            nodes.push(NestedNode {
                node: qual,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::SortGroupClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::GroupingSet(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .content
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::WindowClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .partition_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .order_clause
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let start_offset = n.start_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&start_offset, current_depth));
            nodes.push(NestedNode {
                node: start_offset,
                depth: current_depth,
            });

            let end_offset = n.end_offset.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&end_offset, current_depth));
            nodes.push(NestedNode {
                node: end_offset,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .run_condition
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::ObjectWithArgs(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .objname
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .objargs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .objfuncargs
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AccessPriv(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .cols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CreateOpClassItem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let name = NodeEnum::ObjectWithArgs(n.name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&name, current_depth));
            nodes.push(NestedNode {
                node: name,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .order_family
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .class_args
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let storedtype = NodeEnum::TypeName(n.storedtype.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&storedtype, current_depth));
            nodes.push(NestedNode {
                node: storedtype,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::TableLikeClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::FunctionParameter(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let arg_type = NodeEnum::TypeName(n.arg_type.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&arg_type, current_depth));
            nodes.push(NestedNode {
                node: arg_type,
                depth: current_depth,
            });

            let defexpr = n.defexpr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&defexpr, current_depth));
            nodes.push(NestedNode {
                node: defexpr,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::LockingClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .locked_rels
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RowMarkClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::XmlSerialize(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            let type_name = NodeEnum::TypeName(n.type_name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&type_name, current_depth));
            nodes.push(NestedNode {
                node: type_name,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::WithClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .ctes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::InferClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .index_elems
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::OnConflictClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let infer = NodeEnum::InferClause(n.infer.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&infer, current_depth));
            nodes.push(NestedNode {
                node: infer,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CtesearchClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .search_col_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::CtecycleClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .cycle_col_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let cycle_mark_value = n
                .cycle_mark_value
                .as_ref()
                .unwrap()
                .to_owned()
                .node
                .unwrap();
            nodes.append(&mut get_children(&cycle_mark_value, current_depth));
            nodes.push(NestedNode {
                node: cycle_mark_value,
                depth: current_depth,
            });

            let cycle_mark_default = n
                .cycle_mark_default
                .as_ref()
                .unwrap()
                .to_owned()
                .node
                .unwrap();
            nodes.append(&mut get_children(&cycle_mark_default, current_depth));
            nodes.push(NestedNode {
                node: cycle_mark_default,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::CommonTableExpr(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .aliascolnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            let ctequery = n.ctequery.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&ctequery, current_depth));
            nodes.push(NestedNode {
                node: ctequery,
                depth: current_depth,
            });

            let search_clause =
                NodeEnum::CtesearchClause(n.search_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&search_clause, current_depth));
            nodes.push(NestedNode {
                node: search_clause,
                depth: current_depth,
            });

            let cycle_clause =
                NodeEnum::CtecycleClause(n.cycle_clause.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&cycle_clause, current_depth));
            nodes.push(NestedNode {
                node: cycle_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .ctecolnames
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .ctecoltypes
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .ctecoltypmods
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .ctecolcollations
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::MergeWhenClause(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let condition = n.condition.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&condition, current_depth));
            nodes.push(NestedNode {
                node: condition,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .target_list
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .values
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::RoleSpec(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::TriggerTransition(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::PartitionElem(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let expr = n.expr.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&expr, current_depth));
            nodes.push(NestedNode {
                node: expr,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .collation
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .opclass
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::PartitionSpec(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .part_params
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::PartitionBoundSpec(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes.append(
                &mut n
                    .listdatums
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .lowerdatums
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes.append(
                &mut n
                    .upperdatums
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::PartitionRangeDatum(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let value = n.value.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&value, current_depth));
            nodes.push(NestedNode {
                node: value,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::PartitionCmd(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let name = NodeEnum::RangeVar(n.name.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&name, current_depth));
            nodes.push(NestedNode {
                node: name,
                depth: current_depth,
            });

            let bound = NodeEnum::PartitionBoundSpec(n.bound.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&bound, current_depth));
            nodes.push(NestedNode {
                node: bound,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::VacuumRelation(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .va_cols
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::PublicationObjSpec(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            let pubtable = NodeEnum::PublicationTable(n.pubtable.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&pubtable, current_depth));
            nodes.push(NestedNode {
                node: pubtable,
                depth: current_depth,
            });

            nodes
        }
        NodeEnum::PublicationTable(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            let relation = NodeEnum::RangeVar(n.relation.as_ref().unwrap().to_owned());
            nodes.append(&mut get_children(&relation, current_depth));
            nodes.push(NestedNode {
                node: relation,
                depth: current_depth,
            });

            let where_clause = n.where_clause.as_ref().unwrap().to_owned().node.unwrap();
            nodes.append(&mut get_children(&where_clause, current_depth));
            nodes.push(NestedNode {
                node: where_clause,
                depth: current_depth,
            });

            nodes.append(
                &mut n
                    .columns
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::InlineCodeBlock(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::CallContext(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::Integer(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::Float(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::Boolean(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::String(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::BitString(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
        NodeEnum::List(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::IntList(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::OidList(n) => {
            let mut nodes: Vec<NestedNode> = vec![];
            nodes.append(
                &mut n
                    .items
                    .iter()
                    .flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth))
                    .collect(),
            );

            nodes
        }
        NodeEnum::AConst(n) => {
            let mut nodes: Vec<NestedNode> = vec![];

            nodes
        }
    }
}
