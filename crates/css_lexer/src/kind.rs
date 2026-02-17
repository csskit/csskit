use core::fmt;

use crate::KindSet;

/// Kind represents the token "Type", categorised mostly by the token types within the CSS Syntax spec.
///
/// Maintaining parity with the spec makes it easier to reason about logica round the parser, despite it being possible to
/// group a bunch of these tokens into a single "delimiter" token.
///
/// Importantly, `Kind` is represented as `u8` and must only use the 5 low bits, because the upper 3 bits get used to
/// house details about each kind, that a token would be interested in learning about.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
	// Trivias (mask as 0b0_00XX)
	/// Represents the [&lt;eof-token>][1] defined in CSS. While CSS stipulates that this token is never produced by a
	/// tokenizer, this [Lexer][crate::Lexer] _will_ produce [&lt;eof-token>s][1] if the underlying source has been
	/// fully consumed.
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-eof-token
	Eof = 0b0000,

	/// Represents the [&lt;whitespace-token>][1] defined in CSS.
	///
	/// ```md
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <whitespace-token>
	///  │├─╭─ <whitespace> ─╮─┤│
	///     ╰────────────────╯
	/// ```
	///
	/// While CSS stipulates that this token represents collapsed whitespace, it is possible for [Lexer][crate::Lexer]
	/// to produce multiple consecutive [Kind::Whitespace] tokens if the
	/// [Feature::SeparateWhitespace][crate::Feature::SeparateWhitespace] runtime feature is enabled. In this case,
	/// `<whitespace-token>` becomes:
	///
	/// ```md
	/// <whitespace-token>
	///  │├──╮─╭─ " " ───────╮─╭──┤│
	///      │ ╰─────────────╯ │
	///      ├─╭─ "\t" ──────╮─┤
	///      │ ╰─────────────╯ │
	///      ╰─╭─ <newline> ─╮─╯
	///        ╰─────────────╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#whitespace-token-diagram
	#[default]
	Whitespace = 0b0001,

	/// Represents the [&lt;comment>][1] defined in CSS. While CSS stipulates comment tokens are not produced during
	/// tokenization, they are for this [Lexer][crate::Lexer] as they're needed in order to preserve them.
	///
	/// ```md
	/// <comment>
	///            ╭──────────────────────────────────────────╮
	///  │├─ "/*" ─╯-╭─ (anything but "*" followed by "/") ─╮─╰─ "*/" ─┤│
	///              ╰──────────────────────────────────────╯
	/// ```
	///
	/// It is possible for [Lexer][crate::Lexer] to produce [Kind::Whitespace] tokens that begin `//` if the
	/// [Feature::SingleLineComments][crate::Feature::SingleLineComments] runtime feature is enabled. In this mode,
	/// `<comment>` becomes:
	///
	/// ```md
	/// <comment>
	///               ╭──────────────────────────────────────────╮
	///  │├──╮─ "/*" ─╯-╭─ (anything but "*" followed by "/") ─╮─╰─ "*/" ─╭─┤│
	///      │          ╰──────────────────────────────────────╯          │
	///      │              ╭───────────────────────────╮                 │
	///      ╰─ "//" ───────╯-╭─ (anything but "\n") ─╮─╰─ "\n" ──────────╯
	///                       ╰───────────────────────╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#comment-diagram
	Comment = 0b0010,

	/// Represents both the [&lt;cdc-token>][1] and [&lt;cdo-token>][2]s defined in CSS. While CSS separates these tokens,
	/// they're only useful representations at the top-level stylesheet, anywhere else they represent a parse error, and
	/// it's a little pointless to define two tokens types for what amounts to a parse error.
	///
	/// ```md
	/// <cdo-token>
	///  │├─ "<!--" ─┤│
	///
	/// <cdc-token>
	///  │├─ "-->" ─┤│
	///
	/// <cdc-or-cdo-token> (Not part of the CSS specification)
	///  │├──╮─ <cdo-token> ─╭──┤│
	///      ╰─ <crc-token> ─╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#CDC-token-diagram
	/// [2]: https://drafts.csswg.org/css-syntax/#CDO-token-diagram
	CdcOrCdo = 0b0011,

	// Numerics (mask as 0b0_010X)
	/// Represents the [&lt;number-token>][1].
	///
	/// ```md
	///
	/// <number-token>
	///     ╭─ "+" ─╮
	///  │├─├───────┤───╭─ [digit] ─╮─ "." ─╭─ [digit] ─╮──╭───╮──────────────────────────────────╭──┤│
	///     ╰─ "-" ─╯ │ ╰───────────╯       ╰───────────╯  │   │         ╭─ "+" ─╮                │
	///               ├───────── ╭─ [digit] ─╮─────────────┤   ├─ "e" ─╭─├───────┤──╭─ [digit] ─╮─╯
	///               │          ╰───────────╯             │   ╰─ "E" ─╯ ╰─ "-" ─╯  ╰───────────╯
	///               ╰──── "." ─╭─ [digit] ─╮─────────────╯
	///                          ╰───────────╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#number-token-diagram
	Number = 0b0100,

	/// Represents the [&lt;dimension-token>][1].
	///
	/// Here we deviate from the spec slightly, which has both [&lt;dimension-token>][1] and [&lt;percentage-token>][2].
	/// `<percentage-token>` represents a dimension with a `%` symbol, but having this as a separate token results in more
	/// work in the parser for little gain in the Lexer. So instead this lexer does not have a `<percentage-token>` and
	/// instead folds the grammar for it inside of `<dimension-token>`.
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <ident-token>
	///     ╭───────────────── "--" ─────────────────────╮  ╭───────────────────────────────────────────╮
	///  │├─╯─╮───────╭─╮─ [a-z, A-Z, "_", non-ASCII] ─╭─╰──╯─╭─╮─ [a-z, A-Z, 0-9, "_", non-ASCII] ─╭─╮─╰──┤│
	///       ╰─ "-" ─╯ ╰──────── <escape> ────────────╯      │ ╰──────────── <escape> ─────────────╯ │
	///                                                       ╰───────────────────────────────────────╯
	///
	/// <number-token>
	///     ╭─ "+" ─╮
	///  │├─├───────┤─╮─╭─ [digit] ─╮─ "." ─╭─ [digit] ─╮──╭───╮──────────────────────────────────╭──┤│
	///     ╰─ "-" ─╯ │ ╰───────────╯       ╰───────────╯  │   │         ╭─ "+" ─╮                │
	///               ├───────── ╭─ [digit] ─╮─────────────┤   ├─ "e" ─╭─├───────┤──╭─ [digit] ─╮─╯
	///               │          ╰───────────╯             │   ╰─ "E" ─╯ ╰─ "-" ─╯  ╰───────────╯
	///               ╰──── "." ─╭─ [digit] ─╮─────────────╯
	///                          ╰───────────╯
	///
	/// <dimension-token>
	///  │├─ <number-token> ─ <ident-token> ─┤│
	///
	/// ```
	///
	/// ```md
	///
	/// <dimension-token> // Refined for this lexer, not true to the standard.
	///  │├─ <number-token> ─╮─ <ident-token> ─╭──┤│
	///                      ╰────── "%" ──────╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#dimension-token-diagram
	/// [2]: https://drafts.csswg.org/css-syntax/#percentage-token-diagram
	Dimension = 0b0101,

	// Errors (mask as 0b0_011X)
	/// Represents the [&lt;bad-string-token>][1]. This token is a failure to fully lex the [&lt;string-token>][2].
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-bad-string-token
	/// [2]: https://drafts.csswg.org/css-syntax/#typedef-string-token
	BadString = 0b0110,

	/// Represents the [&lt;bad-url-token>][1]. This token is a failure to fully lex the [&lt;url-token>][2].
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-bad-url-token
	/// [2]: https://drafts.csswg.org/css-syntax/#typedef-url-token
	BadUrl = 0b0111,

	// Variable length Ident-like Tokens (mask: 0b0_1XXX)
	/// Represents the [&lt;ident-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ─────╭──┤│
	///      ├─ "\t" ────┤
	///      ╰─ newline ─╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <ident-token>
	///     ╭───────────────── "--" ─────────────────────╮  ╭───────────────────────────────────────────╮
	///  │├─╯─╮───────╭─╮─ [a-z, A-Z, "_", non-ASCII] ─╭─╰──╯─╭─╮─ [a-z, A-Z, 0-9, "_", non-ASCII] ─╭─╮─╰──┤│
	///       ╰─ "-" ─╯ ╰──────── <escape> ────────────╯      │ ╰──────────── <escape> ─────────────╯ │
	///                                                       ╰───────────────────────────────────────╯
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#ident-token-diagram
	Ident = 0b1000,

	/// Represents the [&lt;function-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <ident-token>
	///     ╭───────────────── "--" ─────────────────────╮  ╭───────────────────────────────────────────╮
	///  │├─╯─╮───────╭─╮─ [a-z, A-Z, "_", non-ASCII] ─╭─╰──╯─╭─╮─ [a-z, A-Z, 0-9, "_", non-ASCII] ─╭─╮─╰──┤│
	///       ╰─ "-" ─╯ ╰──────── <escape> ────────────╯      │ ╰──────────── <escape> ─────────────╯ │
	///                                                       ╰───────────────────────────────────────╯
	///
	/// <function-token>
	///  │├─ <ident-token> ─ "(" ─┤│
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#function-token-diagram
	Function = 0b1001,

	/// Represents the [&lt;at-keyword-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <ident-token>
	///     ╭───────────────── "--" ─────────────────────╮  ╭───────────────────────────────────────────╮
	///  │├─╯─╮───────╭─╮─ [a-z, A-Z, "_", non-ASCII] ─╭─╰──╯─╭─╮─ [a-z, A-Z, 0-9, "_", non-ASCII] ─╭─╮─╰──┤│
	///       ╰─ "-" ─╯ ╰──────── <escape> ────────────╯      │ ╰──────────── <escape> ─────────────╯ │
	///                                                       ╰───────────────────────────────────────╯
	///
	/// <at-keyword-token>
	///  │├─ "@" ─ <ident-token> ─┤│
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#hash-token-diagram
	AtKeyword = 0b1010,

	/// Represents the [&lt;hash-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <hash-token>
	///  │├─ "#" ──╭─╮─ [a-z, A-Z, 0-9, "_", "-", non-ASCII] ─╭─╮─┤│
	///            │ ╰─────────────── <escape> ───────────────╯ │
	///            ╰────────────────────────────────────────────╯
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	Hash = 0b1011,

	/// Represents the [&lt;string-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <string-token>
	///             ╭───────────────────────────────────╮
	///  │├─╮─ """ ─╯─╭─╮─ [not """, "\", newline] ─╭─╮─╰── """ ─╭─┤│
	///     │         │ ├──────── <escape> ─────────┤ │          │
	///     │         │ ╰───── "\" ─ <newline> ─────╯ │          │
	///     │         ╰───────────────────────────────╯          │
	///     │       ╭───────────────────────────────────╮        │
	///     ╰─ "'" ─╯─╭─╮─ [not """, "\", newline] ─╭─╮─╰── "'" ─╯
	///               │ ├──────── <escape> ─────────┤ │
	///               │ ╰───── "\" ─ <newline> ─────╯ │
	///               ╰───────────────────────────────╯
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#string-token-diagram
	String = 0b1100,

	/// Represents the [&lt;url-token>][1].
	///
	/// ```md
	///
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <whitespace-token>
	///  │├─╭─ <whitespace> ─╮─┤│
	///     ╰────────────────╯
	///
	/// <ws*>
	///     ╭──────────────────────────╮
	///  │├─╯─╭─ <whitespace-token> ─╮─╰─┤│
	///       ╰──────────────────────╯
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	///
	/// <escape>
	///  │├─ "\" ─╮───── [not <newline> or <hexdigit>] ───╭─┤│
	///           ╰─╭── <hexdigit> ─╮──╮────────────────╭─╯
	///             ╰─ (1-6 times) ─╯  ╰─ <whitespace> ─╯
	///
	/// <ident-token>
	///     ╭───────────────── "--" ─────────────────────╮  ╭───────────────────────────────────────────╮
	///  │├─╯─╮───────╭─╮─ [a-z, A-Z, "_", non-ASCII] ─╭─╰──╯─╭─╮─ [a-z, A-Z, 0-9, "_", non-ASCII] ─╭─╮─╰──┤│
	///       ╰─ "-" ─╯ ╰──────── <escape> ────────────╯      │ ╰──────────── <escape> ─────────────╯ │
	///                                                       ╰───────────────────────────────────────╯
	///
	/// <url-token>
	///                                         ╭───────────────────────────────────────────────────────────────────╮
	///  │├─ <ident-token "url"> ─ "(" ─ <ws*> ─╯─╭─╮─ [not """ "'" "(" ")" "\" <whitespace> or non-printable] ─╭─╮─╰─ <ws*> ─ ")" ─┤│
	///                                           │ ╰──────────────────────── <escape> ─────────────────────────╯ │
	///                                           ╰───────────────────────────────────────────────────────────────╯
	///
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#url-token-diagram
	Url = 0b1101,

	/// Represents the [&lt;unicode-range-token>][1]. This token is only produced when the
	/// [Feature::UnicodeRange][crate::Feature::UnicodeRange] feature is enabled.
	///
	/// ```md
	///
	/// <hexdigit>
	///  │├─ [ 0-9, A-F, a-f ] ─┤│
	///
	/// <unicode-range-token>
	///  │├─╮─ 'U' ─╭─ '+' ─╭──────────────────╭── <hexdigit> ─╮──────────────────╭─┤│
	///     ╰─ 'u' ─╯       │                  ╰─ (1-6 times) ─╯                  │
	///                     │ ╭───────────────────╮                               │
	///                     ├─╯─╭── <hexdigit> ─╮─╰─╭───────────── ? ───────────╮─┤
	///                     │   ╰─ (1-5 times) ─╯   ╰─ (1 to (6 digits) times) ─╯ │
	///                     │                                                     │
	///                     ╰────╭── <hexdigit> ─╮── '-' ──╭── <hexdigit> ─╮──────╯
	///                          ╰─ (1-5 times) ─╯         ╰─ (1-5 times) ─╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#unicode-range-token-diagram
	UnicodeRange = 0b1110,

	// Single character Tokens (mask 0b1_XXXX)
	/// Represents the [&lt;delim-token>][1]. The `<delim-token>` has a value composed of a single code point.
	///
	/// ```md
	/// <delim-token>
	///  │├─ [codepoint] ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-delim-token
	Delim = 0b1_0000,

	/// Represents the [&lt;colon-token>][1].
	///
	/// ```md
	/// <colon-token>
	///  │├─ ":" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-colon-token
	Colon = 0b1_0001,

	/// Represents the [&lt;semicolon-token>][1].
	///
	/// ```md
	/// <semicolon-token>
	///  │├─ ";" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-semicolon-token
	Semicolon = 0b1_0010,

	/// Represents the [&lt;comma-token>][1].
	///
	/// ```md
	/// <comma-token>
	///  │├─ "," ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-comma-token
	Comma = 0b1_0011,

	/// Represents the [&lt;\[-token>][1].
	///
	/// ```md
	/// <[-token>
	///  │├─ "[" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-square
	LeftSquare = 0b1_0100,

	/// Represents the [&lt;\]-token>][1].
	///
	/// ```md
	/// <]-token>
	///  │├─ "]" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-square
	RightSquare = 0b1_0101,

	/// Represents the [&lt;(-token>][1].
	///
	/// ```md
	/// <(-token>
	///  │├─ "(" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-paren
	LeftParen = 0b1_0110,

	/// Represents the [&lt;)-token>][1].
	///
	/// ```md
	/// <)-token>
	///  │├─ ")" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-paren
	RightParen = 0b1_0111,

	/// Represents the [&lt;{-token>][1].
	///
	/// ```md
	/// <{-token>
	///  │├─ "{" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-curly
	LeftCurly = 0b1_1000,

	/// Represents the [&lt;}-token>][1].
	///
	/// ```md
	/// <}-token>
	///  │├─ "}" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-curly
	RightCurly = 0b1_1001,
}

