---
layout: markdown-base
---

# Analyze

csskit can analyze your CSS to gather statistics and answer questions like:

- What colors are in my CSS? Do they meet contrast requirements?
- How many rules use `z-index`?
- How big are my CSS files?

## Colors

Extract and analyze all colors from your CSS:

```bash
csskit colors styles.css
```

Add `--wcag` to check WCAG contrast criteria (AA/AAA against black and white)
and find the closest colors that meet accessibility requirements:

```bash
csskit colors --wcag styles.css
```

## Custom Analysis

Use `csskit check` with a csskit sheet (`.cks` file) to collect custom
statistics. Given this CSS:

```css
/* style.css */
body {
	margin: 0;
}

.floating-panel {
	z-index: 999;
}
```

Create a stats sheet to count rules and measure file size:

```css
/* stats.cks */
style-rule {
	collect: --number-of-style-rules;
}

style-rule:has([name="z-index"]) {
	collect: --number-of-rules-with-z-index;
}

@stat --total-number-of-bytes {
	type: bytes;
}

:root {
	collect: --total-number-of-bytes;
}
```

Run the analysis:

```bash
csskit check stats.cks style.css
```

Output:

```
Statistics:
  --number-of-rules-with-z-index: 1
  --number-of-style-rules: 2
  --total-number-of-bytes: 56 bytes
```

## Learn More

For a complete reference of all available selectors, pseudo-classes, at-rules, and
how to use `csskit_ast` effectively, see the [Rules](/docs/rules) documentation.
