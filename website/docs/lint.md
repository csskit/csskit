---
layout: markdown-base
---

# Lint

csskit can analyse your css and find problem code, and tell you about it.
Linting in csskit is done using the `csskit check` command. Supply a rule sheet
to define what is allowed or disallowed. This gives you full control over what
issues to flag and even how to flag them.

## Quick Start

Create a `lint.cks` file with your rules. Rules work just like CSS selectors,
except rather than selecting for HTML tags, you select for "CSS nodes".

```css
/* lint.cks */
:prefixed {
	level: error;
	diagnostic: "Avoid prefixed properties";
}

compound-selector:has(id) {
	level: error;
	diagnostic: "Avoid IDs in css selectors";
}

style-value[name="color"]:not(:computed) {
	level: error;
	diagnostic: "Don't use literals in color style values, only our design tokens";
}

compound-selector:has(:nth-child(4)) {
	level: warning;
	diagnostic: "Complex selector, consider refactoring";
}
```

With the above rule sheet, and the following CSS:

```css
/* styles.css */
#a {
	color: red;
	-webkit-animation: spin 3s;
}

div.b.c.d.e {
	color: var(--red-500);
}
```

Running `csskit check lint.cks styles.css` will output:

```md
  × Avoid IDs in css selectors
   ╭─[test.css:1:1]
 1 │ #a {
   · ─┬
   ·  ╰── Avoid IDs in css selectors
 2 │     color: red;
   ╰────
  × Don't use literals in color style values, only our design tokens
   ╭─[test.css:2:2]
 1 │ #a {
 2 │     color: red;
   ·     ─────┬─────
   ·          ╰── Don't use literals in color style values, only our design tokens
 3 │ }
   ╰────
  ⚠ Complex selector, consider refactoring
   ╭─[test.css:5:1]
 4 │
 5 │ div.b.c.d.e {
   · ─────┬─────
   ·      ╰── Complex selector, consider refactoring
 6 │     color: var(--red-500);
   ╰────
Error: 1 files failed check!
```

You can also make more advanced checks by using counters to collect information,
and `@when` rules to trigger behaviour conditionally:

```css
declaration:important {
	collect: --number-of-important-rules;
}

/* Allow 5 !important rules, but error if there are more */
@when (--number-of-important-rules > 5) {
	message: "Too many !important rules!";
	level: error;
}
```

## Learn More

For a complete reference of all available selectors, pseudo-classes, at-rules, and
how to use `csskit_ast` effectively, see the [Rules](/docs/rules) documentation.
