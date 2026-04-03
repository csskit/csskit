---
layout: markdown-base
---

# Neovim

You can use csskit as an LSP server in Neovim via [nvim-lspconfig]. Since csskit
isn't in the lspconfig registry yet, you'll need to register it as a
custom server.

## Setup

Add the following to your Neovim config. If you're using [LazyVim], this goes in
a plugin spec file like `lua/plugins/lsp-config.lua`:

```lua
return {
  {
    "neovim/nvim-lspconfig",
    opts = function(_, opts)
      local lspconfig = require("lspconfig")
      local configs = require("lspconfig.configs")
      local util = require("lspconfig.util")

      if not configs.csskit then
        configs.csskit = {
          default_config = {
            cmd = { "csskit", "lsp" },
            filetypes = { "css" },
            root_dir = util.root_pattern("package.json", ".git"),
            settings = {},
          },
        }
      end

      opts.servers = opts.servers or {}
      opts.servers.csskit = opts.servers.csskit or {}

      lspconfig.csskit.setup({})
    end,
  },
}
```

This assumes `csskit` is on your `$PATH` (e.g. installed via `npm install -g
csskit`). If you're using a local build, replace `"csskit"` with the full path
to the binary:

```lua
cmd = { "/path/to/csskit", "lsp" },
```

## Disabling other CSS LSPs

If you have `cssls` (the VSCode CSS language server) or another CSS LSP running,
you may want to disable it to avoid duplicate results. With LazyVim, add a setup
handler that prevents it from starting:

```lua
opts.setup = opts.setup or {}
opts.setup.cssls = function()
  return true
end
```

Returning `true` tells LazyVim "I'm handling this server" and skips its default
setup.

If `cssls` is being auto-configured by
[mason-lspconfig](https://github.com/williamboman/mason-lspconfig.nvim), you may
need to either uninstall the `css-lsp` Mason package (`:MasonUninstall
css-lsp`) or explicitly prevent it from auto-starting:

```lua
opts.setup.cssls = function()
  lspconfig.cssls.setup({ autostart = false })
  return true
end
```

## Verifying

Open a `.css` file and run `:LspInfo`. You should see `csskit` listed as an
active client. If `cssls` or other CSS servers still appear, check the
"Disabling other CSS LSPs" section above.

[nvim-lspconfig]: https://github.com/neovim/nvim-lspconfig
[LazyVim]: https://www.lazyvim.org/
