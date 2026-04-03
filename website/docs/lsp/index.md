---
layout: markdown-base
permalink: /docs/lsp/
---

# LSP

csskit includes a Language Server Protocol (LSP) server, giving you real-time
feedback directly in your editor. Run it with:

```bash
csskit lsp
```

You probably won't run this directly -- your editor will start it for you.

## Features

The csskit LSP currently provides:

- **Semantic Highlighting**: Rich syntax highlighting that understands CSS
  structure -- selectors, properties, values, at-rules, pseudo-classes, and
  more. The server provides delta updates for efficient re-highlighting as you
  type.

More features like diagnostics, completions, and code actions are planned.

## Editor Setup

csskit has editor extensions and configurations for:

- <b>💻</b> <a href="/docs/lsp/vscode">VSCode</a>
- <b>💻</b> <a href="/docs/lsp/zed">Zed</a>
- <b>💻</b> <a href="/docs/lsp/neovim">Neovim</a>

## Debug Mode

If you're developing or troubleshooting the LSP, you can enable verbose logging:

```bash
csskit --debug lsp
```

This outputs trace-level logs to stderr.
