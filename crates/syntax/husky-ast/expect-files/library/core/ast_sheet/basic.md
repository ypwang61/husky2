Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Type(
                            Form,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::basic::bool`,
                        ),
                    ),
                    ident: `bool`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Type(
                            Structure,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::basic::Trait`,
                        ),
                    ),
                    ident: `Trait`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Type(
                            Structure,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::basic::Module`,
                        ),
                    ),
                    ident: `Module`,
                    is_generic: false,
                    body_kind: None,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)