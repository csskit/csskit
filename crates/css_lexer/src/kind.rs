use core::fmt;

use crate::KindSet;

/// Kind represents the token "Type", categorised mostly by the token types within the CSS Syntax spec.
///
/// Importantly, `Kind` is represented as `u8` and must only use the 5 low bits, because the upper 3 bits get used to
/// house details about each kind, that a token would be interested in learning about.
///
/// Maintaining parity with the spec makes it easier to reason about logic around the parser, despite it being possible to
/// group a bunch of these tokens into a single "delimiter" token. These Delim kinds, however, set the upper bit which
/// means they cannot be inserted directly into a token. Instead a token.
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

	// Errors (mask as 0b1_XXXX)
	/// Represents the [&lt;bad-string-token>][1]. This token is a failure to fully lex the [&lt;string-token>][2].
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-bad-string-token
	/// [2]: https://drafts.csswg.org/css-syntax/#typedef-string-token
	BadString = 0b1_1100,

	/// Represents the [&lt;bad-url-token>][1]. This token is a failure to fully lex the [&lt;url-token>][2].
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-bad-url-token
	/// [2]: https://drafts.csswg.org/css-syntax/#typedef-url-token
	BadUrl = 0b1_1101,

	/// These kind are non-standard Bad kinds and never emitted by the Lexer, but can be used by Parsers to denote a
	/// token that are either:
	///  - a Token that was unexpected in this position.
	///  - a Token that was inserted to recover the parser to a known state.
	BadWhitespace = 0b1_0001,
	BadComment = 0b1_0010,
	BadCdcOrCdo = 0b1_0011,
	BadNumber = 0b1_0100,
	BadDimension = 0b1_0101,
	BadIdent = 0b1_1000,
	BadFunction = 0b1_1001,
	BadAtKeyword = 0b1_1010,
	BadHash = 0b1_1011,
	BadDelim = 0b1_1111,

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
	/// Represents the [&lt;delim-token>][1]. The `<delim-token>` has a value composed of a single code point.
	///
	/// ```md
	/// <delim-token>
	///  │├─ [codepoint] ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-delim-token
	Delim = 0b1111,

	// Single character Tokens (mask 0b11_XXXX)
	/// Represents the [&lt;colon-token>][1].
	///
	/// ```md
	/// <colon-token>
	///  │├─ ":" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-colon-token
	Colon = 0b10_0001,

	/// Represents the [&lt;semicolon-token>][1].
	///
	/// ```md
	/// <semicolon-token>
	///  │├─ ";" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-semicolon-token
	Semicolon = 0b10_0010,

	/// Represents the [&lt;comma-token>][1].
	///
	/// ```md
	/// <comma-token>
	///  │├─ "," ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-comma-token
	Comma = 0b10_0011,

	/// Represents the [&lt;\[-token>][1].
	///
	/// ```md
	/// <[-token>
	///  │├─ "[" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-square
	LeftSquare = 0b10_0100,

	/// Represents the [&lt;\]-token>][1].
	///
	/// ```md
	/// <]-token>
	///  │├─ "]" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-square
	RightSquare = 0b10_0101,

	/// Represents the [&lt;(-token>][1].
	///
	/// ```md
	/// <(-token>
	///  │├─ "(" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-paren
	LeftParen = 0b10_0110,

	/// Represents the [&lt;)-token>][1].
	///
	/// ```md
	/// <)-token>
	///  │├─ ")" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-paren
	RightParen = 0b10_0111,

	/// Represents the [&lt;{-token>][1].
	///
	/// ```md
	/// <{-token>
	///  │├─ "{" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-open-curly
	LeftCurly = 0b10_1000,

	/// Represents the [&lt;}-token>][1].
	///
	/// ```md
	/// <}-token>
	///  │├─ "}" ─┤│
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#typedef-close-curly
	RightCurly = 0b10_1001,

	/// These kind are non-standard Bad kinds and never emitted by the Lexer, but can be used by Parsers to denote a
	/// token that are either:
	///  - a Token that was unexpected in this position.
	///  - a Token that was inserted to recover the parser to a known state.
	BadColon = 0b11_0001,
	BadSemicolon = 0b11_0010,
	BadComma = 0b11_0011,
	BadLeftSquare = 0b11_0100,
	BadRightSquare = 0b11_0101,
	BadLeftParen = 0b11_0110,
	BadRightParen = 0b11_0111,
	BadLeftCurly = 0b11_1000,
	BadRightCurly = 0b11_1001,
}

