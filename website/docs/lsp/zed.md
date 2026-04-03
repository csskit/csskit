---
layout: markdown-base
---

# Zed

csskit has a Zed extension that connects csskit's LSP to Zed.

## Install

Open Zed and go to **Extensions** (or press `cmd+shift+x`), then search for
**csskit** and click **Install**.

The extension will automatically download the csskit binary and start the LSP
server when you open a CSS file. If `csskit` is already on your `$PATH`, the
extension will use that instead.

## Configuration

To enable debug logging, add the following to your Zed settings:

```json
{
	"lsp": {
		"csskit": {
			"settings": {
				"debug": true
			}
		}
	}
}
```

This passes `--debug` to the `csskit lsp` command, which outputs trace-level
logs to stderr.
