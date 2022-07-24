use super::*;
use husky_liason_semantics::{MemberLiason, ParameterLiason};
use husky_trace_protocol::*;
use husky_visual_syntax::StaticVisualTy;
use std::any::TypeId;
use vm::*;

pub static BINARY_GRID_28_BASE_ROUTE: &'static str =
    "domains::ml::datasets::cv::mnist::BinaryGrid28";

pub static BINARY_GRID_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: BINARY_GRID_28_BASE_ROUTE,
        spatial_parameters: &[],
        static_trait_impls: &[StaticTraitImplDefn {
            dev_src: __static_dev_src!(),
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "b32"),
                EntityStaticDefn {
                    dev_src: husky_dev_utils::__static_dev_src!(),
                    name: "index",
                    items: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_liason: ParameterLiason::MemberAccess,
                        parameters: &[StaticParameter {
                            liason: ParameterLiason::Pure,
                            ty: "i32",
                            name: "todo!()",
                        }],
                        output_ty: "b32",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
                        },
                        spatial_parameters: &[],
                        method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
                            copy_access: __SpecificRoutineFp(|_, values| -> __TempValue {
                                let this_value: &BinaryGrid28 = values[0].downcast_temp_ref();
                                let index_value: usize = values[1]
                                    .take_copyable()
                                    .take_i32()
                                    .try_into()
                                    .expect("todo");
                                this_value
                                    .get(index_value)
                                    .unwrap()
                                    .__take_copyable()
                                    .into()
                            }),
                            eval_ref_access: __SpecificRoutineFp(|_, values| -> __TempValue {
                                todo!()
                            }),
                            temp_ref_access: __SpecificRoutineFp(|_, values| -> __TempValue {
                                todo!()
                            }),
                            move_access: __SpecificRoutineFp(|_, _| todo!()),
                            temp_mut_access: index_temp_mut_fp!(BinaryGrid28),
                            nargs: 2,
                            dev_src: __static_dev_src!(),
                        })),
                    },
                },
            ],
        }],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Array,
        visual_ty: StaticVisualTy::Shape2d,
        opt_type_call: Some(&BINARY_GRID28_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub static BINARY_GRID28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "domains::ml::datasets::cv::mnist::BinaryGrid28",
        output_liason: OutputLiason::Transfer,
        linkage: specific_transfer_linkage!(|_, _values| {
            __TempValue::OwnedEval(__OwnedValue::new(BinaryGrid28::default()))
        },
        some BinaryGrid28::__call__)
        .into(),
    },
    dev_src: __static_dev_src!(),
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28 {
    padded_rows: [u32; 31],
}

impl std::ops::Index<usize> for BinaryGrid28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.padded_rows[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryGrid28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.padded_rows[index]
    }
}

impl std::fmt::Debug for BinaryGrid28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryGrid {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryGrid28 {
    pub fn __call__() -> Self {
        Self {
            padded_rows: Default::default(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<u32> {
        self.padded_rows.get(index).map(|x| *x)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.padded_rows.get_mut(index)
    }
}

impl Serialize for BinaryGrid28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl __HasStaticTypeInfo for BinaryGrid28 {
    type __StaticSelf = Self;

    fn __static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __AnyValue<'eval> for BinaryGrid28 {
    fn __print_short(&self) -> String {
        "BinaryGrid28 { ... }".into()
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
        // __ty_route_from_static_binded::<Self>(BINARY_GRID_28_BASE_ROUTE)
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::BinaryGrid28 {
            padded_rows: self.padded_rows.clone(),
        }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }
}
