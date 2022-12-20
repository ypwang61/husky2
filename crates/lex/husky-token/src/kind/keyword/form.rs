use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
    Def,
    Func,
    Proc,
    Fn,
    Function,
    Theorem,
    Lemma,
    Proposition,
}

impl FormKeyword {
    pub const fn as_str(self) -> &'static str {
        match self {
            FormKeyword::Proc => "proc",
            FormKeyword::Func => "func",
            FormKeyword::Def => "def",
            FormKeyword::Fn => "fn",
            FormKeyword::Function => "function",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
        }
    }

    pub fn is_lazy(self) -> bool {
        match self {
            FormKeyword::Def => true,
            _ => false,
        }
    }
}

impl Deref for FormKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl const From<FormKeyword> for Keyword {
    fn from(kw: FormKeyword) -> Self {
        Keyword::Paradigm(kw)
    }
}

impl const From<FormKeyword> for TokenKind {
    fn from(kw: FormKeyword) -> Self {
        TokenKind::Keyword(kw.into())
    }
}