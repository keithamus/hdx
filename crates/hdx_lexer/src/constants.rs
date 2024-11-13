use super::Token;

// EOF is a stand in for "Does not map"
pub const SINGLE_CHAR_TOKENS: &[Token; 128] = &[
	/*   0 */ Token::EOF,
	/*   1 */ Token::EOF,
	/*   2 */ Token::EOF,
	/*   3 */ Token::EOF,
	/*   4 */ Token::EOF,
	/*   5 */ Token::EOF,
	/*   6 */ Token::EOF,
	/*   7 */ Token::EOF,
	/*   8 */ Token::EOF,
	/*   9 */ Token::EOF,
	/*  10 */ Token::EOF,
	/*  11 */ Token::EOF,
	/*  12 */ Token::EOF,
	/*  13 */ Token::EOF,
	/*  14 */ Token::EOF,
	/*  15 */ Token::EOF,
	/*  16 */ Token::EOF,
	/*  17 */ Token::EOF,
	/*  18 */ Token::EOF,
	/*  19 */ Token::EOF,
	/*  20 */ Token::EOF,
	/*  21 */ Token::EOF,
	/*  22 */ Token::EOF,
	/*  23 */ Token::EOF,
	/*  24 */ Token::EOF,
	/*  25 */ Token::EOF,
	/*  26 */ Token::EOF,
	/*  27 */ Token::EOF,
	/*  28 */ Token::EOF,
	/*  29 */ Token::EOF,
	/*  30 */ Token::EOF,
	/*  31 */ Token::EOF,
	/*  32 */ Token::EOF,
	/*  33 */ Token::BANG,
	/*  34 */ Token::EOF,
	/*  35 */ Token::EOF, // # is for hashids, so cannot be added
	/*  36 */ Token::DOLLAR, // 0x24
	/*  37 */ Token::PERCENT, // 0x37
	/*  38 */ Token::AMPERSAND, // 0x38
	/*  39 */ Token::EOF,
	/*  40 */ Token::LEFT_PAREN, // 0x28
	/*  41 */ Token::RIGHT_PAREN, // 0x29
	/*  42 */ Token::ASTERISK, // 0x2A
	/*  43 */ Token::EOF, // + can be start of a number, so cannot be added
	/*  44 */ Token::COMMA, // 0x2C
	/*  45 */ Token::EOF, // - can be the start of an Ident, so cannot be added
	/*  46 */ Token::EOF, // . can be the start of an Ident, so cannot be added
	/*  47 */ Token::EOF, // / is the start of comments, so cannot be added
	/*  48 */ Token::EOF,
	/*  49 */ Token::EOF,
	/*  50 */ Token::EOF,
	/*  51 */ Token::EOF,
	/*  52 */ Token::EOF,
	/*  53 */ Token::EOF,
	/*  54 */ Token::EOF,
	/*  55 */ Token::EOF,
	/*  56 */ Token::EOF,
	/*  57 */ Token::EOF,
	/*  58 */ Token::COLON, // 0x3A
	/*  59 */ Token::SEMICOLON, // 0x3B
	/*  60 */ Token::EOF, // Less than can be the start of CDO, so cannot be added
	/*  61 */ Token::EQUALS, // 0x3D
	/*  62 */ Token::GREATER_THAN, // 0x3E
	/*  63 */ Token::QUESTION, // 0x3F
	/*  64 */ Token::EOF, // @ can be the start of AtKeyword, so cannot be added
	/*  65 */ Token::EOF,
	/*  66 */ Token::EOF,
	/*  67 */ Token::EOF,
	/*  68 */ Token::EOF,
	/*  69 */ Token::EOF,
	/*  70 */ Token::EOF,
	/*  71 */ Token::EOF,
	/*  72 */ Token::EOF,
	/*  73 */ Token::EOF,
	/*  74 */ Token::EOF,
	/*  75 */ Token::EOF,
	/*  76 */ Token::EOF,
	/*  77 */ Token::EOF,
	/*  78 */ Token::EOF,
	/*  79 */ Token::EOF,
	/*  80 */ Token::EOF,
	/*  81 */ Token::EOF,
	/*  82 */ Token::EOF,
	/*  83 */ Token::EOF,
	/*  84 */ Token::EOF,
	/*  85 */ Token::EOF,
	/*  86 */ Token::EOF,
	/*  87 */ Token::EOF,
	/*  88 */ Token::EOF,
	/*  89 */ Token::EOF,
	/*  90 */ Token::EOF,
	/*  91 */ Token::LEFT_SQUARE, // 0x5B
	/*  92 */ Token::EOF, // \ can be the start of an escape code, so cannot be added
	/*  93 */ Token::RIGHT_SQUARE, // 0x5D
	/*  94 */ Token::CARET, // 0x5E
	/*  95 */ Token::EOF, // _ can be the start of an ident, so cannot be added
	/*  96 */ Token::BACKTICK, // 0x60
	/*  97 */ Token::EOF,
	/*  98 */ Token::EOF,
	/*  99 */ Token::EOF,
	/* 100 */ Token::EOF,
	/* 101 */ Token::EOF,
	/* 102 */ Token::EOF,
	/* 103 */ Token::EOF,
	/* 104 */ Token::EOF,
	/* 105 */ Token::EOF,
	/* 106 */ Token::EOF,
	/* 107 */ Token::EOF,
	/* 108 */ Token::EOF,
	/* 109 */ Token::EOF,
	/* 110 */ Token::EOF,
	/* 111 */ Token::EOF,
	/* 112 */ Token::EOF,
	/* 113 */ Token::EOF,
	/* 114 */ Token::EOF,
	/* 115 */ Token::EOF,
	/* 116 */ Token::EOF,
	/* 117 */ Token::EOF,
	/* 118 */ Token::EOF,
	/* 119 */ Token::EOF,
	/* 120 */ Token::EOF,
	/* 121 */ Token::EOF,
	/* 122 */ Token::EOF,
	/* 123 */ Token::LEFT_CURLY, // 0x7B
	/* 124 */ Token::PIPE,
	/* 125 */ Token::RIGHT_CURLY, // 0x7D
	/* 126 */ Token::TILDE, // 0x7E
	/* 127 */ Token::EOF,
];
