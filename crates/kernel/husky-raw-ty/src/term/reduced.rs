use super::*;

pub(crate) fn calc_reduced_raw_term(db: &dyn RawTypeDb, raw_term: RawTerm) -> ReducedRawTerm {
    // ad hoc
    ReducedRawTerm(raw_term)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReducedRawTerm(RawTerm);

impl ReducedRawTerm {
    pub fn raw_term(&self) -> RawTerm {
        self.0
    }

    pub fn new_category(u: impl Into<RawTermUniverse>) -> Self {
        ReducedRawTerm(RawTermCategory::new(u.into()).into())
    }
}

impl<Db: RawTypeDb + ?Sized> salsa::DebugWithDb<Db> for ReducedRawTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        self.0.fmt(f, db, level)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducedRawTermMenu<'a> {
    raw_term_menu: &'a RawTermMenu,
}

impl<'a> ReducedRawTermMenu<'a> {
    pub(crate) fn new(raw_term_menu: &'a RawTermMenu) -> Self {
        Self { raw_term_menu }
    }

    pub fn never(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.never())
    }

    pub fn unit(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.unit())
    }

    pub fn bool(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.bool())
    }

    pub fn i8(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.i8())
    }

    pub fn i16(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.i16())
    }

    pub fn i32(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.i32())
    }

    pub fn r32(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.r32())
    }

    pub fn list(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.list())
    }

    pub fn static_str_ref(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.static_str_ref())
    }

    pub fn universe0(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.universe0().into())
    }

    pub fn universe1(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.universe1().into())
    }

    /// `Prop`
    pub fn prop(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.prop().into())
    }

    /// `RawType`
    pub fn ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.ty0().into())
    }

    // pub fn core(&self) -> ReducedRawTerm {
    //     self.raw_term_menu.core
    // }

    // pub fn core_ops(&self) -> ReducedRawTerm {
    //     self.raw_term_menu.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_add())
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_add_assign())
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_and())
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_and_assign())
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_or())
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_or_assign())
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_xor())
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_bit_or_assign())
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_div())
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_div_assign())
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_mul())
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_mul_assign())
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_neg())
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.core_ops_not())
    }

    pub fn option_ty_path(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.option_ty_path())
    }

    pub fn slice_ty_path(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.slice_ty_path())
    }

    pub fn ref_ty_path(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.ref_ty_path())
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.invariant_ty0_to_trai_ty().into())
    }

    pub fn covariant_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.explicit_covariant_ty0_to_ty0().into())
    }

    pub fn contravariant_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(
            self.raw_term_menu
                .explicit_contravariant_ty0_to_ty0()
                .into(),
        )
    }

    pub fn invariant_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.ex_inv_ty0_to_ty0().into())
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(
            self.raw_term_menu
                .ex_co_lifetime_to_ex_co_ty0_to_ty0()
                .into(),
        )
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(
            self.raw_term_menu
                .ex_co_lifetime_to_ex_ct_ty0_to_ty0()
                .into(),
        )
    }

    pub fn covariant_lifetime_to_invariant_ty0_to_ty0(&self) -> ReducedRawTerm {
        ReducedRawTerm(
            self.raw_term_menu
                .ex_co_lifetime_to_ex_inv_ty0_to_ty0()
                .into(),
        )
    }

    pub fn trai_ty(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.trai_ty())
    }

    pub fn module(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.module())
    }

    pub fn i64(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.i64())
    }

    pub fn f32(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.f32())
    }

    pub fn f64(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.f64())
    }

    pub fn r64(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.r64())
    }

    pub fn eval_lifetime(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.eval_lifetime())
    }

    pub fn static_lifetime(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.static_lifetime())
    }

    pub fn lifetime_ty(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.lifetime_ty())
    }

    pub fn str_ty_path(&self) -> ReducedRawTerm {
        ReducedRawTerm(self.raw_term_menu.str_ty_path())
    }

    pub fn raw_term_menu(&self) -> &'a RawTermMenu {
        self.raw_term_menu
    }
}