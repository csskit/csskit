---
layout: markdown-base
---

# Rules

Rule sheets are defined using a CSS-like syntax. Rules are written using similar
selectors to CSS, but the pseudo-classes and tag names are different. Properties
and rules are also different.

Rules operate on the CSS Abstract Syntax Tree (AST). Each rule you write will
operate on one or more nodes in the tree, just like CSS does to the DOM tree.

## Selectors

### Combinators

csskit rule sheets support all the combinators that CSS does:

- **Descendant combinator** (space) - Matches any descendant node in the AST

  ```css
  style-rule declaration {
  } /* Any declaration within a style-rule */
  ```

- **Child combinator** (`>`) - Matches direct children only

  ```css
  style-rule > declaration {
  } /* Only declarations directly in a style-rule */
  ```

- **Next sibling combinator** (`+`) - Matches the immediately following sibling

  ```css
  media-rule + style-rule {
  } /* style-rule immediately after a media-rule */
  ```

- **Subsequent sibling combinator** (`~`) - Matches any following sibling
  ```css
  media-rule ~ style-rule {
  } /* Any style-rule after a media-rule */
  ```

### Type Selectors

CSS allows you to select for HTML tags like `div` or `section`, but rule sheets
select for CSS AST node types instead. CSS ASTs are complicated, and so there
are hundreds of different types. Here are some of the more interesting ones:

#### Rule types

Select different types of CSS rules:

- `style-rule` - Regular CSS style rules (e.g., `.class { color: red; }`)
- `media-rule` - `@media` queries
- `container-rule` - `@container` queries
- `supports-rule` - `@supports` feature queries
- `layer-rule` - `@layer` cascade layers
- `keyframes-rule` - `@keyframes` animations
- `font-face-rule` - `@font-face` custom fonts
- `property-rule` - `@property` custom property definitions
- `page-rule` - `@page` print rules
- `counter-style-rule` - `@counter-style` custom counters
- `namespace-rule` - `@namespace` XML namespace declarations
- `charset-rule` - `@charset` character encoding
- `starting-style-rule` - `@starting-style` transition start states

Example:

```css
/* Count all media queries */
media-rule {
	collect: --media-query-count;
}

/* Flag all @supports rules */
supports-rule {
	level: warning;
	diagnostic: "Feature query detected";
}
```

#### Selector types

Select selector-related nodes:

- `compound-selector` - A single selector sequence without combinators (e.g., `div.class#id`)
- `selector-list` - A comma-separated list of selectors
- `tag` - A single tag in a selector
- `id` - A single id in a selector
- `pseudo-class` - A single pseudo-class in a selector

Example:

```css
/* Flag complex compound selectors */
compound-selector:has(:nth-child(4)) {
	level: warning;
	diagnostic: "Complex selector - consider refactoring";
}
```

#### Declarations & Values

Select CSS properties and values:

- `declaration` - A CSS property declaration (e.g., `color: red;`)
- `style-value` - Any CSS value (colors, functions, dimensions, keywords, etc.)

Example:

```css
/* Count all declarations */
declaration {
	collect: --total-properties;
}

/* Flag literal color values */
style-value[name="color"]:not(:computed) {
	level: error;
	diagnostic: "Use design tokens instead of literal colors";
}
```

### Attribute Selectors

Query nodes by their properties using attribute selector syntax. Attribute
selectors work just like CSS, but the attributes you can select for are limited.

- `[name]` - Has a name property
- `[name=value]` - Exact match
- `[name^=prefix]` - Starts with prefix
- `[name$=suffix]` - Ends with suffix
- `[name*=contains]` - Contains substring
- `[name~=word]` - Contains word in space-separated list
- `[name|=lang]` - Language prefix match (dash-separated)

Example:

```css
/* Flag all background-* properties */
declaration[name^="background"] {
	collect: --background-props;
}

/* Find specific animation */
keyframes-rule[name="slide"] {
	level: info;
	diagnostic: "Found slide animation";
}

/* Flag color properties */
declaration[name="color"] {
	collect: --color-usage;
}
```

### Pseudo-Classes

#### Selector-Operating Pseudo-Classes

csskit rule sheets support the `:not()` and `:has()` pseudo classes just like
CSS. These pseudo-classes take selectors as arguments and match based on them:

- `:not(<selector>)` - Negation, matches nodes NOT matching the inner selector

  ```css
  /* All non-computed color values */
  style-value[name="color"]:not(:computed) {
  	level: error;
  	diagnostic: "Use CSS variables for colors";
  }
  ```

- `:has(<relative-selector>)` - Matches elements with matching descendants
  ```css
  /* Compound selectors containing IDs */
  compound-selector:has(id) {
  	level: error;
  	diagnostic: "Avoid IDs in selectors";
  }
  ```

#### Tree-Structure Pseudo-Classes

All of CSS' tree pseudo-classes are supported. These match based on a node's
position in the AST:

