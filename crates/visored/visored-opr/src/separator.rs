use crate::precedence::{VdPrecedence, VdPrecedenceRange};
use enum_index::IsEnumIndex;
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};

#[derive(Debug, PartialEq, Eq, Clone, Copy, IsEnumIndex, Hash)]
pub enum VdBaseSeparator {
    Space,
    Comma,
    Semicolon,
    Add,
    Mul,
    Dot,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Subset,
    Supset,
    Subseteq,
    Supseteq,
    Subseteqq,
    Supseteqq,
    Subsetneq,
    Supsetneq,
    In,
    Notin,
    Times,
    Otimes,
}

impl VdBaseSeparator {
    pub const SPACE: Self = Self::Space;
    pub const COMMA: Self = Self::Comma;
    pub const SEMICOLON: Self = Self::Semicolon;
    pub const ADD: Self = Self::Add;
    pub const MUL: Self = Self::Mul;
    pub const DOT: Self = Self::Dot;
    pub const EQ: Self = Self::Eq;
    pub const NE: Self = Self::Ne;
    pub const LT: Self = Self::Lt;
    pub const GT: Self = Self::Gt;
    pub const LE: Self = Self::Le;
    pub const GE: Self = Self::Ge;
    pub const IN: Self = Self::In;
}

impl VdBaseSeparator {
    pub fn class(self) -> VdSeparatorClass {
        match self {
            VdBaseSeparator::Space => VdSeparatorClass::Space,
            VdBaseSeparator::Comma => VdSeparatorClass::Comma,
            VdBaseSeparator::Semicolon => VdSeparatorClass::Space,
            VdBaseSeparator::Add => VdSeparatorClass::Add,
            VdBaseSeparator::Mul | VdBaseSeparator::Times | VdBaseSeparator::Otimes => {
                VdSeparatorClass::Mul
            }
            VdBaseSeparator::Dot => VdSeparatorClass::Mul,
            VdBaseSeparator::Eq
            | VdBaseSeparator::Ne
            | VdBaseSeparator::Lt
            | VdBaseSeparator::Gt
            | VdBaseSeparator::Le
            | VdBaseSeparator::Ge
            | VdBaseSeparator::Subset
            | VdBaseSeparator::Supset
            | VdBaseSeparator::Subseteq
            | VdBaseSeparator::Supseteq
            | VdBaseSeparator::Subseteqq
            | VdBaseSeparator::Supseteqq
            | VdBaseSeparator::Subsetneq
            | VdBaseSeparator::Supsetneq
            | VdBaseSeparator::In
            | VdBaseSeparator::Notin => VdSeparatorClass::Relation,
        }
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        self.class().left_precedence_range()
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        self.class().right_precedence_range()
    }
}

