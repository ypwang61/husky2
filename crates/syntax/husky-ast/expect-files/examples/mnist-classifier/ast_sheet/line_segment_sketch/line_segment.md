Ok(
    AstSheet {
        arena: Arena {
            data: [
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
                        10,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        1..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        3..3,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        3..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        5..5,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        5..6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        3..3,
                    ),
                },
                IfElseStmts {
                    if_stmt: 4,
                    elif_stmts: ArenaIdxRange(
                        5..5,
                    ),
                    else_stmt: Some(
                        6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        7..9,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                IfElseStmts {
                    if_stmt: 2,
                    elif_stmts: ArenaIdxRange(
                        3..3,
                    ),
                    else_stmt: Some(
                        9,
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
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Form,
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `mnist_classifier::line_segment_sketch::line_segment::displacement`,
                        ),
                    ),
                    ident: `displacement`,
                    is_generic: false,
                    body_kind: Block,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        10..13,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem(
                        Form,
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `mnist_classifier::line_segment_sketch::line_segment::dist_to_point`,
                        ),
                    ),
                    ident: `dist_to_point`,
                    is_generic: false,
                    body_kind: Block,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
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
                        13..17,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
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
                            `mnist_classifier::line_segment_sketch::line_segment::LineSegment`,
                        ),
                    ),
                    ident: `LineSegment`,
                    is_generic: false,
                    body_kind: Block,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            17..19,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `geom2d`,
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