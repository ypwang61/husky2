use crate::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenProps {
    pub kind: TraceTokenKind,
    pub value: Cow<'static, str>,
    pub associated_trace: Option<Arc<Trace>>,
}

impl Serialize for TokenProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TokenProps", 3)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field(
            "associated_trace",
            &self.associated_trace.as_ref().map(|trace| trace.id),
        )?;
        state.end()
    }
}

impl From<EvalResult<'static>> for TokenProps {
    fn from(result: EvalResult<'static>) -> Self {
        match result {
            Ok(value) => match value {
                EvalValue::Primitive(value) => fade!(value),
                EvalValue::Boxed(_) => todo!(),
                EvalValue::Volatile(_) => todo!(),
                EvalValue::GlobalRef(_) => todo!(),
                EvalValue::Undefined => fade!("undefined"),
            },
            Err(e) => Self {
                value: e.into(),
                associated_trace: None,
                kind: TraceTokenKind::Error,
            },
        }
    }
}

impl From<StackValueSnapshot> for TokenProps {
    fn from(value: StackValueSnapshot) -> Self {
        match value {
            StackValueSnapshot::Primitive(value) => value.into(),
            StackValueSnapshot::MutRef {
                value, owner, gen, ..
            } => fade!(format!("{:?}", value)),
        }
    }
}

impl From<VMResult<StackValueSnapshot>> for TokenProps {
    fn from(_: VMResult<StackValueSnapshot>) -> Self {
        todo!()
    }
}

impl From<VMResult<PrimitiveValue>> for TokenProps {
    fn from(result: VMResult<PrimitiveValue>) -> Self {
        match result {
            Ok(value) => value.into(),
            Err(e) => Self {
                value: e.into(),
                associated_trace: None,
                kind: TraceTokenKind::Error,
            },
        }
    }
}

impl From<PrimitiveValue> for TokenProps {
    fn from(value: PrimitiveValue) -> Self {
        fade!(value)
    }
}

impl From<InitKind> for TokenProps {
    fn from(init_kind: InitKind) -> Self {
        match init_kind {
            InitKind::Let => keyword!("let "),
            InitKind::Var => keyword!("var "),
            InitKind::Decl => panic!(),
        }
    }
}

// ts: string
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Label,
    Ident,
    Literal,
    Special,
    Scope,
    Fade,
    Error,
}

#[macro_export]
macro_rules! keyword {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            associated_trace: None,
        }
    }};
}

#[macro_export]
macro_rules! label {
    ($value:expr, $associated:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Label,
            value: $value.into(),
            spaces_before: None,
            associated: $associated,
            associated: vec![],
        }
    }};
}

#[macro_export]
macro_rules! ident {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            associated_trace: None,
        }
    }};
    ($value:expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            associated_trace: None,
        }
    }};
}

#[macro_export]
macro_rules! special {
    ($value: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            associated_trace: None,
        }
    }};

    ($value: expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! scope {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            associated_trace: None,
        }
    }};

    ($value:expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! fade {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            associated_trace: None,
        }
    }};
    ($value:expr, $associated:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            associated_trace: $associated,
        }
    }};
}

use vm::{EvalResult, EvalValue, InitKind, PrimitiveValue, StackValueSnapshot, VMResult};
