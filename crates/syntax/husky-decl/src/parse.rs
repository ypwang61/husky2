use crate::*;
use husky_ast::*;
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::*;
use husky_entity_tree::*;
use husky_opn_syntax::BinaryOpr;
use husky_print_utils::p;
use husky_token::*;
use husky_vfs::CratePath;
use parsec::*;
use salsa::DebugWithDb;
use vec_like::VecPairMap;

pub(crate) fn module_item_decl(db: &dyn DeclDb, path: ModuleItemPath) -> DeclResult<Decl> {
    Ok(match path {
        ModuleItemPath::Type(path) => type_decl(db, path)?.into(),
        ModuleItemPath::Trait(path) => trait_decl(db, path)?.into(),
        ModuleItemPath::Form(path) => form_decl(db, path)?.into(),
    })
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn type_decl(db: &dyn DeclDb, path: TypePath) -> DeclResult<TypeDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_ty_decl(path)
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn form_decl(db: &dyn DeclDb, path: FormPath) -> DeclResult<FormDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_form_decl(path)
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trait_decl(db: &dyn DeclDb, path: TraitPath) -> DeclResult<TraitDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_trai_decl(path)
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn impl_block_decl(db: &dyn DeclDb, impl_block: ImplBlock) -> DeclResult<ImplBlockDecl> {
    let parser = DeclParser::new(db, impl_block.module_path(db))?;
    parser.parse_impl_block_decl(impl_block)
}

pub(crate) struct DeclParser<'a> {
    db: &'a dyn DeclDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    module_entity_tree: &'a EntityTreeModuleSheet,
    entity_tree_crate_bundle: &'a EntityTreeCrateBundle,
}

impl<'a> DeclParser<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path)?,
            ast_sheet: db.ast_sheet(path)?,
            module_entity_tree: db.entity_tree_sheet(path)?,
            entity_tree_crate_bundle: db.entity_tree_crate_bundle(path.crate_path(db))?,
        })
    }

    fn parse_ty_decl(&self, path: TypePath) -> DeclResult<TypeDecl> {
        let ident = path.ident(self.db);
        let module_item = self
            .module_entity_tree
            .module_specific_symbols()
            .get_entry(ident)
            .unwrap()
            .module_item()
            .unwrap();

        let ast_idx: AstIdx = module_item.ast_idx();
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                is_generic,
                body_kind,
                saved_stream_state,
                ..
            } => self.parse_ty_decl_aux(
                ast_idx,
                path.ty_kind(self.db),
                path,
                entity_kind,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }
    fn parse_ty_decl_aux(
        &self,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        path: TypePath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        match type_kind {
            TypeKind::Enum => {
                self.parse_enum_type_decl(ast_idx, path, token_group_idx, body, saved_stream_state)
            }
            TypeKind::Inductive => self.parse_inductive_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Record => todo!(),
            TypeKind::Struct => self.parse_struct_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Structure => self.parse_structure_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Foreign => self.parse_foreign_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
        }
    }

    fn parse_enum_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(EnumTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters).into())
    }

    fn parse_trai_decl(&self, path: TraitPath) -> DeclResult<TraitDecl> {
        let ident = path.ident(self.db);
        let module_item = self
            .module_entity_tree
            .module_specific_symbols()
            .get_entry(ident)
            .unwrap()
            .module_item()
            .unwrap();
        let ast_idx: AstIdx = module_item.ast_idx();
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                is_generic,
                body_kind,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, path, token_group_idx, body, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: TraitPath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TraitDecl> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(TraitDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        ))
    }

    fn parse_inductive_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(
            InductiveTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        )
    }

    fn parse_struct_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        if let Some(lcurl) = ctx.parse::<LeftCurlyBraceToken>()? {
            let (fields, separators) = parse_separated_list(&mut ctx)?;
            let rcurl: RightCurlyBraceToken = ctx.parse_expected()?;
            Ok(PropsStructTypeDecl::new(
                self.db,
                path,
                ast_idx,
                parser.finish(),
                implicit_parameters,
                lcurl,
                fields,
                separators,
                rcurl,
            )
            .into())
        } else if let Some(lbox) = ctx.parse::<LeftBoxBracketToken>()? {
            todo!()
        } else {
            Err(DeclError::ExpectLCurlOrLParOrSemicolon(ctx.save_state()))
        }
    }

    fn expr_parser(&self, expr_path: DeclExprPath) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            expr_path.into(),
            self.token_sheet_data,
            SymbolContextMut::new(self.module_symbol_context, None),
        )
    }

    fn parse_structure_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(
            StructureTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        )
    }

    // get declaration from tokens
    fn parse_foreign_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(AlienTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters).into())
    }

    fn parse_form_decl(&self, path: FormPath) -> DeclResult<FormDecl> {
        let ident = path.ident(self.db);
        let module_item = self
            .module_entity_tree
            .module_specific_symbols()
            .get_entry(ident)
            .unwrap()
            .module_item()
            .unwrap();
        let ast_idx: AstIdx = module_item.ast_idx();
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                is_generic,
                body_kind,
                saved_stream_state,
                ..
            } => self.parse_form_decl_aux(
                ast_idx,
                path,
                entity_kind,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_form_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: FormPath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<FormDecl, DeclError> {
        match path.form_kind(self.db) {
            FormKind::Feature => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Function => {
                self.parse_function_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Value => todo!(),
            FormKind::TypeAlias => todo!(),
        }
    }

    fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        Ok(FeatureDecl::new(self.db, path, ast_idx, parser.finish()).into())
    }

    fn parse_function_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(DeclExprPath::Entity(path.into()));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameter_decl_list = ctx.parse()?;
        let Some(parameter_decl_list) = ctx.parse()? else {
            todo!()
        };
        Ok(FunctionDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
        )
        .into())
    }

    fn parse_impl_block_decl(&self, impl_block: ImplBlock) -> DeclResult<ImplBlockDecl> {
        let ast_idx = impl_block.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Impl {
                token_group_idx,
                body,
            } => match impl_block.kind(self.db) {
                ImplBlockKind::Type { ty } => Ok(self
                    .parse_ty_impl_block_decl(ast_idx, impl_block, token_group_idx)?
                    .into()),
                ImplBlockKind::TypeAsTrait { ty, trai } => todo!(),
                ImplBlockKind::Err => Err(DeclError::ImplBlockErr),
            },
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_block_decl(
        &self,
        ast_idx: AstIdx,
        impl_block: ImplBlock,
        token_group_idx: TokenGroupIdx,
    ) -> DeclResult<TypeImplBlockDecl> {
        let mut parser = self.expr_parser(DeclExprPath::ImplBlock(impl_block));
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, None),
        );
        let impl_token = ctx.parse().unwrap().unwrap();
        let ty = ctx.parse_expr(ExprParseEnvironment::None).unwrap();
        let eol_colon = ctx.parse_expected();
        Ok(TypeImplBlockDecl::new(
            self.db,
            ast_idx,
            impl_block,
            impl_token,
            ty,
            eol_colon,
            parser.finish(),
        ))
    }
}