Ok(
    TokenSheet {
        tokens: [
            Token {
                range: [1:1, 1:4),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [1:5, 1:8),
                kind: Keyword(
                    Use,
                ),
            },
            Token {
                range: [1:9, 1:13),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [1:13, 1:15),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [1:15, 1:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [1:20, 1:22),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [1:22, 1:23),
                kind: Special(
                    BinaryOpr(
                        PureClosed(
                            Mul,
                        ),
                    ),
                ),
            },
        ],
        group_starts: [
            0,
        ],
    },
)