impl Kind {
	pub(crate) const fn from_bits(bits: u8) -> Self {
		match bits {
			0b0001 => Self::Whitespace,
			0b0010 => Self::Comment,
			0b0011 => Self::CdcOrCdo,
			0b0100 => Self::Number,
			0b0101 => Self::Dimension,
			// 0b0110 => Reserved
			// 0b0111 => Reserved
			0b1000 => Self::Ident,
			0b1001 => Self::Function,
			0b1010 => Self::AtKeyword,
			0b1011 => Self::Hash,
			0b1100 => Self::String,
			0b1101 => Self::Url,
			0b1110 => Self::UnicodeRange,
			0b1111 => Self::Delim,

			// Error tokens are represented in 5 bits.
			0b1_0001 => Self::BadWhitespace,
			0b1_0010 => Self::BadComment,
			0b1_0011 => Self::BadCdcOrCdo,
			0b1_0100 => Self::BadNumber,
			0b1_0101 => Self::BadDimension,
			// 0b1_0110 => Self::Reserved,
			// 0b1_0111 => Self::Reserved,
			0b1_1000 => Self::BadIdent,
			0b1_1001 => Self::BadFunction,
			0b1_1010 => Self::BadAtKeyword,
			0b1_1011 => Self::BadHash,
			0b1_1100 => Self::BadString,
			0b1_1101 => Self::BadUrl,
			0b1_1110 => Self::UnicodeRange,
			0b1_1111 => Self::BadDelim,

			// Single character delimiters are represented in 6 bits
			0b10_0001 => Self::Colon,
			0b10_0010 => Self::Semicolon,
			0b10_0011 => Self::Comma,
			0b10_0100 => Self::LeftSquare,
			0b10_0101 => Self::RightSquare,
			0b10_0110 => Self::LeftParen,
			0b10_0111 => Self::RightParen,
			0b10_1000 => Self::LeftCurly,
			0b10_1001 => Self::RightCurly,

			0b11_0001 => Self::BadColon,
			0b11_0010 => Self::BadSemicolon,
			0b11_0011 => Self::BadComma,
			0b11_0100 => Self::BadLeftSquare,
			0b11_0101 => Self::BadRightSquare,
			0b11_0110 => Self::BadLeftParen,
			0b11_0111 => Self::BadRightParen,
			0b11_1000 => Self::BadLeftCurly,
			0b11_1001 => Self::BadRightCurly,
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
			Kind::Ident => "Ident",
			Kind::Function => "Function",
			Kind::AtKeyword => "AtKeyword",
			Kind::Hash => "Hash",
			Kind::String => "String",
			Kind::Url => "Url",
			Kind::UnicodeRange => "UnicodeRange",
			Kind::Delim => "Delim",

			Kind::BadWhitespace => "BadWhitespace",
			Kind::BadComment => "BadComment",
			Kind::BadCdcOrCdo => "BadCdcOrCdo",
			Kind::BadNumber => "BadNumber",
			Kind::BadDimension => "BadDimension",
			Kind::BadIdent => "BadIdent",
			Kind::BadFunction => "BadFunction",
			Kind::BadAtKeyword => "BadAtKeyword",
			Kind::BadHash => "BadHash",
			Kind::BadString => "BadString",
			Kind::BadUrl => "BadUrl",
			Kind::BadDelim => "BadDelim",

			Kind::Colon => "Colon",
			Kind::Semicolon => "Semicolon",
			Kind::Comma => "Comma",
			Kind::LeftSquare => "LeftSquare",
			Kind::RightSquare => "RightSquare",
			Kind::LeftParen => "LeftParen",
			Kind::RightParen => "RightParen",
			Kind::LeftCurly => "LeftCurly",
			Kind::RightCurly => "RightCurly",

			Kind::BadColon => "BadColon",
			Kind::BadSemicolon => "BadSemicolon",
			Kind::BadComma => "BadComma",
			Kind::BadLeftSquare => "BadLeftSquare",
			Kind::BadRightSquare => "BadRightSquare",
			Kind::BadLeftParen => "BadLeftParen",
			Kind::BadRightParen => "BadRightParen",
			Kind::BadLeftCurly => "BadLeftCurly",
			Kind::BadRightCurly => "BadRightCurly",
		}
	}

