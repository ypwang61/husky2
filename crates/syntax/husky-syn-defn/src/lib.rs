#![feature(trait_upcasting)]
mod db;
mod defn;
mod error;
pub mod helpers;
#[cfg(test)]
mod tests;
mod utils;

pub use self::db::*;
pub use self::defn::*;
pub use self::error::*;

use husky_entity_path::*;
use husky_entity_syn_tree::*;
use husky_syn_decl::*;
use husky_syn_expr::*;
use husky_vfs::{ModulePath, SubmodulePath};

#[salsa::jar(db = SynDefnDb)]
pub struct SynDefnJar(
    // type
    ty_syn_node_defn,
    ty_syn_defn,
    EnumTypeSynNodeDefn,
    EnumTypeSynDefn,
    UnitStructTypeSynNodeDefn,
    UnitStructTypeSynDefn,
    TupleStructTypeSynNodeDefn,
    TupleStructTypeSynDefn,
    PropsStructTypeSynNodeDefn,
    PropsStructTypeSynDefn,
    RecordTypeSynNodeDefn,
    RecordTypeSynDefn,
    InductiveTypeSynNodeDefn,
    InductiveTypeSynDefn,
    StructureTypeSynNodeDefn,
    StructureTypeSynDefn,
    ExternTypeSynNodeDefn,
    ExternTypeSynDefn,
    UnionTypeSynNodeDefn,
    UnionTypeSynDefn,
    // fugitive
    fugitive_syn_node_defn,
    // fugitive_syn_defn,
    ValSynNodeDefn,
    ValSynDefn,
    FnSynNodeDefn,
    FnSynDefn,
    GnSynNodeDefn,
    GnSynDefn,
    // morphism_defn,
    TypeAliasSynNodeDefn,
    TypeAliasSynDefn,
    // type_alias_defn,
    // trait
    TraitSynNodeDefn,
    trai_syn_node_defn,
    TraitSynDefn,
    trai_syn_defn,
    // enum variant,
    UnitVariantSynNodeDefn,
    UnitVariantSynDefn,
    TupleVariantSynNodeDefn,
    TupleVariantSynDefn,
    PropsVariantSynNodeDefn,
    PropsVariantSynDefn,
    // type item
    ty_item_syn_node_defn,
    // ty_item_syn_defn,
    TypeAssociatedFnSynNodeDefn,
    TypeAssociatedFnSynDefn,
    TypeMethodFnSynNodeDefn,
    TypeMethodFnSynDefn,
    TypeAssociatedTypeSynNodeDefn,
    TypeAssociatedTypeSynDefn,
    TypeAssociatedValSynNodeDefn,
    TypeAssociatedValSynDefn,
    TypeMemoizedFieldSynNodeDefn,
    TypeMemoizedFieldSynDefn,
    // trait item
    trai_item_syn_node_defn,
    // trai_item_syn_defn,
    TraitAssociatedFnSynNodeDefn,
    TraitAssociatedFnSynDefn,
    TraitMethodFnSynNodeDefn,
    TraitMethodFnSynDefn,
    TraitAssociatedTypeSynNodeDefn,
    TraitAssociatedTypeSynDefn,
    TraitAssociatedValSynNodeDefn,
    TraitAssociatedValSynDefn,
    // trait for type item
    trai_for_ty_item_syn_node_defn,
    // trai_for_ty_item_syn_defn,
    TraitForTypeAssociatedFnSynNodeDefn,
    TraitForTypeAssociatedFnSynDefn,
    TraitForTypeMethodFnSynNodeDefn,
    TraitForTypeMethodFnSynDefn,
    TraitForTypeAssociatedTypeSynNodeDefn,
    TraitForTypeAssociatedTypeSynDefn,
    TraitForTypeAssociatedValSynNodeDefn,
    TraitForTypeAssociatedValSynDefn,
    // sheet
    module_syn_defns,
    module_syn_node_defns,
);
