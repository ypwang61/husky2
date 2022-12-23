Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: ExpectIdentifier(
                        Some(
                            [7:11, 7:12),
                        ),
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: ExcessiveIndent,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..4,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Type(
                            Struct,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`,
                        ),
                    ),
                    ident: `ConvexCompoent`,
                    is_generic: false,
                    body_kind: Block,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            4..6,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 1,
                },
            ],
        },
    },
)