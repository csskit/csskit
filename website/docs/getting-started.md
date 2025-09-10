---
layout: markdown-base
---

# 🚀 Getting Started

Welcome to csskit! This guide will get you up and running in less than 5 minutes.

## Quick Start

> [!WARNING]
> csskit is still alpha software! It is ready to experiment with and provide
> feedback but using it for production projects might result in bugs!

### 1. Install csskit

```bash
# Install globally (recommended for trying out)
npm install --global csskit@canary

# Or install in your project
npm install --save-dev csskit@canary

# Or use it once without installign
npx csskit@canary
```

### 2. Compress your first file

Given a CSS file:

```css
/* styles.css - before minify */
body {
	color: red;
	background: white;
	margin: 0;
	padding: 10px;
}

.card {
	border: 1px solid #ccc;
	padding: 20px;
	margin: 10px;
}
```

Compress it with csskit:

```bash
csskit min styles.css
```

```css
body {
	color: red;
	background: white;
	margin: 0;
	padding: 10px;
}
.card {
	border: 1px solid #ccc;
	padding: 20px;
	margin: 10px;
}
```

You'll see the minified output:

### 3. Try the other tools

```bash
# Lint for issues and suggestions
csskit lint styles.css

# Minify for production
csskit min styles.css

# Take many files and build them into a shared set of files:
csskit bundle entry1.css entry2.css --output css-dist/

# Analyze colors in your CSS
csskit colors styles.css
```

## Installation Options

**Global install** - Use csskit anywhere:

```bash
npm install -g csskit
csskit fmt styles.css
```

**Project install** - Better for teams:

```bash
npm install --save-dev csskit
npx csskit fmt styles.css
```

**Package scripts** - Most convenient:

```json5
// package.json
{
	scripts: {
		"css:fmt": "csskit fmt src/**/*.css --write",
		"css:lint": "csskit lint src/**/*.css",
		"css:build": "csskit bundle src/entry-*.css -o dist/",
	},
}
```

Each script can be used:

- `npm run css:fmt`
- `npm run css:lint`
- `npm run css:build`

## What's Next?

- **Deep dive:** Read the full guides on each tool:
  - <b>💅</b> <a href="/docs/fmt">Format</a>
  - <b>🔍</b> <a href="/docs/lint">Lint</a>
  - <b>⚡</b> <a href="/docs/min">Minify</a>
  - <b>🔄</b> <a href="/docs/transform">Transpile</a>
  - <b>📦</b> <a href="/docs/bundle">Bundle</a>
  - <b>🔬</b> <a href="/docs/analyze">Analyze</a>
  - <b>💻</b> <a href="/docs/lsp">LSP</a>
- **Try online:** Test csskit in the [🎪 playground](/playground/) without installing
- **Get help:** Join our [💬 community discussions](https://github.com/csskit/csskit/discussions)