impl Kind {
	pub(crate) const fn from_bits(bits: u8) -> Self {
		match bits {
			0b0001 => Self::Whitespace,
			0b0010 => Self::Comment,
			0b0011 => Self::CdcOrCdo,
			0b0100 => Self::Number,
			0b0101 => Self::Dimension,
			0b0110 => Self::BadString,
			0b0111 => Self::BadUrl,
			0b1000 => Self::Ident,
			0b1001 => Self::Function,
			0b1010 => Self::AtKeyword,
			0b1011 => Self::Hash,
			0b1100 => Self::String,
			0b1101 => Self::Url,
			0b1110 => Self::UnicodeRange,
			0b1_0000 => Self::Delim,
			0b1_0001 => Self::Colon,
			0b1_0010 => Self::Semicolon,
			0b1_0011 => Self::Comma,
			0b1_0100 => Self::LeftSquare,
			0b1_0101 => Self::RightSquare,
			0b1_0110 => Self::LeftParen,
			0b1_0111 => Self::RightParen,
			0b1_1000 => Self::LeftCurly,
			0b1_1001 => Self::RightCurly,
			_ => Self::Eof,
		}
	}

	#[doc(hidden)]
	pub const fn as_str(&self) -> &str {
		match *self {
			Kind::Eof => "Eof",
			Kind::Whitespace => "Whitespace",
			Kind::Comment => "Comment",
			Kind::CdcOrCdo => "CdcOrCdo",
			Kind::Number => "Number",
			Kind::Dimension => "Dimension",
			Kind::BadString => "BadString",
			Kind::BadUrl => "BadUrl",
			Kind::Ident => "Ident",
			Kind::Function => "Function",
			Kind::AtKeyword => "AtKeyword",
			Kind::Hash => "Hash",
			Kind::String => "String",
			Kind::Url => "Url",
			Kind::UnicodeRange => "UnicodeRange",
			Kind::Delim => "Delim",
			Kind::Colon => "Colon",
			Kind::Semicolon => "Semicolon",
			Kind::Comma => "Comma",
			Kind::LeftSquare => "LeftSquare",
			Kind::RightSquare => "RightSquare",
			Kind::LeftParen => "LeftParen",
			Kind::RightParen => "RightParen",
			Kind::LeftCurly => "LeftCurly",
			Kind::RightCurly => "RightCurly",
		}
	}
}

