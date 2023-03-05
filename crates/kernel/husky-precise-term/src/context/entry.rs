use husky_entity_path::{ModuleItemPath, TypePath};
use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct PreciseTermSymbolShowEntry {
    symbol: PreciseTermSymbol,
    show_kind: PreciseTermSymbolShowKind,
    idx: u8,
    /// number of lambdas using this symbol
    /// level 0 means this symbol is external
    level: u8,
    external_symbol_ident: Option<Identifier>,
}

impl PreciseTermSymbolShowEntry {
    pub(crate) fn show(
        &self,
        db: &dyn PreciseTermDb,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(external_symbol_ident) = self.external_symbol_ident && self.level == 0 {
            todo!()
        } else {
            match self.show_kind {
                PreciseTermSymbolShowKind::Lifetime => {
                    match self.idx {
                        0 => f.write_str("'a"),
                        1 => f.write_str("'b"),
                        2 => f.write_str("'c"),
                        3 => f.write_str("'d"),
                        4 => f.write_str("'e"),
                        5 => f.write_str("'f"),
                        idx => f.write_fmt(format_args!("'a{}", idx))
                    }
                },
                PreciseTermSymbolShowKind::Binding => {
                    match self.idx {
                        0 => f.write_str("'α"),
                        1 => f.write_str("'β"),
                        2 => f.write_str("'γ"),
                        3 => f.write_str("'δ"),
                        4 => f.write_str("'ϵ"),
                        5 => f.write_str("'ζ"),
                        6 => f.write_str("'η"),
                        idx => f.write_fmt(format_args!("'α{}", idx))
                    }
                },
                PreciseTermSymbolShowKind::Prop => {
                    match self.idx {
                        0 => f.write_str("p"),
                        1 => f.write_str("q"),
                        idx => f.write_fmt(format_args!("p{}", idx))
                    }
                },
                PreciseTermSymbolShowKind::Type => {
                    match self.idx {
                        0 => f.write_str("t"),
                        1 => f.write_str("s"),
                        idx => f.write_fmt(format_args!("t{}", idx))
                    }
                },
                PreciseTermSymbolShowKind::Kind => {
                    match self.idx {
                        0 => f.write_str("α"),
                        1 => f.write_str("β"),
                        2 => f.write_str("γ"),
                        3 => f.write_str("δ"),
                        4 => f.write_str("ϵ"),
                        5 => f.write_str("ζ"),
                        6 => f.write_str("η"),
                        idx => f.write_fmt(format_args!("α{}", idx))
                    }
                },
                PreciseTermSymbolShowKind::Other => {
                    match self.idx {
                        0 => f.write_str("a"),
                        1 => f.write_str("b"),
                        idx => f.write_fmt(format_args!("a{}", idx))
                    }
                }
            }
        }
    }
}

impl AsVecMapEntry for PreciseTermSymbolShowEntry {
    type K = PreciseTermSymbol;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.symbol
    }

    fn key_ref(&self) -> &Self::K {
        &self.symbol
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum PreciseTermSymbolShowKind {
    Lifetime,
    Binding,
    Prop,
    Type,
    Kind,
    Other,
}

impl PreciseTermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &dyn PreciseTermDb,
        symbol: PreciseTermSymbol,
        external_symbol_ident: Option<Identifier>,
    ) -> PreciseTermSymbolShowEntry {
        self.new_entry(db, symbol, 0, external_symbol_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &dyn PreciseTermDb,
        symbol: PreciseTermSymbol,
    ) -> PreciseTermSymbolShowEntry {
        self.new_entry(db, symbol, 1, None)
    }

    fn new_entry(
        &self,
        db: &dyn PreciseTermDb,
        symbol: PreciseTermSymbol,
        level: u8,
        external_symbol_ident: Option<Identifier>,
    ) -> PreciseTermSymbolShowEntry {
        let show_kind = symbol_show_kind(symbol, db);
        let idx = self.issue_idx(show_kind);
        PreciseTermSymbolShowEntry {
            symbol,
            show_kind,
            idx,
            level,
            external_symbol_ident,
        }
    }

    fn issue_idx(&self, show_kind: PreciseTermSymbolShowKind) -> u8 {
        let last_idx = self
            .entries
            .data()
            .iter()
            .rev()
            .find(|entry| entry.show_kind == show_kind)
            .map(|entry| entry.idx);
        match last_idx {
            Some(last_idx) => last_idx + 1,
            None => 0,
        }
    }

    // todo: put this into an internal table struct
    pub(super) fn enter_block(&mut self, db: &dyn PreciseTermDb, symbol: PreciseTermSymbol) {
        if let Some(entry) = self.entries.get_entry_mut(symbol) {
            entry.level += 1
        } else {
            let new_entry = self.new_internal_entry(db, symbol);
            self.entries.insert_new(new_entry).unwrap();
        }
    }

    pub(super) fn exit_block(&mut self, symbol: PreciseTermSymbol) {
        self.entries.get_entry_mut(symbol).unwrap().level -= 1
    }
}

fn symbol_show_kind(
    symbol: PreciseTermSymbol,
    db: &dyn PreciseTermDb,
) -> PreciseTermSymbolShowKind {
    match symbol.ty(db) {
        Ok(PreciseTerm::EntityPath(PreciseTermEntityPath::TypeOntology(ty)))
            if is_ty_path_lifetime_ty(db, ty) =>
        {
            PreciseTermSymbolShowKind::Lifetime
        }
        Ok(PreciseTerm::Category(cat)) if cat.universe().raw() == 0 => {
            PreciseTermSymbolShowKind::Prop
        }
        Ok(PreciseTerm::Category(cat)) if cat.universe().raw() == 1 => {
            PreciseTermSymbolShowKind::Type
        }
        Ok(PreciseTerm::Category(_)) => PreciseTermSymbolShowKind::Kind,
        _ => PreciseTermSymbolShowKind::Other,
    }
}

// this might be the most efficient way to do it
// only use this in this module
#[salsa::tracked(jar = PreciseTermJar)]
pub(crate) fn is_ty_path_lifetime_ty(db: &dyn PreciseTermDb, ty_path: TypePath) -> bool {
    let toolchain = ty_path.toolchain(db);
    let Ok(entity_path_menu) = db.entity_path_menu(toolchain) else {
        return false
    };
    ty_path == entity_path_menu.lifetime_ty_path()
}