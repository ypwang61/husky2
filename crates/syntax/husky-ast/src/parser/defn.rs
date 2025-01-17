use super::*;
use husky_entity_kind::{MajorItemConnectionKind, MajorItemKind, TypeKind};
use husky_entity_path::{
    path::{
        major_item::{
            connection::MajorItemConnection, form::MajorFormPath, trai::TraitPath, ty::TypePath,
        },
        submodule::SubmoduleItemPath,
        ty_variant::TypeVariantRegistry,
    },
    *,
};
use parsec::IsStreamParser;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn<C: IsAstChildren>(
        &mut self,
        token_verse_idx: TokenVerseIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenStreamState>,
    ) -> AstData {
        self.parse_defn_aux::<C>(token_verse_idx, visibility_expr, state)
            .unwrap_or_else(|error| AstData::Err {
                token_verse_idx,
                error,
            })
    }

    fn parse_defn_aux<C: IsAstChildren>(
        &mut self,
        token_verse_idx: TokenVerseIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenStreamState>,
    ) -> AstResult<AstData> {
        let aux_parser = BasicAuxAstParser::new(
            self.db,
            self.module_path,
            self.token_sheet
                .token_verse_token_stream(token_verse_idx, state),
        );
        let (item_kind, ident_token, is_generic, saved_stream_state) =
            aux_parser.parse_head::<C>()?;
        let ident = ident_token.ident();
        let block = match item_kind {
            EntityKind::Module => DefnBlock::Submodule {
                path: SubmoduleItemPath::new(
                    ModulePath::new_child(self.db, self.module_path, ident).map_err_or_none(
                        |error| OriginalAstError::SubmoduleFileNotFound { ident_token, error },
                    )?,
                    self.db,
                ),
            },
            EntityKind::MajorItem {
                module_item_kind,
                connection,
            } => {
                let connection = self.new_connection(ident, connection);
                match module_item_kind {
                    MajorItemKind::Type(ty_kind) => {
                        let path =
                            TypePath::new(self.module_path, ident, connection, ty_kind, self.db)
                                .into();
                        DefnBlock::Type {
                            path,
                            variants: match ty_kind {
                                TypeKind::Enum | TypeKind::Inductive => {
                                    Some(self.parse_expected_with_context(
                                        path,
                                        OriginalAstError::ExpectedTypeVariants,
                                    )?)
                                }
                                _ => None,
                            },
                        }
                    }
                    MajorItemKind::Form(form_kind) => DefnBlock::Form {
                        path: MajorFormPath::new(
                            self.module_path,
                            ident,
                            connection,
                            form_kind,
                            self.db,
                        )
                        .into(),
                        body: self.try_parse_option()?, // todo: check that this is coherent with decl
                    },
                    MajorItemKind::Trait => DefnBlock::Trait {
                        path: TraitPath::new(self.module_path, ident, connection, self.db).into(),
                        items: self.try_parse_option()?,
                    },
                }
            }
            EntityKind::AssocItem { .. } => DefnBlock::AssocItem {
                body: self.try_parse_option()?,
            },
            EntityKind::TypeVariant | EntityKind::ImplBlock | EntityKind::Attr => {
                unreachable!("it should be guaranteed by callers")
            }
            EntityKind::Script => todo!(),
        };
        Ok(AstData::Identifiable {
            visibility_expr,
            ident_token,
            is_generic,
            token_verse_idx,
            block,
            item_kind,
            saved_stream_state,
        })
    }

    #[inline(always)]
    fn new_connection(
        &mut self,
        ident: Ident,
        kind: MajorItemConnectionKind,
    ) -> MajorItemConnection {
        match kind {
            MajorItemConnectionKind::Connected => MajorItemConnection::Connected,
            MajorItemConnectionKind::Disconnected => {
                MajorItemConnection::Disconnected(self.disambiguator_registry.issue_new(ident))
            }
        }
    }
    /// parse variants of enum or inductive types
    #[inline(always)]
    pub(crate) fn parse_ty_variants(&mut self, path: TypePath) -> AstIdxRange {
        let mut ty_variants = vec![];
        let registry = &mut TypeVariantRegistry::new_u8();
        loop {
            let state = self.token_verse_iter.state();
            let Some((token_verse_idx, _)) = self.token_verse_iter.next() else {
                break;
            };
            // todo: change the api of `self.token_verses.next()`
            // it should directly return a token stream
            let mut aux_parser = BasicAuxAstParser::new(
                self.db,
                self.module_path,
                self.token_sheet
                    .token_verse_token_stream(token_verse_idx, None),
            );
            let Ok(Some(vertical_token)) = aux_parser.try_parse_option::<VerticalToken>() else {
                self.token_verse_iter.rollback(state);
                break;
            };
            ty_variants.push(
                match aux_parser.try_parse_expected::<IdentToken, _>(
                    OriginalAstError::ExpectedIdentForTypeVariant,
                ) {
                    Ok(ident_token) => AstData::TypeVariant {
                        token_verse_idx,
                        variant_path: TypeVariantPath::new(
                            path,
                            ident_token.ident(),
                            self.db,
                            registry,
                        ),
                        vertical_token,
                        ident_token,
                        saved_stream_state: aux_parser.state(),
                    },
                    Err(error) => AstData::Err {
                        token_verse_idx,
                        error,
                    },
                },
            )
        }
        self.alloc_asts(ty_variants)
    }
}

impl<'a> BasicAuxAstParser<'a> {
    fn parse_head<C: IsAstChildren>(
        mut self,
    ) -> AstResult<(EntityKind, IdentToken, bool, TokenStreamState)> {
        let item_keyword_group =
            self.try_parse_expected(OriginalAstError::ExpectedEntityKeywordGroup)?;
        let ident: IdentToken = self.try_parse_expected(OriginalAstError::ExpectedIdent)?;
        let is_generic = self.parse_is_generic();
        let item_kind = C::determine_item_kind(item_keyword_group)?;
        Ok((
            item_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }
}
