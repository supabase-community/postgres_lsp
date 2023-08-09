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
    let mut nodes: Vec<NestedNode> = vec![];
    // Node, depth, location
    let mut stack: Vec<(NodeEnum, i32)> = vec![(node.to_owned(), current_depth)];
    while stack.len() > 0 {
        let (node, depth) = stack.pop().unwrap();
        let current_depth = depth + 1;
        match &node {
            NodeEnum::Alias(n) => {
                n.colnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RangeVar(n) => {
                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::TableFunc(n) => {
                n.ns_uris.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.ns_names.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.docexpr.is_some() {
                    let docexpr = n.docexpr.to_owned().unwrap().node.unwrap();
                    stack.push((docexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: docexpr,
                        depth: current_depth,
                    });
                }

                if n.rowexpr.is_some() {
                    let rowexpr = n.rowexpr.to_owned().unwrap().node.unwrap();
                    stack.push((rowexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rowexpr,
                        depth: current_depth,
                    });
                }

                n.colnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.coltypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.coltypmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.colcollations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.colexprs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.coldefexprs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::Var(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::Param(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::Aggref(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.aggargtypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.aggdirectargs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.aggorder.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.aggdistinct.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.aggfilter.is_some() {
                    let aggfilter = n.aggfilter.to_owned().unwrap().node.unwrap();
                    stack.push((aggfilter.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: aggfilter,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::GroupingFunc(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.refs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.cols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::WindowFunc(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.aggfilter.is_some() {
                    let aggfilter = n.aggfilter.to_owned().unwrap().node.unwrap();
                    stack.push((aggfilter.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: aggfilter,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SubscriptingRef(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.refupperindexpr.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.reflowerindexpr.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.refexpr.is_some() {
                    let refexpr = n.refexpr.to_owned().unwrap().node.unwrap();
                    stack.push((refexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: refexpr,
                        depth: current_depth,
                    });
                }

                if n.refassgnexpr.is_some() {
                    let refassgnexpr = n.refassgnexpr.to_owned().unwrap().node.unwrap();
                    stack.push((refassgnexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: refassgnexpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::FuncExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::NamedArgExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::OpExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DistinctExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::NullIfExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ScalarArrayOpExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::BoolExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::SubLink(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.testexpr.is_some() {
                    let testexpr = n.testexpr.to_owned().unwrap().node.unwrap();
                    stack.push((testexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: testexpr,
                        depth: current_depth,
                    });
                }

                n.oper_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.subselect.is_some() {
                    let subselect = n.subselect.to_owned().unwrap().node.unwrap();
                    stack.push((subselect.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: subselect,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SubPlan(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.testexpr.is_some() {
                    let testexpr = n.testexpr.to_owned().unwrap().node.unwrap();
                    stack.push((testexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: testexpr,
                        depth: current_depth,
                    });
                }

                n.param_ids.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.set_param.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.par_param.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlternativeSubPlan(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.subplans.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::FieldSelect(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::FieldStore(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                n.newvals.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.fieldnums.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RelabelType(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CoerceViaIo(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ArrayCoerceExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                if n.elemexpr.is_some() {
                    let elemexpr = n.elemexpr.to_owned().unwrap().node.unwrap();
                    stack.push((elemexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: elemexpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ConvertRowtypeExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CollateExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CaseExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.defresult.is_some() {
                    let defresult = n.defresult.to_owned().unwrap().node.unwrap();
                    stack.push((defresult.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: defresult,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CaseWhen(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }

                if n.result.is_some() {
                    let result = n.result.to_owned().unwrap().node.unwrap();
                    stack.push((result.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: result,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CaseTestExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ArrayExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.elements.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RowExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.colnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RowCompareExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.opnos.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.opfamilies.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.inputcollids.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.largs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.rargs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CoalesceExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::MinMaxExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::SqlvalueFunction(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::XmlExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                n.named_args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.arg_names.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::NullTest(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::BooleanTest(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CoerceToDomain(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CoerceToDomainValue(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SetToDefault(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CurrentOfExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::NextValueExpr(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::InferenceElem(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::TargetEntry(n) => {
                if n.xpr.is_some() {
                    let xpr = n.xpr.to_owned().unwrap().node.unwrap();
                    stack.push((xpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: xpr,
                        depth: current_depth,
                    });
                }

                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeTblRef(n) => (),
            NodeEnum::JoinExpr(n) => {
                if n.larg.is_some() {
                    let larg = n.larg.to_owned().unwrap().node.unwrap();
                    stack.push((larg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: larg,
                        depth: current_depth,
                    });
                }

                if n.rarg.is_some() {
                    let rarg = n.rarg.to_owned().unwrap().node.unwrap();
                    stack.push((rarg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rarg,
                        depth: current_depth,
                    });
                }

                n.using_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.join_using_alias.is_some() {
                    let join_using_alias = NodeEnum::Alias(n.join_using_alias.to_owned().unwrap());
                    stack.push((join_using_alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: join_using_alias,
                        depth: current_depth,
                    });
                }

                if n.quals.is_some() {
                    let quals = n.quals.to_owned().unwrap().node.unwrap();
                    stack.push((quals.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: quals,
                        depth: current_depth,
                    });
                }

                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::FromExpr(n) => {
                n.fromlist.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.quals.is_some() {
                    let quals = n.quals.to_owned().unwrap().node.unwrap();
                    stack.push((quals.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: quals,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::OnConflictExpr(n) => {
                n.arbiter_elems.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.arbiter_where.is_some() {
                    let arbiter_where = n.arbiter_where.to_owned().unwrap().node.unwrap();
                    stack.push((arbiter_where.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arbiter_where,
                        depth: current_depth,
                    });
                }

                n.on_conflict_set.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.on_conflict_where.is_some() {
                    let on_conflict_where = n.on_conflict_where.to_owned().unwrap().node.unwrap();
                    stack.push((on_conflict_where.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: on_conflict_where,
                        depth: current_depth,
                    });
                }

                n.excl_rel_tlist.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::IntoClause(n) => {
                if n.rel.is_some() {
                    let rel = NodeEnum::RangeVar(n.rel.to_owned().unwrap());
                    stack.push((rel.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rel,
                        depth: current_depth,
                    });
                }

                n.col_names.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.view_query.is_some() {
                    let view_query = n.view_query.to_owned().unwrap().node.unwrap();
                    stack.push((view_query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: view_query,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::MergeAction(n) => {
                if n.qual.is_some() {
                    let qual = n.qual.to_owned().unwrap().node.unwrap();
                    stack.push((qual.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: qual,
                        depth: current_depth,
                    });
                }

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.update_colnos.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RawStmt(n) => {
                if n.stmt.is_some() {
                    let stmt = n.stmt.to_owned().unwrap().node.unwrap();
                    stack.push((stmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: stmt,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::Query(n) => {
                if n.utility_stmt.is_some() {
                    let utility_stmt = n.utility_stmt.to_owned().unwrap().node.unwrap();
                    stack.push((utility_stmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: utility_stmt,
                        depth: current_depth,
                    });
                }

                n.cte_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.rtable.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.jointree.is_some() {
                    let jointree = NodeEnum::FromExpr(n.jointree.to_owned().unwrap());
                    stack.push((jointree.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: jointree,
                        depth: current_depth,
                    });
                }

                n.merge_action_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.on_conflict.is_some() {
                    let on_conflict = NodeEnum::OnConflictExpr(n.on_conflict.to_owned().unwrap());
                    stack.push((on_conflict.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: on_conflict,
                        depth: current_depth,
                    });
                }

                n.returning_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.group_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.grouping_sets.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.having_qual.is_some() {
                    let having_qual = n.having_qual.to_owned().unwrap().node.unwrap();
                    stack.push((having_qual.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: having_qual,
                        depth: current_depth,
                    });
                }

                n.window_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.distinct_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.sort_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.limit_offset.is_some() {
                    let limit_offset = n.limit_offset.to_owned().unwrap().node.unwrap();
                    stack.push((limit_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: limit_offset,
                        depth: current_depth,
                    });
                }

                if n.limit_count.is_some() {
                    let limit_count = n.limit_count.to_owned().unwrap().node.unwrap();
                    stack.push((limit_count.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: limit_count,
                        depth: current_depth,
                    });
                }

                n.row_marks.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.set_operations.is_some() {
                    let set_operations = n.set_operations.to_owned().unwrap().node.unwrap();
                    stack.push((set_operations.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: set_operations,
                        depth: current_depth,
                    });
                }

                n.constraint_deps.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.with_check_options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::InsertStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.cols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.select_stmt.is_some() {
                    let select_stmt = n.select_stmt.to_owned().unwrap().node.unwrap();
                    stack.push((select_stmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: select_stmt,
                        depth: current_depth,
                    });
                }

                if n.on_conflict_clause.is_some() {
                    let on_conflict_clause =
                        NodeEnum::OnConflictClause(n.on_conflict_clause.to_owned().unwrap());
                    stack.push((on_conflict_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: on_conflict_clause,
                        depth: current_depth,
                    });
                }

                n.returning_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.with_clause.is_some() {
                    let with_clause = NodeEnum::WithClause(n.with_clause.to_owned().unwrap());
                    stack.push((with_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::DeleteStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.using_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.returning_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.with_clause.is_some() {
                    let with_clause = NodeEnum::WithClause(n.with_clause.to_owned().unwrap());
                    stack.push((with_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::UpdateStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.from_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.returning_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.with_clause.is_some() {
                    let with_clause = NodeEnum::WithClause(n.with_clause.to_owned().unwrap());
                    stack.push((with_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::MergeStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.source_relation.is_some() {
                    let source_relation = n.source_relation.to_owned().unwrap().node.unwrap();
                    stack.push((source_relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: source_relation,
                        depth: current_depth,
                    });
                }

                if n.join_condition.is_some() {
                    let join_condition = n.join_condition.to_owned().unwrap().node.unwrap();
                    stack.push((join_condition.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: join_condition,
                        depth: current_depth,
                    });
                }

                n.merge_when_clauses.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.with_clause.is_some() {
                    let with_clause = NodeEnum::WithClause(n.with_clause.to_owned().unwrap());
                    stack.push((with_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SelectStmt(n) => {
                n.distinct_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.into_clause.is_some() {
                    let into_clause = NodeEnum::IntoClause(n.into_clause.to_owned().unwrap());
                    stack.push((into_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: into_clause,
                        depth: current_depth,
                    });
                }

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.from_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.group_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.having_clause.is_some() {
                    let having_clause = n.having_clause.to_owned().unwrap().node.unwrap();
                    stack.push((having_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: having_clause,
                        depth: current_depth,
                    });
                }

                n.window_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.values_lists.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.sort_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.limit_offset.is_some() {
                    let limit_offset = n.limit_offset.to_owned().unwrap().node.unwrap();
                    stack.push((limit_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: limit_offset,
                        depth: current_depth,
                    });
                }

                if n.limit_count.is_some() {
                    let limit_count = n.limit_count.to_owned().unwrap().node.unwrap();
                    stack.push((limit_count.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: limit_count,
                        depth: current_depth,
                    });
                }

                n.locking_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.with_clause.is_some() {
                    let with_clause = NodeEnum::WithClause(n.with_clause.to_owned().unwrap());
                    stack.push((with_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_clause,
                        depth: current_depth,
                    });
                }

                if n.larg.is_some() {
                    let larg = NodeEnum::SelectStmt(n.larg.to_owned().unwrap());
                    stack.push((larg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: larg,
                        depth: current_depth,
                    });
                }

                if n.rarg.is_some() {
                    let rarg = NodeEnum::SelectStmt(n.rarg.to_owned().unwrap());
                    stack.push((rarg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rarg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ReturnStmt(n) => {
                if n.returnval.is_some() {
                    let returnval = n.returnval.to_owned().unwrap().node.unwrap();
                    stack.push((returnval.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: returnval,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::PlassignStmt(n) => {
                n.indirection.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.val.is_some() {
                    let val = NodeEnum::SelectStmt(n.val.to_owned().unwrap());
                    stack.push((val.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: val,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterTableStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.cmds.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterTableCmd(n) => {
                if n.newowner.is_some() {
                    let newowner = NodeEnum::RoleSpec(n.newowner.to_owned().unwrap());
                    stack.push((newowner.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: newowner,
                        depth: current_depth,
                    });
                }

                if n.def.is_some() {
                    let def = n.def.to_owned().unwrap().node.unwrap();
                    stack.push((def.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: def,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterDomainStmt(n) => {
                n.type_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.def.is_some() {
                    let def = n.def.to_owned().unwrap().node.unwrap();
                    stack.push((def.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: def,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SetOperationStmt(n) => {
                if n.larg.is_some() {
                    let larg = n.larg.to_owned().unwrap().node.unwrap();
                    stack.push((larg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: larg,
                        depth: current_depth,
                    });
                }

                if n.rarg.is_some() {
                    let rarg = n.rarg.to_owned().unwrap().node.unwrap();
                    stack.push((rarg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rarg,
                        depth: current_depth,
                    });
                }

                n.col_types.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.col_typmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.col_collations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.group_clauses.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::GrantStmt(n) => {
                n.objects.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.privileges.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.grantees.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.grantor.is_some() {
                    let grantor = NodeEnum::RoleSpec(n.grantor.to_owned().unwrap());
                    stack.push((grantor.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: grantor,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::GrantRoleStmt(n) => {
                n.granted_roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.grantee_roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.grantor.is_some() {
                    let grantor = NodeEnum::RoleSpec(n.grantor.to_owned().unwrap());
                    stack.push((grantor.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: grantor,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterDefaultPrivilegesStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.action.is_some() {
                    let action = NodeEnum::GrantStmt(n.action.to_owned().unwrap());
                    stack.push((action.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: action,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ClosePortalStmt(n) => (),
            NodeEnum::ClusterStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CopyStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }

                n.attlist.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.table_elts.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.inh_relations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.partbound.is_some() {
                    let partbound = NodeEnum::PartitionBoundSpec(n.partbound.to_owned().unwrap());
                    stack.push((partbound.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: partbound,
                        depth: current_depth,
                    });
                }

                if n.partspec.is_some() {
                    let partspec = NodeEnum::PartitionSpec(n.partspec.to_owned().unwrap());
                    stack.push((partspec.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: partspec,
                        depth: current_depth,
                    });
                }

                if n.of_typename.is_some() {
                    let of_typename = NodeEnum::TypeName(n.of_typename.to_owned().unwrap());
                    stack.push((of_typename.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: of_typename,
                        depth: current_depth,
                    });
                }

                n.constraints.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DefineStmt(n) => {
                n.defnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.definition.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropStmt(n) => {
                n.objects.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::TruncateStmt(n) => {
                n.relations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CommentStmt(n) => {
                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::FetchStmt(n) => (),
            NodeEnum::IndexStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.index_params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.index_including_params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.exclude_op_names.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateFunctionStmt(n) => {
                n.funcname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.parameters.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.return_type.is_some() {
                    let return_type = NodeEnum::TypeName(n.return_type.to_owned().unwrap());
                    stack.push((return_type.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: return_type,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.sql_body.is_some() {
                    let sql_body = n.sql_body.to_owned().unwrap().node.unwrap();
                    stack.push((sql_body.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: sql_body,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterFunctionStmt(n) => {
                if n.func.is_some() {
                    let func = NodeEnum::ObjectWithArgs(n.func.to_owned().unwrap());
                    stack.push((func.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: func,
                        depth: current_depth,
                    });
                }

                n.actions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DoStmt(n) => {
                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RenameStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RuleStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.actions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::NotifyStmt(n) => (),
            NodeEnum::ListenStmt(n) => (),
            NodeEnum::UnlistenStmt(n) => (),
            NodeEnum::TransactionStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ViewStmt(n) => {
                if n.view.is_some() {
                    let view = NodeEnum::RangeVar(n.view.to_owned().unwrap());
                    stack.push((view.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: view,
                        depth: current_depth,
                    });
                }

                n.aliases.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::LoadStmt(n) => (),
            NodeEnum::CreateDomainStmt(n) => {
                n.domainname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }

                if n.coll_clause.is_some() {
                    let coll_clause = NodeEnum::CollateClause(n.coll_clause.to_owned().unwrap());
                    stack.push((coll_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: coll_clause,
                        depth: current_depth,
                    });
                }

                n.constraints.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreatedbStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropdbStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::VacuumStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.rels.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ExplainStmt(n) => {
                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateTableAsStmt(n) => {
                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }

                if n.into.is_some() {
                    let into = NodeEnum::IntoClause(n.into.to_owned().unwrap());
                    stack.push((into.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: into,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateSeqStmt(n) => {
                if n.sequence.is_some() {
                    let sequence = NodeEnum::RangeVar(n.sequence.to_owned().unwrap());
                    stack.push((sequence.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: sequence,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterSeqStmt(n) => {
                if n.sequence.is_some() {
                    let sequence = NodeEnum::RangeVar(n.sequence.to_owned().unwrap());
                    stack.push((sequence.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: sequence,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::VariableSetStmt(n) => {
                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::VariableShowStmt(n) => (),
            NodeEnum::DiscardStmt(n) => (),
            NodeEnum::CreateTrigStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.funcname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.columns.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.when_clause.is_some() {
                    let when_clause = n.when_clause.to_owned().unwrap().node.unwrap();
                    stack.push((when_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: when_clause,
                        depth: current_depth,
                    });
                }

                n.transition_rels.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.constrrel.is_some() {
                    let constrrel = NodeEnum::RangeVar(n.constrrel.to_owned().unwrap());
                    stack.push((constrrel.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: constrrel,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreatePlangStmt(n) => {
                n.plhandler.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.plinline.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.plvalidator.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateRoleStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterRoleStmt(n) => {
                if n.role.is_some() {
                    let role = NodeEnum::RoleSpec(n.role.to_owned().unwrap());
                    stack.push((role.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: role,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropRoleStmt(n) => {
                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::LockStmt(n) => {
                n.relations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ConstraintsSetStmt(n) => {
                n.constraints.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ReindexStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CheckPointStmt(n) => (),
            NodeEnum::CreateSchemaStmt(n) => {
                if n.authrole.is_some() {
                    let authrole = NodeEnum::RoleSpec(n.authrole.to_owned().unwrap());
                    stack.push((authrole.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: authrole,
                        depth: current_depth,
                    });
                }

                n.schema_elts.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterDatabaseStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterDatabaseRefreshCollStmt(n) => (),
            NodeEnum::AlterDatabaseSetStmt(n) => {
                if n.setstmt.is_some() {
                    let setstmt = NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap());
                    stack.push((setstmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: setstmt,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterRoleSetStmt(n) => {
                if n.role.is_some() {
                    let role = NodeEnum::RoleSpec(n.role.to_owned().unwrap());
                    stack.push((role.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: role,
                        depth: current_depth,
                    });
                }

                if n.setstmt.is_some() {
                    let setstmt = NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap());
                    stack.push((setstmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: setstmt,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateConversionStmt(n) => {
                n.conversion_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.func_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateCastStmt(n) => {
                if n.sourcetype.is_some() {
                    let sourcetype = NodeEnum::TypeName(n.sourcetype.to_owned().unwrap());
                    stack.push((sourcetype.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: sourcetype,
                        depth: current_depth,
                    });
                }

                if n.targettype.is_some() {
                    let targettype = NodeEnum::TypeName(n.targettype.to_owned().unwrap());
                    stack.push((targettype.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: targettype,
                        depth: current_depth,
                    });
                }

                if n.func.is_some() {
                    let func = NodeEnum::ObjectWithArgs(n.func.to_owned().unwrap());
                    stack.push((func.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: func,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateOpClassStmt(n) => {
                n.opclassname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.opfamilyname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.datatype.is_some() {
                    let datatype = NodeEnum::TypeName(n.datatype.to_owned().unwrap());
                    stack.push((datatype.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: datatype,
                        depth: current_depth,
                    });
                }

                n.items.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateOpFamilyStmt(n) => {
                n.opfamilyname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterOpFamilyStmt(n) => {
                n.opfamilyname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.items.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::PrepareStmt(n) => {
                n.argtypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ExecuteStmt(n) => {
                n.params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DeallocateStmt(n) => (),
            NodeEnum::DeclareCursorStmt(n) => {
                if n.query.is_some() {
                    let query = n.query.to_owned().unwrap().node.unwrap();
                    stack.push((query.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: query,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateTableSpaceStmt(n) => {
                if n.owner.is_some() {
                    let owner = NodeEnum::RoleSpec(n.owner.to_owned().unwrap());
                    stack.push((owner.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: owner,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropTableSpaceStmt(n) => (),
            NodeEnum::AlterObjectDependsStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }

                if n.extname.is_some() {
                    let extname = NodeEnum::String(n.extname.to_owned().unwrap());
                    stack.push((extname.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: extname,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterObjectSchemaStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterOwnerStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }

                if n.newowner.is_some() {
                    let newowner = NodeEnum::RoleSpec(n.newowner.to_owned().unwrap());
                    stack.push((newowner.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: newowner,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterOperatorStmt(n) => {
                if n.opername.is_some() {
                    let opername = NodeEnum::ObjectWithArgs(n.opername.to_owned().unwrap());
                    stack.push((opername.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: opername,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterTypeStmt(n) => {
                n.type_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropOwnedStmt(n) => {
                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ReassignOwnedStmt(n) => {
                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.newrole.is_some() {
                    let newrole = NodeEnum::RoleSpec(n.newrole.to_owned().unwrap());
                    stack.push((newrole.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: newrole,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CompositeTypeStmt(n) => {
                if n.typevar.is_some() {
                    let typevar = NodeEnum::RangeVar(n.typevar.to_owned().unwrap());
                    stack.push((typevar.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: typevar,
                        depth: current_depth,
                    });
                }

                n.coldeflist.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateEnumStmt(n) => {
                n.type_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.vals.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateRangeStmt(n) => {
                n.type_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterEnumStmt(n) => {
                n.type_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterTsdictionaryStmt(n) => {
                n.dictname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterTsconfigurationStmt(n) => {
                n.cfgname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.tokentype.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.dicts.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateFdwStmt(n) => {
                n.func_options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterFdwStmt(n) => {
                n.func_options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateForeignServerStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterForeignServerStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateUserMappingStmt(n) => {
                if n.user.is_some() {
                    let user = NodeEnum::RoleSpec(n.user.to_owned().unwrap());
                    stack.push((user.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: user,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterUserMappingStmt(n) => {
                if n.user.is_some() {
                    let user = NodeEnum::RoleSpec(n.user.to_owned().unwrap());
                    stack.push((user.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: user,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropUserMappingStmt(n) => {
                if n.user.is_some() {
                    let user = NodeEnum::RoleSpec(n.user.to_owned().unwrap());
                    stack.push((user.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: user,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterTableSpaceOptionsStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterTableMoveAllStmt(n) => {
                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::SecLabelStmt(n) => {
                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateForeignTableStmt(n) => {
                if n.base_stmt.is_some() {
                    let base_stmt = NodeEnum::CreateStmt(n.base_stmt.to_owned().unwrap());
                    stack.push((base_stmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: base_stmt,
                        depth: current_depth,
                    });
                }

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ImportForeignSchemaStmt(n) => {
                n.table_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateExtensionStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterExtensionStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterExtensionContentsStmt(n) => {
                if n.object.is_some() {
                    let object = n.object.to_owned().unwrap().node.unwrap();
                    stack.push((object.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: object,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateEventTrigStmt(n) => {
                n.whenclause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.funcname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterEventTrigStmt(n) => (),
            NodeEnum::RefreshMatViewStmt(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ReplicaIdentityStmt(n) => (),
            NodeEnum::AlterSystemStmt(n) => {
                if n.setstmt.is_some() {
                    let setstmt = NodeEnum::VariableSetStmt(n.setstmt.to_owned().unwrap());
                    stack.push((setstmt.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: setstmt,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreatePolicyStmt(n) => {
                if n.table.is_some() {
                    let table = NodeEnum::RangeVar(n.table.to_owned().unwrap());
                    stack.push((table.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: table,
                        depth: current_depth,
                    });
                }

                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.qual.is_some() {
                    let qual = n.qual.to_owned().unwrap().node.unwrap();
                    stack.push((qual.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: qual,
                        depth: current_depth,
                    });
                }

                if n.with_check.is_some() {
                    let with_check = n.with_check.to_owned().unwrap().node.unwrap();
                    stack.push((with_check.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_check,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AlterPolicyStmt(n) => {
                if n.table.is_some() {
                    let table = NodeEnum::RangeVar(n.table.to_owned().unwrap());
                    stack.push((table.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: table,
                        depth: current_depth,
                    });
                }

                n.roles.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.qual.is_some() {
                    let qual = n.qual.to_owned().unwrap().node.unwrap();
                    stack.push((qual.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: qual,
                        depth: current_depth,
                    });
                }

                if n.with_check.is_some() {
                    let with_check = n.with_check.to_owned().unwrap().node.unwrap();
                    stack.push((with_check.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: with_check,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateTransformStmt(n) => {
                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }

                if n.fromsql.is_some() {
                    let fromsql = NodeEnum::ObjectWithArgs(n.fromsql.to_owned().unwrap());
                    stack.push((fromsql.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: fromsql,
                        depth: current_depth,
                    });
                }

                if n.tosql.is_some() {
                    let tosql = NodeEnum::ObjectWithArgs(n.tosql.to_owned().unwrap());
                    stack.push((tosql.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: tosql,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CreateAmStmt(n) => {
                n.handler_name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreatePublicationStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.pubobjects.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterPublicationStmt(n) => {
                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.pubobjects.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateSubscriptionStmt(n) => {
                n.publication.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterSubscriptionStmt(n) => {
                n.publication.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DropSubscriptionStmt(n) => (),
            NodeEnum::CreateStatsStmt(n) => {
                n.defnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.stat_types.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.exprs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.relations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterCollationStmt(n) => {
                n.collname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CallStmt(n) => {
                if n.funccall.is_some() {
                    let funccall = NodeEnum::FuncCall(n.funccall.to_owned().unwrap());
                    stack.push((funccall.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: funccall,
                        depth: current_depth,
                    });
                }

                if n.funcexpr.is_some() {
                    let funcexpr = NodeEnum::FuncExpr(n.funcexpr.to_owned().unwrap());
                    stack.push((funcexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: funcexpr,
                        depth: current_depth,
                    });
                }

                n.outargs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AlterStatsStmt(n) => {
                n.defnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AExpr(n) => {
                n.name.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.lexpr.is_some() {
                    let lexpr = n.lexpr.to_owned().unwrap().node.unwrap();
                    stack.push((lexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: lexpr,
                        depth: current_depth,
                    });
                }

                if n.rexpr.is_some() {
                    let rexpr = n.rexpr.to_owned().unwrap().node.unwrap();
                    stack.push((rexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rexpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::ColumnRef(n) => {
                n.fields.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ParamRef(n) => (),
            NodeEnum::FuncCall(n) => {
                n.funcname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.agg_order.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.agg_filter.is_some() {
                    let agg_filter = n.agg_filter.to_owned().unwrap().node.unwrap();
                    stack.push((agg_filter.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: agg_filter,
                        depth: current_depth,
                    });
                }

                if n.over.is_some() {
                    let over = NodeEnum::WindowDef(n.over.to_owned().unwrap());
                    stack.push((over.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: over,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AStar(n) => (),
            NodeEnum::AIndices(n) => {
                if n.lidx.is_some() {
                    let lidx = n.lidx.to_owned().unwrap().node.unwrap();
                    stack.push((lidx.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: lidx,
                        depth: current_depth,
                    });
                }

                if n.uidx.is_some() {
                    let uidx = n.uidx.to_owned().unwrap().node.unwrap();
                    stack.push((uidx.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: uidx,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::AIndirection(n) => {
                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                n.indirection.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AArrayExpr(n) => {
                n.elements.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ResTarget(n) => {
                n.indirection.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.val.is_some() {
                    let val = n.val.to_owned().unwrap().node.unwrap();
                    stack.push((val.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: val,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::MultiAssignRef(n) => {
                if n.source.is_some() {
                    let source = n.source.to_owned().unwrap().node.unwrap();
                    stack.push((source.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: source,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::TypeCast(n) => {
                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CollateClause(n) => {
                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }

                n.collname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::SortBy(n) => {
                if n.node.is_some() {
                    let node = n.node.to_owned().unwrap().node.unwrap();
                    stack.push((node.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: node,
                        depth: current_depth,
                    });
                }

                n.use_op.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::WindowDef(n) => {
                n.partition_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.order_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.start_offset.is_some() {
                    let start_offset = n.start_offset.to_owned().unwrap().node.unwrap();
                    stack.push((start_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: start_offset,
                        depth: current_depth,
                    });
                }

                if n.end_offset.is_some() {
                    let end_offset = n.end_offset.to_owned().unwrap().node.unwrap();
                    stack.push((end_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: end_offset,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeSubselect(n) => {
                if n.subquery.is_some() {
                    let subquery = n.subquery.to_owned().unwrap().node.unwrap();
                    stack.push((subquery.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: subquery,
                        depth: current_depth,
                    });
                }

                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeFunction(n) => {
                n.functions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }

                n.coldeflist.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RangeTableSample(n) => {
                if n.relation.is_some() {
                    let relation = n.relation.to_owned().unwrap().node.unwrap();
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.method.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.repeatable.is_some() {
                    let repeatable = n.repeatable.to_owned().unwrap().node.unwrap();
                    stack.push((repeatable.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: repeatable,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeTableFunc(n) => {
                if n.docexpr.is_some() {
                    let docexpr = n.docexpr.to_owned().unwrap().node.unwrap();
                    stack.push((docexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: docexpr,
                        depth: current_depth,
                    });
                }

                if n.rowexpr.is_some() {
                    let rowexpr = n.rowexpr.to_owned().unwrap().node.unwrap();
                    stack.push((rowexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: rowexpr,
                        depth: current_depth,
                    });
                }

                n.namespaces.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.columns.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeTableFuncCol(n) => {
                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }

                if n.colexpr.is_some() {
                    let colexpr = n.colexpr.to_owned().unwrap().node.unwrap();
                    stack.push((colexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: colexpr,
                        depth: current_depth,
                    });
                }

                if n.coldefexpr.is_some() {
                    let coldefexpr = n.coldefexpr.to_owned().unwrap().node.unwrap();
                    stack.push((coldefexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: coldefexpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::TypeName(n) => {
                n.names.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.typmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.array_bounds.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ColumnDef(n) => {
                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }

                if n.raw_default.is_some() {
                    let raw_default = n.raw_default.to_owned().unwrap().node.unwrap();
                    stack.push((raw_default.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: raw_default,
                        depth: current_depth,
                    });
                }

                if n.cooked_default.is_some() {
                    let cooked_default = n.cooked_default.to_owned().unwrap().node.unwrap();
                    stack.push((cooked_default.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: cooked_default,
                        depth: current_depth,
                    });
                }

                if n.identity_sequence.is_some() {
                    let identity_sequence =
                        NodeEnum::RangeVar(n.identity_sequence.to_owned().unwrap());
                    stack.push((identity_sequence.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: identity_sequence,
                        depth: current_depth,
                    });
                }

                if n.coll_clause.is_some() {
                    let coll_clause = NodeEnum::CollateClause(n.coll_clause.to_owned().unwrap());
                    stack.push((coll_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: coll_clause,
                        depth: current_depth,
                    });
                }

                n.constraints.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.fdwoptions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::IndexElem(n) => {
                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }

                n.collation.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.opclass.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.opclassopts.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::StatsElem(n) => {
                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::Constraint(n) => {
                if n.raw_expr.is_some() {
                    let raw_expr = n.raw_expr.to_owned().unwrap().node.unwrap();
                    stack.push((raw_expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: raw_expr,
                        depth: current_depth,
                    });
                }

                n.keys.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.including.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.exclusions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.options.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                if n.pktable.is_some() {
                    let pktable = NodeEnum::RangeVar(n.pktable.to_owned().unwrap());
                    stack.push((pktable.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: pktable,
                        depth: current_depth,
                    });
                }

                n.fk_attrs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.pk_attrs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.fk_del_set_cols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.old_conpfeqop.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::DefElem(n) => {
                if n.arg.is_some() {
                    let arg = n.arg.to_owned().unwrap().node.unwrap();
                    stack.push((arg.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::RangeTblEntry(n) => {
                if n.tablesample.is_some() {
                    let tablesample =
                        NodeEnum::TableSampleClause(n.tablesample.to_owned().unwrap());
                    stack.push((tablesample.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: tablesample,
                        depth: current_depth,
                    });
                }

                if n.subquery.is_some() {
                    let subquery = NodeEnum::Query(n.subquery.to_owned().unwrap());
                    stack.push((subquery.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: subquery,
                        depth: current_depth,
                    });
                }

                n.joinaliasvars.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.joinleftcols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.joinrightcols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.join_using_alias.is_some() {
                    let join_using_alias = NodeEnum::Alias(n.join_using_alias.to_owned().unwrap());
                    stack.push((join_using_alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: join_using_alias,
                        depth: current_depth,
                    });
                }

                n.functions.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.tablefunc.is_some() {
                    let tablefunc = NodeEnum::TableFunc(n.tablefunc.to_owned().unwrap());
                    stack.push((tablefunc.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: tablefunc,
                        depth: current_depth,
                    });
                }

                n.values_lists.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.coltypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.coltypmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.colcollations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.alias.is_some() {
                    let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
                    stack.push((alias.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: alias,
                        depth: current_depth,
                    });
                }

                if n.eref.is_some() {
                    let eref = NodeEnum::Alias(n.eref.to_owned().unwrap());
                    stack.push((eref.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: eref,
                        depth: current_depth,
                    });
                }

                n.security_quals.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RangeTblFunction(n) => {
                if n.funcexpr.is_some() {
                    let funcexpr = n.funcexpr.to_owned().unwrap().node.unwrap();
                    stack.push((funcexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: funcexpr,
                        depth: current_depth,
                    });
                }

                n.funccolnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.funccoltypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.funccoltypmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.funccolcollations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::TableSampleClause(n) => {
                n.args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.repeatable.is_some() {
                    let repeatable = n.repeatable.to_owned().unwrap().node.unwrap();
                    stack.push((repeatable.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: repeatable,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::WithCheckOption(n) => {
                if n.qual.is_some() {
                    let qual = n.qual.to_owned().unwrap().node.unwrap();
                    stack.push((qual.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: qual,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::SortGroupClause(n) => (),
            NodeEnum::GroupingSet(n) => {
                n.content.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::WindowClause(n) => {
                n.partition_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.order_clause.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.start_offset.is_some() {
                    let start_offset = n.start_offset.to_owned().unwrap().node.unwrap();
                    stack.push((start_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: start_offset,
                        depth: current_depth,
                    });
                }

                if n.end_offset.is_some() {
                    let end_offset = n.end_offset.to_owned().unwrap().node.unwrap();
                    stack.push((end_offset.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: end_offset,
                        depth: current_depth,
                    });
                }

                n.run_condition.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::ObjectWithArgs(n) => {
                n.objname.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.objargs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.objfuncargs.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AccessPriv(n) => {
                n.cols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CreateOpClassItem(n) => {
                if n.name.is_some() {
                    let name = NodeEnum::ObjectWithArgs(n.name.to_owned().unwrap());
                    stack.push((name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: name,
                        depth: current_depth,
                    });
                }

                n.order_family.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.class_args.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.storedtype.is_some() {
                    let storedtype = NodeEnum::TypeName(n.storedtype.to_owned().unwrap());
                    stack.push((storedtype.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: storedtype,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::TableLikeClause(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::FunctionParameter(n) => {
                if n.arg_type.is_some() {
                    let arg_type = NodeEnum::TypeName(n.arg_type.to_owned().unwrap());
                    stack.push((arg_type.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: arg_type,
                        depth: current_depth,
                    });
                }

                if n.defexpr.is_some() {
                    let defexpr = n.defexpr.to_owned().unwrap().node.unwrap();
                    stack.push((defexpr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: defexpr,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::LockingClause(n) => {
                n.locked_rels.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RowMarkClause(n) => (),
            NodeEnum::XmlSerialize(n) => {
                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }

                if n.type_name.is_some() {
                    let type_name = NodeEnum::TypeName(n.type_name.to_owned().unwrap());
                    stack.push((type_name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: type_name,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::WithClause(n) => {
                n.ctes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::InferClause(n) => {
                n.index_elems.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::OnConflictClause(n) => {
                if n.infer.is_some() {
                    let infer = NodeEnum::InferClause(n.infer.to_owned().unwrap());
                    stack.push((infer.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: infer,
                        depth: current_depth,
                    });
                }

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CtesearchClause(n) => {
                n.search_col_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::CtecycleClause(n) => {
                n.cycle_col_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.cycle_mark_value.is_some() {
                    let cycle_mark_value = n.cycle_mark_value.to_owned().unwrap().node.unwrap();
                    stack.push((cycle_mark_value.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: cycle_mark_value,
                        depth: current_depth,
                    });
                }

                if n.cycle_mark_default.is_some() {
                    let cycle_mark_default = n.cycle_mark_default.to_owned().unwrap().node.unwrap();
                    stack.push((cycle_mark_default.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: cycle_mark_default,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::CommonTableExpr(n) => {
                n.aliascolnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                if n.ctequery.is_some() {
                    let ctequery = n.ctequery.to_owned().unwrap().node.unwrap();
                    stack.push((ctequery.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: ctequery,
                        depth: current_depth,
                    });
                }

                if n.search_clause.is_some() {
                    let search_clause =
                        NodeEnum::CtesearchClause(n.search_clause.to_owned().unwrap());
                    stack.push((search_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: search_clause,
                        depth: current_depth,
                    });
                }

                if n.cycle_clause.is_some() {
                    let cycle_clause = NodeEnum::CtecycleClause(n.cycle_clause.to_owned().unwrap());
                    stack.push((cycle_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: cycle_clause,
                        depth: current_depth,
                    });
                }

                n.ctecolnames.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.ctecoltypes.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.ctecoltypmods.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.ctecolcollations.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::MergeWhenClause(n) => {
                if n.condition.is_some() {
                    let condition = n.condition.to_owned().unwrap().node.unwrap();
                    stack.push((condition.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: condition,
                        depth: current_depth,
                    });
                }

                n.target_list.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.values.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::RoleSpec(n) => (),
            NodeEnum::TriggerTransition(n) => (),
            NodeEnum::PartitionElem(n) => {
                if n.expr.is_some() {
                    let expr = n.expr.to_owned().unwrap().node.unwrap();
                    stack.push((expr.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: expr,
                        depth: current_depth,
                    });
                }

                n.collation.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.opclass.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::PartitionSpec(n) => {
                n.part_params.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::PartitionBoundSpec(n) => {
                n.listdatums.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.lowerdatums.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });

                n.upperdatums.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::PartitionRangeDatum(n) => {
                if n.value.is_some() {
                    let value = n.value.to_owned().unwrap().node.unwrap();
                    stack.push((value.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: value,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::PartitionCmd(n) => {
                if n.name.is_some() {
                    let name = NodeEnum::RangeVar(n.name.to_owned().unwrap());
                    stack.push((name.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: name,
                        depth: current_depth,
                    });
                }

                if n.bound.is_some() {
                    let bound = NodeEnum::PartitionBoundSpec(n.bound.to_owned().unwrap());
                    stack.push((bound.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: bound,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::VacuumRelation(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                n.va_cols.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::PublicationObjSpec(n) => {
                if n.pubtable.is_some() {
                    let pubtable = NodeEnum::PublicationTable(n.pubtable.to_owned().unwrap());
                    stack.push((pubtable.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: pubtable,
                        depth: current_depth,
                    });
                }
            }
            NodeEnum::PublicationTable(n) => {
                if n.relation.is_some() {
                    let relation = NodeEnum::RangeVar(n.relation.to_owned().unwrap());
                    stack.push((relation.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: relation,
                        depth: current_depth,
                    });
                }

                if n.where_clause.is_some() {
                    let where_clause = n.where_clause.to_owned().unwrap().node.unwrap();
                    stack.push((where_clause.to_owned(), current_depth));
                    nodes.push(NestedNode {
                        node: where_clause,
                        depth: current_depth,
                    });
                }

                n.columns.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::InlineCodeBlock(n) => (),
            NodeEnum::CallContext(n) => (),
            NodeEnum::Integer(n) => (),
            NodeEnum::Float(n) => (),
            NodeEnum::Boolean(n) => (),
            NodeEnum::String(n) => (),
            NodeEnum::BitString(n) => (),
            NodeEnum::List(n) => {
                n.items.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::IntList(n) => {
                n.items.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::OidList(n) => {
                n.items.iter().for_each(|x| {
                    stack.push((x.node.to_owned().unwrap(), current_depth));
                    nodes.push(NestedNode {
                        node: x.node.to_owned().unwrap(),
                        depth: current_depth,
                    });
                });
            }
            NodeEnum::AConst(n) => {
                if n.val.is_some() {
                    let value = match n.val.to_owned().unwrap() {
                        pg_query::protobuf::a_const::Val::Ival(v) => NodeEnum::Integer(v),
                        pg_query::protobuf::a_const::Val::Fval(v) => NodeEnum::Float(v),
                        pg_query::protobuf::a_const::Val::Boolval(v) => NodeEnum::Boolean(v),
                        pg_query::protobuf::a_const::Val::Sval(v) => NodeEnum::String(v),
                        pg_query::protobuf::a_const::Val::Bsval(v) => NodeEnum::BitString(v),
                    };

                    nodes.push(NestedNode {
                        node: value,
                        depth: current_depth,
                    });
                }
            }
        };
    }
    nodes
}
