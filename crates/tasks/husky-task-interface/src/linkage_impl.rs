use serde::Serialize;

use crate::{val_control_flow::ValControlFlow, val_repr::ValReprInterface, value::IsValue};
use crate::{val_repr::ValArgumentReprInterface, DevEvalContext};

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: std::fmt::Debug + Copy + 'static;
    type Value: IsValue;
    type Error: std::fmt::Debug + Serialize;

    /// assumed that pedestal has already been
    fn eval(
        self,
        val_repr_interface: ValReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<Self>;
}

pub type LinkageImplValControlFlow<LinkageImpl, C = <LinkageImpl as IsLinkageImpl>::Value> =
    ValControlFlow<C, <LinkageImpl as IsLinkageImpl>::Value, <LinkageImpl as IsLinkageImpl>::Error>;
