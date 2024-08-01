use atlas_core::{keyword, lexer, number, symbols};

lexer!();
number!(enable_f64: false, enable_i64: true);
symbols!(
    '$' => DollarSign,
    '#' => HashTag,
    '.' => Dot,
    ':' => Colon,
    ';' => SemiColon,
    '&' => Ampersand,
    '[' => LBracket,
    ']' => RBracket
);
keyword!(
    "section", "code", "start", "push", "call", "print", "hlt", "dup", "lt", "lte", "gt", "gte",
    "jmp", "jmpz", "jmpnz", "ret", "swap", "add", "sub", "mul", "div"
);
