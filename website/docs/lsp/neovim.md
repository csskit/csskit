---
layout: markdown-base
---

# Neovim

csskit is available in both the [nvim-lspconfig] and [mason.nvim] registries.

If you're using a "neovim distro" such as [LazyVim], [NvChad], [AstroNvim], or [LunarVim], these all include [mason.nvim] and [nvim-lspconfig] by default.

## Quick setup (Mason)

If you already have [mason.nvim] and [nvim-lspconfig] installed, run:

```
:MasonInstall csskit
```

Mason will install the binary and configure lspconfig automatically. Open a
`.css` file and run `:LspInfo` to confirm csskit is active.

## Manual setup

If you're not using Mason, install the `csskit` binary yourself:

- **npm**: `npm install -g csskit`
- **Cargo**: `cargo install csskit`
- **Binary download**: Grab a release from [GitHub Releases](https://github.com/csskit/csskit/releases)

Then register the server in your Neovim config:

```lua
require("lspconfig").csskit.setup({})
```

Open a `.css` file and run `:LspInfo` to confirm csskit is active.

## Disabling other CSS LSPs

If you have `cssls` (the VSCode CSS language server) or another CSS LSP running,
you may want to disable it to avoid duplicate results.

**Uninstall via Mason:**

```
:MasonUninstall css-lsp
```

**Or prevent it from starting** (LazyVim):

```lua
opts.setup = opts.setup or {}
opts.setup.cssls = function()
  return true
end
```

Returning `true` tells LazyVim "I'm handling this server" and skips its default
setup.

[nvim-lspconfig]: https://github.com/neovim/nvim-lspconfig
[mason.nvim]: https://github.com/williamboman/mason.nvim
[LazyVim]: https://www.lazyvim.org/
[NvChad]: https://nvchad.com/
[AstroNvim]: https://astronvim.com/
[LunarVim]: https://www.lunarvim.org/