	pub const fn is_bad(&self) -> bool {
		(*self as u8) & 0b11_0000 == 0b01_0000
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
	assert_eq!(Kind::from_bits(Kind::Ident as u8), Kind::Ident);
	assert_eq!(Kind::from_bits(Kind::Function as u8), Kind::Function);
	assert_eq!(Kind::from_bits(Kind::AtKeyword as u8), Kind::AtKeyword);
	assert_eq!(Kind::from_bits(Kind::Hash as u8), Kind::Hash);
	assert_eq!(Kind::from_bits(Kind::String as u8), Kind::String);
	assert_eq!(Kind::from_bits(Kind::Url as u8), Kind::Url);
	assert_eq!(Kind::from_bits(Kind::UnicodeRange as u8), Kind::UnicodeRange);
	assert_eq!(Kind::from_bits(Kind::Delim as u8), Kind::Delim);

	assert_eq!(Kind::from_bits(Kind::BadWhitespace as u8), Kind::BadWhitespace);
	assert_eq!(Kind::from_bits(Kind::BadComment as u8), Kind::BadComment);
	assert_eq!(Kind::from_bits(Kind::BadCdcOrCdo as u8), Kind::BadCdcOrCdo);
	assert_eq!(Kind::from_bits(Kind::BadNumber as u8), Kind::BadNumber);
	assert_eq!(Kind::from_bits(Kind::BadDimension as u8), Kind::BadDimension);
	assert_eq!(Kind::from_bits(Kind::BadIdent as u8), Kind::BadIdent);
	assert_eq!(Kind::from_bits(Kind::BadFunction as u8), Kind::BadFunction);
	assert_eq!(Kind::from_bits(Kind::BadAtKeyword as u8), Kind::BadAtKeyword);
	assert_eq!(Kind::from_bits(Kind::BadHash as u8), Kind::BadHash);
	assert_eq!(Kind::from_bits(Kind::BadString as u8), Kind::BadString);
	assert_eq!(Kind::from_bits(Kind::BadUrl as u8), Kind::BadUrl);
	assert_eq!(Kind::from_bits(Kind::BadDelim as u8), Kind::BadDelim);

	assert_eq!(Kind::from_bits(Kind::Colon as u8), Kind::Colon);
	assert_eq!(Kind::from_bits(Kind::Semicolon as u8), Kind::Semicolon);
	assert_eq!(Kind::from_bits(Kind::Comma as u8), Kind::Comma);
	assert_eq!(Kind::from_bits(Kind::LeftSquare as u8), Kind::LeftSquare);
	assert_eq!(Kind::from_bits(Kind::RightSquare as u8), Kind::RightSquare);
	assert_eq!(Kind::from_bits(Kind::LeftParen as u8), Kind::LeftParen);
	assert_eq!(Kind::from_bits(Kind::RightParen as u8), Kind::RightParen);
	assert_eq!(Kind::from_bits(Kind::LeftCurly as u8), Kind::LeftCurly);
	assert_eq!(Kind::from_bits(Kind::RightCurly as u8), Kind::RightCurly);

	assert_eq!(Kind::from_bits(Kind::BadColon as u8), Kind::BadColon);
	assert_eq!(Kind::from_bits(Kind::BadSemicolon as u8), Kind::BadSemicolon);
	assert_eq!(Kind::from_bits(Kind::BadComma as u8), Kind::BadComma);
	assert_eq!(Kind::from_bits(Kind::BadLeftSquare as u8), Kind::BadLeftSquare);
	assert_eq!(Kind::from_bits(Kind::BadRightSquare as u8), Kind::BadRightSquare);
	assert_eq!(Kind::from_bits(Kind::BadLeftParen as u8), Kind::BadLeftParen);
	assert_eq!(Kind::from_bits(Kind::BadRightParen as u8), Kind::BadRightParen);
	assert_eq!(Kind::from_bits(Kind::BadLeftCurly as u8), Kind::BadLeftCurly);
	assert_eq!(Kind::from_bits(Kind::BadRightCurly as u8), Kind::BadRightCurly);
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Kind>(), 1);
}
