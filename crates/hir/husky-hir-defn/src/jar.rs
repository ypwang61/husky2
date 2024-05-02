use crate::*;

#[salsa::jar]
pub struct HirDefnJar(
    // defn
    // - type
    crate::defn::major_item::ty::ty_hir_defn,
    crate::defn::major_item::ty::r#enum::EnumHirDefn,
    crate::defn::major_item::ty::r#enum::enum_hir_defn_dependencies,
    crate::defn::major_item::ty::r#enum::enum_hir_defn_version_stamp,
    crate::defn::major_item::ty::unit_struct::UnitStructHirDefn,
    crate::defn::major_item::ty::unit_struct::unit_struct_hir_defn_dependencies,
    crate::defn::major_item::ty::unit_struct::unit_struct_hir_defn_version_stamp,
    crate::defn::major_item::ty::tuple_struct::TupleStructHirDefn,
    crate::defn::major_item::ty::tuple_struct::tuple_struct_hir_defn_dependencies,
    crate::defn::major_item::ty::tuple_struct::tuple_struct_hir_defn_version_stamp,
    crate::defn::major_item::ty::props_struct::PropsStructHirDefn,
    crate::defn::major_item::ty::props_struct::props_struct_hir_defn_dependencies,
    crate::defn::major_item::ty::props_struct::props_struct_hir_defn_version_stamp,
    crate::defn::major_item::ty::r#extern::ExternHirDefn,
    crate::defn::major_item::ty::r#extern::extern_hir_defn_dependencies,
    crate::defn::major_item::ty::r#extern::extern_hir_defn_version_stamp,
    crate::defn::major_item::ty::union::UnionHirDefn,
    crate::defn::major_item::ty::union::union_hir_defn_dependencies,
    crate::defn::major_item::ty::union::union_hir_defn_version_stamp,
    // - major form
    crate::defn::major_item::form::form_hir_defn,
    crate::defn::major_item::form::val::MajorValHirDefn,
    crate::defn::major_item::form::val::val_hir_defn_dependencies,
    crate::defn::major_item::form::val::val_hir_defn_version_stamp,
    crate::defn::major_item::form::r#const::MajorConstHirDefn,
    crate::defn::major_item::form::r#const::major_const_hir_defn_dependencies,
    crate::defn::major_item::form::r#const::major_const_hir_defn_version_stamp,
    crate::defn::major_item::form::r#static::MajorStaticHirDefn,
    crate::defn::major_item::form::r#static::major_static_hir_defn_dependencies,
    crate::defn::major_item::form::r#static::major_static_hir_defn_version_stamp,
    crate::defn::major_item::form::function_ritchie::MajorFunctionRitchieHirDefn,
    crate::defn::major_item::form::function_ritchie::major_function_ritchie_hir_defn_dependencies,
    crate::defn::major_item::form::function_ritchie::major_function_ritchie_hir_defn_version_stamp,
    // - morphism_defn,
    crate::defn::major_item::form::ty_alias::TypeAliasHirDefn,
    crate::defn::major_item::form::ty_alias::ty_alias_hir_defn_dependencies,
    crate::defn::major_item::form::ty_alias::ty_alias_hir_defn_version_stamp,
    // - type_alias_defn,
    // - trait
    crate::defn::major_item::trai::TraitHirDefn,
    crate::defn::major_item::trai::trai_hir_defn,
    crate::defn::major_item::trai::trai_hir_defn_dependencies,
    crate::defn::major_item::trai::trai_hir_defn_version_stamp,
    // - enum variant,
    crate::defn::ty_variant::enum_unit_ty_variant::EnumUnitVariantHirDefn,
    crate::defn::ty_variant::enum_unit_ty_variant::enum_unit_variant_hir_defn_dependencies,
    crate::defn::ty_variant::enum_unit_ty_variant::enum_unit_variant_hir_defn_version_stamp,
    crate::defn::ty_variant::enum_tuple_ty_variant::EnumTupleVariantHirDefn,
    crate::defn::ty_variant::enum_tuple_ty_variant::enum_tuple_variant_hir_defn_dependencies,
    crate::defn::ty_variant::enum_tuple_ty_variant::enum_tuple_variant_hir_defn_version_stamp,
    crate::defn::ty_variant::enum_props_ty_variant::EnumPropsVariantHirDefn,
    crate::defn::ty_variant::enum_props_ty_variant::enum_props_variant_hir_defn_dependencies,
    crate::defn::ty_variant::enum_props_ty_variant::enum_props_variant_hir_defn_version_stamp,
    // - type item
    crate::defn::assoc_item::ty_item::ty_item_hir_defn,
    crate::defn::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieHirDefn,
    crate::defn::assoc_item::ty_item::assoc_ritchie::ty_assoc_ritchie_hir_defn_dependencies,
    crate::defn::assoc_item::ty_item::assoc_ritchie::ty_assoc_ritchie_hir_defn_version_stamp,
    crate::defn::assoc_item::ty_item::method_ritchie::TypeMethodRitchieHirDefn,
    crate::defn::assoc_item::ty_item::method_ritchie::ty_method_ritchie_hir_defn_dependencies,
    crate::defn::assoc_item::ty_item::method_ritchie::ty_method_ritchie_hir_defn_version_stamp,
    crate::defn::assoc_item::ty_item::assoc_ty::TypeAssocTypeHirDefn,
    crate::defn::assoc_item::ty_item::assoc_ty::ty_assoc_ty_hir_defn_dependencies,
    crate::defn::assoc_item::ty_item::assoc_ty::ty_assoc_ty_hir_defn_version_stamp,
    crate::defn::assoc_item::ty_item::assoc_val::TypeAssocValHirDefn,
    crate::defn::assoc_item::ty_item::assoc_val::ty_assoc_val_hir_defn_dependencies,
    crate::defn::assoc_item::ty_item::assoc_val::ty_assoc_val_hir_defn_version_stamp,
    crate::defn::assoc_item::ty_item::memo_field::TypeMemoizedFieldHirDefn,
    crate::defn::assoc_item::ty_item::memo_field::ty_memo_field_hir_defn_dependencies,
    crate::defn::assoc_item::ty_item::memo_field::ty_memo_field_hir_defn_version_stamp,
    // - trait item
    crate::defn::assoc_item::trai_item::trai_item_hir_defn,
    crate::defn::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieHirDefn,
    crate::defn::assoc_item::trai_item::assoc_ritchie::trai_assoc_fn_hir_defn_dependencies,
    crate::defn::assoc_item::trai_item::assoc_ritchie::trai_assoc_fn_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_item::method_ritchie::TraitMethodFnHirDefn,
    crate::defn::assoc_item::trai_item::method_ritchie::trai_method_ritchie_hir_defn_dependencies,
    crate::defn::assoc_item::trai_item::method_ritchie::trai_method_ritchie_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_item::assoc_ty::TraitAssocTypeHirDefn,
    crate::defn::assoc_item::trai_item::assoc_ty::trai_assoc_ty_hir_defn_dependencies,
    crate::defn::assoc_item::trai_item::assoc_ty::trai_assoc_ty_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_item::assoc_val::TraitAssocValHirDefn,
    crate::defn::assoc_item::trai_item::assoc_val::trai_assoc_val_hir_defn_dependencies,
    crate::defn::assoc_item::trai_item::assoc_val::trai_assoc_val_hir_defn_version_stamp,
    // - trait for type item
    crate::defn::assoc_item::trai_for_ty_item::trai_for_ty_item_hir_defn,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieHirDefn,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ritchie::trai_for_ty_assoc_fn_hir_defn_dependencies,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ritchie::trai_for_ty_assoc_ritchie_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieHirDefn,
    crate::defn::assoc_item::trai_for_ty_item::method_ritchie::trai_for_ty_method_ritchie_hir_defn_dependencies,
    crate::defn::assoc_item::trai_for_ty_item::method_ritchie::trai_for_ty_method_ritchie_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeHirDefn,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ty::trai_for_ty_assoc_ty_hir_defn_dependencies,
    crate::defn::assoc_item::trai_for_ty_item::assoc_ty::trai_for_ty_assoc_ty_hir_defn_version_stamp,
    crate::defn::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValHirDefn,
    crate::defn::assoc_item::trai_for_ty_item::assoc_val::trai_for_ty_assoc_val_hir_defn_dependencies,
    crate::defn::assoc_item::trai_for_ty_item::assoc_val::trai_for_ty_assoc_val_hir_defn_version_stamp,
    // - impl block
    crate::defn::impl_block::ty_impl_block::ty_impl_block_dependencies,
    crate::defn::impl_block::ty_impl_block::ty_impl_block_version_stamp,
    crate::defn::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_dependencies,
    crate::defn::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_version_stamp,
    // dependencies
    crate::dependencies::HirDefnDependencies,
    // version stamp
    crate::version_stamp::HirDefnVersionStamp,
);