impl VdBaseSeparator {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdBaseSeparator::Space => VdPrecedence::SPACE,
            VdBaseSeparator::Comma => todo!(),
            VdBaseSeparator::Semicolon => VdPrecedence::SEMICOLON,
            VdBaseSeparator::Add => VdPrecedence::ADD_SUB,
            VdBaseSeparator::Mul | VdBaseSeparator::Times | VdBaseSeparator::Otimes => {
                VdPrecedence::MUL_DIV
            }
            VdBaseSeparator::Dot => VdPrecedence::MUL_DIV,
            VdBaseSeparator::Eq
            | VdBaseSeparator::Ne
            | VdBaseSeparator::Lt
            | VdBaseSeparator::Gt
            | VdBaseSeparator::Le
            | VdBaseSeparator::Ge
            | VdBaseSeparator::Subset
            | VdBaseSeparator::Supset
            | VdBaseSeparator::Subseteq
            | VdBaseSeparator::Supseteq
            | VdBaseSeparator::Subseteqq
            | VdBaseSeparator::Supseteqq
            | VdBaseSeparator::Subsetneq
            | VdBaseSeparator::Supsetneq
            | VdBaseSeparator::In
            | VdBaseSeparator::Notin => VdPrecedence::RELATION,
        }
    }

    pub fn latex_code(self) -> &'static str {
        match self {
            VdBaseSeparator::Space => "␣",
            VdBaseSeparator::Comma => ",",
            VdBaseSeparator::Semicolon => ";",
            VdBaseSeparator::Add => "+",
            VdBaseSeparator::Mul => "\\times",
            VdBaseSeparator::Dot => "\\cdot",
            VdBaseSeparator::Eq => "=",
            VdBaseSeparator::Ne => "\\neq",
            VdBaseSeparator::Lt => "<",
            VdBaseSeparator::Gt => ">",
            VdBaseSeparator::Le => "\\le",
            VdBaseSeparator::Ge => "\\ge",
            VdBaseSeparator::Subset => "\\subseteq",
            VdBaseSeparator::Supset => "\\supseteq",
            VdBaseSeparator::Subseteq => "\\subseteqq",
            VdBaseSeparator::Supseteq => "\\supseteqq",
            VdBaseSeparator::Subseteqq => "\\subseteqqq",
            VdBaseSeparator::Supseteqq => "\\supseteqqq",
            VdBaseSeparator::Subsetneq => "\\subsetneq",
            VdBaseSeparator::Supsetneq => "\\supsetneq",
            VdBaseSeparator::In => "\\in",
            VdBaseSeparator::Notin => "\\notin",
            VdBaseSeparator::Times => "\\times",
            VdBaseSeparator::Otimes => "\\otimes",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSeparatorClass {
    Relation,
    Space,
    Comma,
    Semicolon,
    Mul,
    Add,
}

impl VdSeparatorClass {
    pub const RELATION: Self = Self::Relation;
    pub const SPACE: Self = Self::Space;
    pub const COMMA: Self = Self::Comma;
    pub const SEMICOLON: Self = Self::Semicolon;
    pub const MUL: Self = Self::Mul;
    pub const ADD: Self = Self::Add;
}

impl VdSeparatorClass {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdSeparatorClass::Space => VdPrecedence::SPACE,
            VdSeparatorClass::Comma => VdPrecedence::COMMA,
            VdSeparatorClass::Semicolon => VdPrecedence::SEMICOLON,
            VdSeparatorClass::Add => VdPrecedence::ADD_SUB,
            VdSeparatorClass::Mul => VdPrecedence::MUL_DIV,
            VdSeparatorClass::Relation => VdPrecedence::RELATION,
        }
    }

    pub fn latex_code(self) -> &'static str {
        todo!()
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdSeparatorClass::Space => VdPrecedenceRange::NoLess(VdPrecedence::SPACE),
            VdSeparatorClass::Comma => VdPrecedenceRange::NoLess(VdPrecedence::COMMA),
            VdSeparatorClass::Semicolon => VdPrecedenceRange::NoLess(VdPrecedence::SEMICOLON),
            VdSeparatorClass::Add => VdPrecedenceRange::NoLess(VdPrecedence::ADD_SUB),
            VdSeparatorClass::Mul => VdPrecedenceRange::NoLess(VdPrecedence::MUL_DIV),
            VdSeparatorClass::Relation => VdPrecedenceRange::NoLess(VdPrecedence::RELATION),
        }
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdSeparatorClass::Space => VdPrecedenceRange::Greater(VdPrecedence::SPACE),
            VdSeparatorClass::Comma => VdPrecedenceRange::Greater(VdPrecedence::COMMA),
            VdSeparatorClass::Semicolon => VdPrecedenceRange::Greater(VdPrecedence::SEMICOLON),
            VdSeparatorClass::Add => VdPrecedenceRange::Greater(VdPrecedence::ADD_SUB),
            VdSeparatorClass::Mul => VdPrecedenceRange::Greater(VdPrecedence::MUL_DIV),
            VdSeparatorClass::Relation => VdPrecedenceRange::Greater(VdPrecedence::RELATION),
        }
    }
}

impl VdBaseSeparator {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &::salsa::Db) -> Self {
        let LpCsvExprData::Ident(ref ident) = expr.data else {
            todo!()
        };
        match ident.as_str() {
            "space" => Self::Space,
            "comma" => Self::Comma,
            "semicolon" => Self::Semicolon,
            "add" => Self::Add,
            "mul" => Self::Mul,
            "dot" => Self::Dot,
            "eq" => Self::Eq,
            "ne" => Self::Ne,
            "lt" => Self::Lt,
            "gt" => Self::Gt,
            "le" => Self::Le,
            "ge" => Self::Ge,
            "subset" => Self::Subset,
            "supset" => Self::Supset,
            "in" => Self::In,
            "notin" => Self::Notin,
            "times" => Self::Times,
            "otimes" => Self::Otimes,
            _ => todo!(),
        }
    }
}
