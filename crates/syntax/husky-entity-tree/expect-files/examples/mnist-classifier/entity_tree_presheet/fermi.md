Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::fermi`,
        module_symbols: [
            ModuleItem {
                ident: `FermiMatchResult`,
                accessibility: PubicUnder(
                    `mnist_classifier::fermi`,
                ),
                ast_idx: 24,
                path: `mnist_classifier::fermi::FermiMatchResult`,
            },
            ModuleItem {
                ident: `fermi_match`,
                accessibility: PubicUnder(
                    `mnist_classifier::fermi`,
                ),
                ast_idx: 26,
                path: `mnist_classifier::fermi::fermi_match`,
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 23,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 1,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)