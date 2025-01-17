use visored_opr::{
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr},
    separator::VdBaseSeparator,
};
use visored_signature::{
    menu::{vd_signature_menu, VdSignatureMenu},
    signature::{
        attach::{VdAttachSignature, VdPowerSignature},
        prefix_opr::VdBasePrefixOprSignature,
        separator::base::VdBaseSeparatorSignature,
    },
};
use visored_term::menu::{vd_ty_menu, VdTypeMenu};

use crate::dispatch::{
    attach::VdAttachGlobalDispatch, binary_opr::VdBinaryOprGlobalDispatch,
    frac::VdFracGlobalDispatch, prefix_opr::VdPrefixOprGlobalDispatch,
    separator::VdSeparatorGlobalDispatch, sqrt::VdSqrtGlobalDispatch,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdGlobalDispatchMenu {
    /// # prefix oprs
    /// ## pos
    pub int_pos: VdPrefixOprGlobalDispatch,
    pub rat_pos: VdPrefixOprGlobalDispatch,
    pub real_pos: VdPrefixOprGlobalDispatch,
    pub complex_pos: VdPrefixOprGlobalDispatch,
    /// ## neg
    pub int_neg: VdPrefixOprGlobalDispatch,
    pub rat_neg: VdPrefixOprGlobalDispatch,
    pub real_neg: VdPrefixOprGlobalDispatch,
    pub complex_neg: VdPrefixOprGlobalDispatch,
    /// # binary oprs
    /// ## sub
    pub int_sub: VdBinaryOprGlobalDispatch,
    pub rat_sub: VdBinaryOprGlobalDispatch,
    pub real_sub: VdBinaryOprGlobalDispatch,
    pub complex_sub: VdBinaryOprGlobalDispatch,
    /// ## div
    pub rat_div: VdBinaryOprGlobalDispatch,
    pub real_div: VdBinaryOprGlobalDispatch,
    pub complex_div: VdBinaryOprGlobalDispatch,
    /// # separators
    /// ## add
    pub nat_add: VdSeparatorGlobalDispatch,
    pub int_add: VdSeparatorGlobalDispatch,
    pub rat_add: VdSeparatorGlobalDispatch,
    pub real_add: VdSeparatorGlobalDispatch,
    pub complex_add: VdSeparatorGlobalDispatch,
    /// ## mul
    pub nat_space_mul: VdSeparatorGlobalDispatch,
    pub int_space_mul: VdSeparatorGlobalDispatch,
    pub rat_space_mul: VdSeparatorGlobalDispatch,
    pub real_space_mul: VdSeparatorGlobalDispatch,
    pub complex_space_mul: VdSeparatorGlobalDispatch,
    /// ## power
    pub nat_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub int_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub rat_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub real_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub complex_to_the_power_of_nat: VdAttachGlobalDispatch,
    /// ## eq
    pub nat_eq: VdSeparatorGlobalDispatch,
    pub int_eq: VdSeparatorGlobalDispatch,
    pub rat_eq: VdSeparatorGlobalDispatch,
    pub real_eq: VdSeparatorGlobalDispatch,
    pub complex_eq: VdSeparatorGlobalDispatch,
    /// ## ne
    pub nat_ne: VdSeparatorGlobalDispatch,
    pub int_ne: VdSeparatorGlobalDispatch,
    pub rat_ne: VdSeparatorGlobalDispatch,
    pub real_ne: VdSeparatorGlobalDispatch,
    pub complex_ne: VdSeparatorGlobalDispatch,
    /// ## lt
    pub nat_lt: VdSeparatorGlobalDispatch,
    pub int_lt: VdSeparatorGlobalDispatch,
    pub rat_lt: VdSeparatorGlobalDispatch,
    pub real_lt: VdSeparatorGlobalDispatch,
    /// ## gt
    pub nat_gt: VdSeparatorGlobalDispatch,
    pub int_gt: VdSeparatorGlobalDispatch,
    pub rat_gt: VdSeparatorGlobalDispatch,
    pub real_gt: VdSeparatorGlobalDispatch,
    /// ## le
    pub nat_le: VdSeparatorGlobalDispatch,
    pub int_le: VdSeparatorGlobalDispatch,
    pub rat_le: VdSeparatorGlobalDispatch,
    pub real_le: VdSeparatorGlobalDispatch,
    /// ## ge
    pub nat_ge: VdSeparatorGlobalDispatch,
    pub int_ge: VdSeparatorGlobalDispatch,
    pub rat_ge: VdSeparatorGlobalDispatch,
    pub real_ge: VdSeparatorGlobalDispatch,
    /// ## in
    pub in_set: VdSeparatorGlobalDispatch,
    /// # sqrt
    pub real_sqrt: VdSqrtGlobalDispatch,
    /// # frac
    pub rat_frac: VdFracGlobalDispatch,
    pub real_frac: VdFracGlobalDispatch,
    pub complex_frac: VdFracGlobalDispatch,
}

#[salsa::tracked(return_ref)]
pub fn vd_global_dispatch_menu(db: &::salsa::Db) -> VdGlobalDispatchMenu {
    let VdTypeMenu {
        nat,
        int,
        rat,
        real,
        complex,
        set,
        prop,
    } = *vd_ty_menu(db);
    let VdSignatureMenu {
        int_pos,
        rat_pos,
        real_pos,
        complex_pos,
        int_neg,
        rat_neg,
        real_neg,
        complex_neg,
        int_sub,
        rat_sub,
        real_sub,
        complex_sub,
        nat_add,
        int_add,
        rat_add,
        real_add,
        complex_add,
        nat_mul,
        int_mul,
        rat_mul,
        real_mul,
        complex_mul,
        rat_div,
        real_div,
        complex_div,
        nat_to_the_power_of_nat,
        int_to_the_power_of_nat,
        rat_to_the_power_of_nat,
        real_to_the_power_of_nat,
        complex_to_the_power_of_nat,
        nat_eq,
        int_eq,
        rat_eq,
        real_eq,
        complex_eq,
        nat_ne,
        int_ne,
        rat_ne,
        real_ne,
        complex_ne,
        nat_lt,
        int_lt,
        rat_lt,
        real_lt,
        nat_gt,
        int_gt,
        rat_gt,
        real_gt,
        nat_le,
        int_le,
        rat_le,
        real_le,
        nat_ge,
        int_ge,
        rat_ge,
        real_ge,
        real_sqrt,
    } = *vd_signature_menu(db);
    let pre = |base_opr, signature| VdPrefixOprGlobalDispatch::Base {
        base_opr,
        signature,
    };
    let bin = |base_binary_opr, signature| VdBinaryOprGlobalDispatch::Normal {
        base_binary_opr,
        signature,
    };
    let sep = |base_separator, signature| VdSeparatorGlobalDispatch::Normal {
        base_separator,
        signature,
    };
    let pow = |signature| VdAttachGlobalDispatch::Normal {
        signature: VdAttachSignature::Power(signature),
    };
    VdGlobalDispatchMenu {
        // # prefix oprs
        // ## pos
        int_pos: pre(VdBasePrefixOpr::POS, int_pos),
        rat_pos: pre(VdBasePrefixOpr::POS, rat_pos),
        real_pos: pre(VdBasePrefixOpr::POS, real_pos),
        complex_pos: pre(VdBasePrefixOpr::POS, complex_pos),
        // ## neg
        int_neg: pre(VdBasePrefixOpr::NEG, int_neg),
        rat_neg: pre(VdBasePrefixOpr::NEG, rat_neg),
        real_neg: pre(VdBasePrefixOpr::NEG, real_neg),
        complex_neg: pre(VdBasePrefixOpr::NEG, complex_neg),
        // # binary oprs
        // ## sub
        int_sub: bin(VdBaseBinaryOpr::Sub, int_sub),
        rat_sub: bin(VdBaseBinaryOpr::Sub, rat_sub),
        real_sub: bin(VdBaseBinaryOpr::Sub, real_sub),
        complex_sub: bin(VdBaseBinaryOpr::Sub, complex_sub),
        // ## div
        rat_div: bin(VdBaseBinaryOpr::Div, rat_div),
        real_div: bin(VdBaseBinaryOpr::Div, real_div),
        complex_div: bin(VdBaseBinaryOpr::Div, complex_div),
        // # separators
        // ## add
        nat_add: sep(VdBaseSeparator::Add, nat_add),
        int_add: sep(VdBaseSeparator::Add, int_add),
        rat_add: sep(VdBaseSeparator::Add, rat_add),
        real_add: sep(VdBaseSeparator::Add, real_add),
        complex_add: sep(VdBaseSeparator::Add, complex_add),
        // ## mul
        nat_space_mul: sep(VdBaseSeparator::Space, nat_mul),
        int_space_mul: sep(VdBaseSeparator::Space, int_mul),
        rat_space_mul: sep(VdBaseSeparator::Space, rat_mul),
        real_space_mul: sep(VdBaseSeparator::Space, real_mul),
        complex_space_mul: sep(VdBaseSeparator::Space, complex_mul),
        // ## power
        nat_to_the_power_of_nat: pow(nat_to_the_power_of_nat),
        int_to_the_power_of_nat: pow(int_to_the_power_of_nat),
        rat_to_the_power_of_nat: pow(rat_to_the_power_of_nat),
        real_to_the_power_of_nat: pow(real_to_the_power_of_nat),
        complex_to_the_power_of_nat: pow(complex_to_the_power_of_nat),
        // ## eq
        nat_eq: sep(VdBaseSeparator::Eq, nat_eq),
        int_eq: sep(VdBaseSeparator::Eq, int_eq),
        rat_eq: sep(VdBaseSeparator::Eq, rat_eq),
        real_eq: sep(VdBaseSeparator::Eq, real_eq),
        complex_eq: sep(VdBaseSeparator::Eq, complex_eq),
        // ## ne
        nat_ne: sep(VdBaseSeparator::Ne, nat_ne),
        int_ne: sep(VdBaseSeparator::Ne, int_ne),
        rat_ne: sep(VdBaseSeparator::Ne, rat_ne),
        real_ne: sep(VdBaseSeparator::Ne, real_ne),
        complex_ne: sep(VdBaseSeparator::Ne, complex_ne),
        // ## lt
        nat_lt: sep(VdBaseSeparator::Lt, nat_lt),
        int_lt: sep(VdBaseSeparator::Lt, int_lt),
        rat_lt: sep(VdBaseSeparator::Lt, rat_lt),
        real_lt: sep(VdBaseSeparator::Lt, real_lt),
        // ## gt
        nat_gt: sep(VdBaseSeparator::Gt, nat_gt),
        int_gt: sep(VdBaseSeparator::Gt, int_gt),
        rat_gt: sep(VdBaseSeparator::Gt, rat_gt),
        real_gt: sep(VdBaseSeparator::Gt, real_gt),
        // ## le
        nat_le: sep(VdBaseSeparator::Le, nat_le),
        int_le: sep(VdBaseSeparator::Le, int_le),
        rat_le: sep(VdBaseSeparator::Le, rat_le),
        real_le: sep(VdBaseSeparator::Le, real_le),
        // ## ge
        nat_ge: sep(VdBaseSeparator::Ge, nat_ge),
        int_ge: sep(VdBaseSeparator::Ge, int_ge),
        rat_ge: sep(VdBaseSeparator::Ge, rat_ge),
        real_ge: sep(VdBaseSeparator::Ge, real_ge),
        // ## in
        in_set: VdSeparatorGlobalDispatch::InSet { expr_ty: prop },
        // # sqrt
        real_sqrt: VdSqrtGlobalDispatch::Base {
            signature: real_sqrt,
        },
        // # frac
        rat_frac: VdFracGlobalDispatch::Div { signature: rat_div },
        real_frac: VdFracGlobalDispatch::Div {
            signature: real_div,
        },
        complex_frac: VdFracGlobalDispatch::Div {
            signature: complex_div,
        },
    }
}
