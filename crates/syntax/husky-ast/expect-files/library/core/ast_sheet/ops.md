Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Type(
                            Form,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::ops::Output`,
                        ),
                    ),
                    ident: `Output`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem,
                    entity_path: Some(
                        AssociatedItem(
                            AssociatedItemPath {
                                parent_path: `core::ops::Add`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ident: `add`,
                    is_generic: false,
                    body_kind: None,
                },
                Decor {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Trait,
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::ops::Add`,
                        ),
                    ),
                    ident: `Add`,
                    is_generic: true,
                    body_kind: Block,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..4,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)