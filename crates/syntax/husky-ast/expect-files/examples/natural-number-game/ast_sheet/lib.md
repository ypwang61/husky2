Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem,
                    entity_path: None,
                    ident: `add`,
                    is_generic: false,
                    body_kind: EnumVariants,
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Type(
                            Inductive,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `natural_number_game::Nat`,
                        ),
                    ),
                    ident: `Nat`,
                    is_generic: false,
                    body_kind: EnumVariants,
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Impl {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..3,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: ExpectIdentifier(
                        Some(
                            [10:8, 10:9),
                        ),
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: ExcessiveIndent,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Type(
                            Structure,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `natural_number_game::OddNat`,
                        ),
                    ),
                    ident: `OddNat`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Type(
                            Structure,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `natural_number_game::EvenNat`,
                        ),
                    ),
                    ident: `EvenNat`,
                    is_generic: false,
                    body_kind: None,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            3..11,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)