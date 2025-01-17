#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]
pub mod dev_eval_context;
pub mod exception;
pub mod linket_impl;
pub mod linket_impls;
pub mod pedestal;
pub mod static_var;
#[cfg(feature = "ugly")]
pub mod ugly;
pub mod var_id;

use crate::linket_impl::LinketImplTrackedException;
use crate::linket_impl::LinketImplVmControlFlow;
use crate::pedestal::IsPedestalFull;
use dev_eval_context::{DevEvalContext, IsDevRuntimeInterfaceDyn};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiArgumentReprInterface;
use husky_value::vm_control_flow::VmControlFlow;
use husky_wild_utils::arb_ref;
use linket_impl::{
    IsLinketImpl, LinketImplFrozenValue, LinketImplKiControlFlow, LinketImplThawedValue,
    VmArgumentValue,
};
use smallvec::SmallVec;

pub type LinketImplVmControlFlowThawed<LinketImpl, C = LinketImplThawedValue<LinketImpl>> =
    VmControlFlow<C, LinketImplThawedValue<LinketImpl>, LinketImplTrackedException<LinketImpl>>;
pub type LinketImplVmControlFlowFrozen<LinketImpl, C = LinketImplFrozenValue<LinketImpl>> =
    VmControlFlow<C, LinketImplFrozenValue<LinketImpl>, LinketImplTrackedException<LinketImpl>>;

pub trait IsFnKiLinketImplSource<LinketImpl: IsLinketImpl, FnPointer>:
    IsFnVmLinketImplSource<LinketImpl, FnPointer>
{
    fn into_fn_linket_impl(
        self,
        fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> LinketImplKiControlFlow<LinketImpl>,
        fn_vm_wrapper: fn(
            SmallVec<[VmArgumentValue<LinketImpl>; 4]>,
        ) -> LinketImplVmControlFlow<LinketImpl>,
        fn_pointer: FnPointer,
    ) -> LinketImpl;

    fn fn_ki_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<LinketImpl, Self::FnOutput>;
}

pub trait IsFnVmLinketImplSource<LinketImpl: IsLinketImpl, FnPointer> {
    type FnOutput;

    fn into_fn_linket_impl_vm_only(
        self,
        fn_vm_wrapper: fn(
            SmallVec<[VmArgumentValue<LinketImpl>; 4]>,
        ) -> LinketImplVmControlFlow<LinketImpl>,
        fn_pointer: FnPointer,
    ) -> LinketImpl;