impl fmt::Debug for Kind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Kind::{}", self.as_str())
	}
}

impl fmt::Display for Kind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Kind::{}", self.as_str())
	}
}

impl PartialEq<KindSet> for Kind {
	fn eq(&self, other: &KindSet) -> bool {
		other.contains_bits(*self as u8)
	}
}

#[test]
fn test_from_bits() {
	assert_eq!(Kind::from_bits(Kind::Eof as u8), Kind::Eof);
	assert_eq!(Kind::from_bits(Kind::Whitespace as u8), Kind::Whitespace);
	assert_eq!(Kind::from_bits(Kind::Comment as u8), Kind::Comment);
	assert_eq!(Kind::from_bits(Kind::CdcOrCdo as u8), Kind::CdcOrCdo);
	assert_eq!(Kind::from_bits(Kind::Number as u8), Kind::Number);
	assert_eq!(Kind::from_bits(Kind::Dimension as u8), Kind::Dimension);
	assert_eq!(Kind::from_bits(Kind::BadString as u8), Kind::BadString);
	assert_eq!(Kind::from_bits(Kind::BadUrl as u8), Kind::BadUrl);
	assert_eq!(Kind::from_bits(Kind::Ident as u8), Kind::Ident);
	assert_eq!(Kind::from_bits(Kind::Function as u8), Kind::Function);
	assert_eq!(Kind::from_bits(Kind::AtKeyword as u8), Kind::AtKeyword);
	assert_eq!(Kind::from_bits(Kind::Hash as u8), Kind::Hash);
	assert_eq!(Kind::from_bits(Kind::String as u8), Kind::String);
	assert_eq!(Kind::from_bits(Kind::Url as u8), Kind::Url);
	assert_eq!(Kind::from_bits(Kind::UnicodeRange as u8), Kind::UnicodeRange);
	assert_eq!(Kind::from_bits(Kind::Delim as u8), Kind::Delim);
	assert_eq!(Kind::from_bits(Kind::Colon as u8), Kind::Colon);
	assert_eq!(Kind::from_bits(Kind::Semicolon as u8), Kind::Semicolon);
	assert_eq!(Kind::from_bits(Kind::Comma as u8), Kind::Comma);
	assert_eq!(Kind::from_bits(Kind::LeftSquare as u8), Kind::LeftSquare);
	assert_eq!(Kind::from_bits(Kind::RightSquare as u8), Kind::RightSquare);
	assert_eq!(Kind::from_bits(Kind::LeftParen as u8), Kind::LeftParen);
	assert_eq!(Kind::from_bits(Kind::RightParen as u8), Kind::RightParen);
	assert_eq!(Kind::from_bits(Kind::LeftCurly as u8), Kind::LeftCurly);
	assert_eq!(Kind::from_bits(Kind::RightCurly as u8), Kind::RightCurly);
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Kind>(), 1);
}