- `:first-child` - First child of its parent
- `:last-child` - Last child of its parent
- `:only-child` - Only child of its parent
- `:first-of-type` - First of its type among siblings
- `:last-of-type` - Last of its type among siblings
- `:only-of-type` - Only one of its type among siblings

- `:nth-child(<An+B>)` - Matches nth children (supports `An+B` notation)

  ```css
  /* Every other declaration */
  declaration:nth-child(2n) {
  	collect: --even-declarations;
  }
  ```

- `:nth-last-child(<An+B>)` - Matches from the end
- `:nth-of-type(<An+B>)` - Matches nth of same type
- `:nth-last-of-type(<An+B>)` - Matches nth of type from end

Example:

```css
/* Flag the first rule in a file */
style-rule:first-child {
	level: info;
	diagnostic: "First rule in file";
}
```

#### Stateful Pseudo-Classes

These pseudo-classes check properties or characteristics of nodes:

- `:important` - Matches `!important` declarations

  ```css
  declaration:important {
  	level: warning;
  	diagnostic: "Avoid !important";
  }
  ```

- `:custom` - Matches custom properties (`--*`)

  ```css
  declaration:custom {
  	collect: --custom-property-count;
  }
  ```

- `:computed` - Matches computed values (`calc()`, `var()`, etc.)

  ```css
  style-value:computed {
  	collect: --computed-values;
  }
  ```

- `:shorthand` - Matches shorthand properties

  ```css
  declaration:shorthand {
  	collect: --shorthand-usage;
  }
  ```

- `:longhand` - Matches longhand properties

  ```css
  declaration:longhand {
  	collect: --longhand-usage;
  }
  ```

- `:unknown` - Matches unknown/unrecognized properties

  ```css
  declaration:unknown {
  	level: warning;
  	diagnostic: "Unknown property";
  }
  ```

- `:prefixed` - Matches vendor-prefixed properties

  ```css
  :prefixed {
  	level: error;
  	diagnostic: "Use autoprefixer instead of manual prefixes";
  }
  ```

- `:prefixed(<vendor>)` - Matches specific vendor prefixes

  ```css
  /* Only flag webkit prefixes */
  :prefixed(webkit) {
  	level: error;
  	diagnostic: "Remove -webkit- prefix";
  }
  ```

  Vendors: `webkit`, `moz`, `ms`, `o`

- `:property-type(<group>)` - Matches properties in a specific group

  ```css
  /* Flag animation properties */
  declaration:property-type(animation) {
  	collect: --animation-usage;
  }
  ```

  Available groups: `animation`, `background`, `border`, `color`, `display`,
  `flex`, `font`, `grid`, `layout`, `margin`, `padding`, `position`, `text`,
  `transform`, `transition`, and more.

- `:rule` - Matches declarations within rules
- `:at-rule` - Matches at-rules
- `:function` - Matches function values
- `:empty` - Matches empty containers
- `:root` - Matches the root node

- `:size(<comparison>)` - Matches nodes by size

  ```css
  /* Flag large selector lists */
  selector-list:size(> 10) {
  	level: warning;
  	diagnostic: "Selector list too large";
  }
  ```

  Operators: `>`, `<`, `>=`, `<=`, `!=`

## At-Rules in Rule Sheets

Rule sheets support special at-rules for defining statistics and conditional behavior.

### @stat

Define statistics to collect across your CSS:

```css
@stat --counter-name {
	type: counter; /* or: bytes, lines */
}
```

Types:

- `counter` - Count occurrences (default)
- `bytes` - Sum byte sizes
- `lines` - Sum line counts

Example:

```css
@stat --total-rules {
	type: counter;
}

@stat --total-size {
	type: bytes;
}

style-rule {
	collect: --total-rules;
}

:root {
	collect: --total-size;
}
```

When you run `csskit check`, statistics are displayed at the end:

```
Statistics:
  --total-rules: 42
  --total-size: 15.2 KB
```

### @when

Trigger behavior conditionally based on collected statistics:

```css
@when (<condition>) {
	message: "Custom message";
	level: error; /* or: warning, advice, info */
}
```

Conditions support comparisons: `>`, `<`, `>=`, `<=`, `=`, `!=`

Example:

```css
declaration:important {
	collect: --important-count;
}

/* Allow up to 5 !important, but error beyond that */
@when (--important-count > 5) {
	message: "Too many !important declarations!";
	level: error;
}

/* Warn about large files */
@when (--total-size > 100000) {
	message: "CSS file exceeds 100KB";
	level: warning;
}
```

## Properties in Rule Sheets

Within rule blocks, you can use these properties:

### collect

Increment a stat counter for matching nodes:

```css
declaration[name="color"] {
	collect: --color-usage;
}
```

### level

Set the diagnostic severity level:

- `error` - Fails the check (exit code 1)
- `warning` - Shows as warning (⚠)
- `advice` - Shows as suggestion
- `info` - Shows as information

```css
:prefixed {
	level: error;
}
```

### diagnostic

Set the message shown for matching nodes:

```css
compound-selector:has(id) {
	level: error;
	diagnostic: "Avoid IDs in selectors for better specificity management";
}
```