    fn fn_vm_wrapper_aux(
        self,
        arguments: SmallVec<[VmArgumentValue<LinketImpl>; 4]>,
    ) -> LinketImplVmControlFlow<LinketImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! fn_linket_impl {
    (vm only $fn_item: expr) => {{
        fn fn_vm_wrapper(arguments: __SmallVec<[__VmArgumentValue; 4]>) -> __VmControlFlow {
            // todo: catch unwind
            __VmControlFlow::Continue(
                unsafe {
                    FnLinketImplSource($fn_item)
                        .fn_vm_wrapper_aux(arguments)?
                        .into_thawed()
                }
                .into_thawed_value(),
            )
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coercion
        FnLinketImplSource($fn_item).into_fn_linket_impl_vm_only(fn_vm_wrapper, $fn_item)
    }};
    ($fn_item: expr) => {{
        fn fn_ki_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                FnLinketImplSource($fn_item)
                    .fn_ki_wrapper_aux(arguments)?
                    .into_value(),
            )
        }
        fn fn_vm_wrapper(arguments: __SmallVec<[__VmArgumentValue; 4]>) -> __VmControlFlow {
            // todo: catch unwind
            __VmControlFlow::Continue(
                unsafe {
                    FnLinketImplSource($fn_item)
                        .fn_vm_wrapper_aux(arguments)?
                        .into_thawed()
                }
                .into_thawed_value(),
            )
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coercion
        FnLinketImplSource($fn_item).into_fn_linket_impl(fn_ki_wrapper, fn_vm_wrapper, $fn_item)
    }};
}

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_fn_linket_impl_source {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, $($input,)* $output> IsFnKiLinketImplSource<LinketImpl, fn($($input,)*) -> $output> for FnLinketImplSource<F>
        where
            LinketImpl: IsLinketImpl,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue + Boiled,)*
            $output: Send,
        {
            fn into_fn_linket_impl(
                self,
                fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardKiControlFlow,
                fn_vm_wrapper: fn(SmallVec<[StandardVmArgumentValue; 4]>) -> StandardVmControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinketImpl {
                LinketImpl::RitchieFn {
                    fn_ki_wrapper: Some(fn_ki_wrapper),
                    fn_vm_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn fn_ki_wrapper_aux(
                self,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardKiControlFlow<Self::FnOutput> {
                let ctx = LinketImpl::dev_eval_context();
                #[allow(unused_variables)]
                let mut arguments = arguments.iter();
                #[allow(unused_variables)]
                let slush_values = &mut SlushValues::default();
                ki_catch_unwind!(
                    self.0,
                    $({
                        let argument = arguments.next().unwrap();
                        match *argument {
                            KiArgumentReprInterface::Simple(ki_repr_interface) => {
                                <$input as FromValue>::from_value_temp(
                                    ctx.eval_ki_repr_interface(ki_repr_interface)?,
                                    (slush_values)
                                )
                            },
                            KiArgumentReprInterface::Keyed(argument) => todo!("KiArgumentReprInterface::Keyed(argument)"),
                            KiArgumentReprInterface::Variadic(ref ki_repr_interfaces) => {
                                <$input as FromValue>::from_variadic_values(
                                    ki_repr_interfaces.iter().map(
                                        |&ki_repr_interface| ctx.eval_ki_repr_interface(ki_repr_interface)
                                    ),
                                    Some(slush_values)
                                )?
                            },
                            KiArgumentReprInterface::Branch { .. } => unreachable!(),
                            KiArgumentReprInterface::RuntimeConstants(ref argument) => todo!(),
                        }}),*
                )
            }
        }

        #[allow(non_snake_case, unused_mut)]
        impl<F, $($input,)* $output> IsFnVmLinketImplSource<LinketImpl, fn($($input,)*) -> $output> for FnLinketImplSource<F>
        where
            LinketImpl: IsLinketImpl,
            F: Fn($($input,)*) -> $output,
            $($input: Send + Boiled,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linket_impl_vm_only(
                self,
                fn_vm_wrapper: fn(SmallVec<[StandardVmArgumentValue; 4]>) -> StandardVmControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinketImpl {
                LinketImpl::RitchieFn {
                    fn_ki_wrapper: None,
                    fn_vm_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }


            fn fn_vm_wrapper_aux(
                self,
                arguments: SmallVec<[VmArgumentValue<LinketImpl>;4]>,
            ) -> StandardVmControlFlow<Self::FnOutput> {
                let ctx = LinketImpl::dev_eval_context();
                #[allow(unused_variables)]
                let mut arguments = arguments.into_iter();
                #[allow(unused_variables)]
                let slush_values = &mut SlushValues::default();
                vm_catch_unwind!(
                    self.0,
                    $({
                        let argument = arguments.next().unwrap();
                        match argument  {
                            VmArgumentValue::Simple(value) => {
                                unsafe {
                                    $input::from_thawed(<$input::Thawed as FromThawedValue>::from_thawed_value_temp(
                                        value,
                                        (slush_values)
                                    ))
                                }
                            },
                            VmArgumentValue::Keyed(value_opt) => {
                                todo!()
                            },
                            VmArgumentValue::Variadic(values) => {
                                todo!()
                            },
                            VmArgumentValue::RuntimeConstants(constants) => {
                                todo!()
                            },
                        }
                    }),*
                )
            }
        }
    };
}

#[macro_export]
macro_rules! ty_default_linket_impl {
    ($ty: ty) => {
        fn_linket_impl!(|| <$ty as Default>::default())
    };
}

// unveils

pub trait IsUnveilFnLinketImplSource<LinketImpl: IsLinketImpl, Target, FnPointer> {
    type FnOutput;

    fn into_unveil_linket_impl(
        self,
        fn_ki_wrapper: fn(
            arguments: &[KiArgumentReprInterface],
        ) -> LinketImplKiControlFlow<LinketImpl>,
        fn_vm_wrapper: fn(
            arguments: [VmArgumentValue<LinketImpl>; 2],
        ) -> LinketImplVmControlFlow<LinketImpl>,
        fn_pointer: FnPointer,
    ) -> LinketImpl;
    fn unveil_fn_ki_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<LinketImpl, Self::FnOutput>;
    fn unveil_fn_vm_wrapper_aux(
        self,
        arguments: [VmArgumentValue<LinketImpl>; 2],
    ) -> LinketImplVmControlFlow<LinketImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! unveil_fn_linket_impl {
    ($fn_item: expr) => {{
        fn fn_ki_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                UnveilFnLinketImplSource($fn_item)
                    .unveil_fn_ki_wrapper_aux(arguments)?
                    .into_value(),
            )
        }
        fn fn_vm_wrapper(arguments: [__VmArgumentValue; 2]) -> __VmControlFlow {
            // todo: catch unwind
            __VmControlFlow::Continue(
                unsafe {
                    UnveilFnLinketImplSource($fn_item)
                        .unveil_fn_vm_wrapper_aux(arguments)?
                        .into_thawed()
                }
                .into_thawed_value(),
            )
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coercion
        UnveilFnLinketImplSource($fn_item).into_unveil_linket_impl(
            fn_ki_wrapper,
            fn_vm_wrapper,
            $fn_item,
        )
    }};
}

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_unveil_fn_linket_impl_source {
    (
        [$($runtime_constant: ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, B, Target, $($runtime_constant,)* $output> IsUnveilFnLinketImplSource<
            LinketImpl,
            Target,
            fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>
        > for UnveilFnLinketImplSource<F>
        where
            F: Fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            B: IntoValue + IntoThawedValue,
            Target: Send + FromValue + FromThawedValue,
            $($runtime_constant: Send + FromValue,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_unveil_linket_impl(
                self,
                fn_ki_wrapper: fn(
                    &[KiArgumentReprInterface],
                ) -> StandardKiControlFlow,
                fn_vm_wrapper: fn(
                    [StandardVmArgumentValue;2],
                ) -> StandardVmControlFlow,
                fn_pointer: fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            ) -> LinketImpl {
                LinketImpl::RitchieUnveilFn {
                    fn_ki_wrapper,
                    fn_vm_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn unveil_fn_ki_wrapper_aux(
                self,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardKiControlFlow<Self::FnOutput> {
                let ctx = LinketImpl::dev_eval_context();
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentReprInterface::Simple(target) = arguments[0] else {
                    unreachable!("expect ordinary argument")
                };
                let KiArgumentReprInterface::RuntimeConstants(
                    ref runtime_constants
                ) = arguments[1] else {
                    unreachable!("expect runtime constants, but got {:?} instead", arguments[1])
                };
                let slush_values = &mut SlushValues::default();
                let mut runtime_constants = runtime_constants.iter();
                ki_catch_unwind2!(
                    self.0,
                    |cf| match cf {
                        std::ops::ControlFlow::Continue(c) => KiControlFlow::Continue(c),
                        std::ops::ControlFlow::Break(b) => KiControlFlow::Return(b.into_value()),
                    },
                    <Target as FromValue>::from_value_temp(
                        ctx.eval_ki_repr_interface(target)?,
                        slush_values
                    ),
                    ($(<$runtime_constant as FromValue>::from_value_temp(
                        ctx.eval_ki_runtime_compterm(
                            *runtime_constants.next().expect("missing runtime constant")
                        ),
                        slush_values
                    ),)*)
                )
            }

            fn unveil_fn_vm_wrapper_aux(
                self,
                arguments: [VmArgumentValue<LinketImpl>;2],
            ) -> StandardVmControlFlow<Self::FnOutput> {
                let ctx = LinketImpl::dev_eval_context();
                let [arg0,arg1] =arguments;
                let VmArgumentValue::Simple(target) = arg0 else {
                    unreachable!("expect ordinary argument")
                };
                let VmArgumentValue::RuntimeConstants(
                    runtime_constants
                ) = arg1 else {
                    unreachable!("expect runtime constants, but got {:?} instead", arg1)
                };
                let slush_values = &mut SlushValues::default();
                let mut runtime_constants = runtime_constants.into_iter();
                vm_catch_unwind2!(
                    self.0,
                    |cf| match cf {
                        std::ops::ControlFlow::Continue(c) => VmControlFlow::Continue(c),
                        std::ops::ControlFlow::Break(b) => VmControlFlow::Return(b.into_thawed_value()),
                    },
                    <Target as FromThawedValue>::from_thawed_value_temp(
                        target,
                        slush_values
                    ),
                    ($(<$runtime_constant as FromValue>::from_value_temp(
                        ctx.eval_ki_runtime_compterm(
                            *runtime_constants.next().expect("missing runtime constant")
                        ),
                        slush_values
                    ),)*)
                )
            }
        }
    };
}
