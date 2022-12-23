Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                },
                Comment {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        2..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        4..12,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        13..13,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        13..14,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
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
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                IfElseStmts {
                    if_stmt: 12,
                    elif_stmts: ArenaIdxRange(
                        13..13,
                    ),
                    else_stmt: Some(
                        14,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    use_expr_idx: 8,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        15..20,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Form,
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `mnist_classifier::line_segment_sketch::convexity::is_convex`,
                        ),
                    ),
                    ident: `is_convex`,
                    is_generic: false,
                    body_kind: Block,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            20..24,
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
                All,
                ScopeResolution {
                    parent: `raw_contour`,
                    child: 3,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 4,
                },
                All,
                ScopeResolution {
                    parent: `geom2d`,
                    child: 6,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 7,
                },
            ],
        },
    },
)