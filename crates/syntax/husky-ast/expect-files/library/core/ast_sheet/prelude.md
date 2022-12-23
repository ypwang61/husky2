Ok(
    AstSheet {
        arena: Arena {
            data: [
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 6,
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
                                value: 6,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..2,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `basic`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 1,
                },
                All,
                ScopeResolution {
                    parent: `num`,
                    child: 3,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 4,
                },
            ],
        },
    },
)