#[cfg(test)]
mod tests;

use husky_token::TokenIdxRange;
use husky_vfs::{ModulePath, VfsDb};
use std::cmp::Ordering;
use with_db::{PartialOrdWithDb, WithDb};

/// Accessibility is greater if it can be accessed from more places
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                  // everyone can access it
    PublicUnder(ModulePath), // everyone under a path can access it
    Private,                 // only self
    Disconnected {
        module_path: ModulePath,
        file_accessibility: FileAccessibility,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileAccessibility {
    token_idx_range: TokenIdxRange,
}

impl PartialOrdWithDb<dyn VfsDb + '_> for Accessibility {
    fn partial_cmp_with_db(&self, db: &dyn VfsDb, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Accessibility::Public, Accessibility::Public) => Some(Ordering::Equal),
            (Accessibility::Public, _) => Some(Ordering::Greater),
            (Accessibility::PublicUnder(_), Accessibility::Public) => Some(Ordering::Less),
            (
                Accessibility::PublicUnder(module_path0),
                Accessibility::PublicUnder(module_path1),
            ) => module_path0.partial_cmp_with_db(db, module_path1),
            (
                Accessibility::PublicUnder(_),
                Accessibility::Private | Accessibility::Disconnected { .. },
            ) => Some(Ordering::Greater),
            (Accessibility::Private, Accessibility::Public) => {
                todo!()
            }
            (Accessibility::Private, Accessibility::PublicUnder(_)) => todo!(),
            (Accessibility::Private, Accessibility::Private) => Some(Ordering::Equal),
            (Accessibility::Private, Accessibility::Disconnected { .. }) => Some(Ordering::Greater),
            (Accessibility::Disconnected { .. }, _) => todo!(),
        }
    }
}

impl<Db: VfsDb> PartialOrdWithDb<Db> for Accessibility {
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<std::cmp::Ordering> {
        self.partial_cmp_with_db(db as &dyn VfsDb, other)
    }
}

impl Accessibility {
    pub fn is_accessible_from(self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        match self {
            Accessibility::Public => true,
            Accessibility::PublicUnder(parent_module) => module_path.starts_with(db, parent_module),
            Accessibility::Private => todo!(),
            Accessibility::Disconnected { .. } => todo!(),
        }
    }
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Accessibility::Public => f.write_str("Public"),
            Accessibility::PublicUnder(module_path) => f
                .debug_tuple("PubicUnder")
                .field(&module_path.debug_with(db, include_all_fields))
                .finish(),
            Accessibility::Private => f.write_str("Private"),
            Accessibility::Disconnected {
                module_path,
                file_accessibility,
            } => f
                .debug_struct("Disconnected")
                .field(
                    "module_path",
                    &module_path.debug_with(db, include_all_fields),
                )
                .field("file_accessibility", &file_accessibility)
                .finish(),
        }
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}