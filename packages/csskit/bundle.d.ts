// Type definitions for the WebAssembly entry (`csskit/bundle`).

/** A parse diagnostic produced by the wasm backend. */
export interface Diagnostic {
	from: number;
	to: number;
	severity: string;
	code: string;
	message: string;
	help: string;
}

/** Result of a wasm parse: a serialised AST plus diagnostics. */
export interface ParserResult {
	ast: unknown;
	diagnostics: Diagnostic[];
}

/** Parse `source` into a serialised AST plus diagnostics. */
export declare function parse(source: string): ParserResult;

/** Tokenise `source`. */
export declare function lex(source: string): unknown;

/** Minify `source`. */
export declare function minify(source: string): string;

/** Format `source` with the given options. */
export declare function format(source: string, options?: unknown): string;

/** Render a human-readable parse-error report for `source`. */
export declare function parseErrorReport(source: string): string;
