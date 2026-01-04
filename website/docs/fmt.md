---
layout: markdown-base
---

# Formatting

csskit comes with a CSS code formatter built in. Use `csskit fmt` to format your
CSS files for better readability and consistency.

## Quick Start

Format a single file:

```bash
csskit fmt styles.css
```

Format multiple files:

```bash
csskit fmt styles.css theme.css components.css
```

Format from stdin (useful for piping):

```bash
cat styles.css | csskit fmt
echo "body{margin:0}" | csskit fmt
```

By default, `csskit fmt` writes formatted output to stdout. Use `-o` to write to
a file:

```bash
csskit fmt input.css -o output.css
```

## Options

### Tab Expansion

By default `csskit fmt` will use tabs for indentation. If you prefer spaces over
tabs, add `--expand-tab N` and it will expand tabs into N space characters:

```bash
csskit fmt --expand-tab 2 styles.css
```

### Quote Style

By default `csskit fmt` will use double quotes for strings. If you prefer single
quote characters over double quotes, add `--single-quotes`:

```bash
csskit fmt --single-quotes styles.css
```

Example:

Input:

```css
div {
	content: "hello world";
	font-family: "Arial", "Helvetica", sans-serif;
}
```

Output:

```css
div {
	content: "hello world";
	font-family: 'Arial', 'Helvetica', sans-serif;
}
```

### Check Mode

Use `--check` to verify if files need formatting without making changes. This is
useful in CI/CD pipelines:

```bash
csskit fmt --check styles.css
```

This will:

- Exit with status code 0 if no changes needed
- Exit with non-zero status code if formatting would change the file
- Report which changes would be made without writing them

Example in CI:

```bash
# Fail the build if CSS isn't formatted
csskit fmt --check src/**/*.css
```

### Processing Content Directly

Use `--content` (or `-c`) to format CSS content directly from the command line:

```bash
csskit fmt -c "body{margin:0;padding:0}"
```

Output:

```css
body {
	margin: 0;
	padding: 0;
}
```

## See Also

- [Lint](/docs/lint) - Check CSS for style violations
- [Analyze](/docs/analyze) - Gather statistics about your CSS
