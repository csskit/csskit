# lsp_service

A language-agnostic [Language Server Protocol] transport for building language
servers on top of.

📖 **[Full Documentation]**

## Features

- **JSON-RPC 2.0 over stdio**: header framing, thread-based, no async runtime
- **`Handler` trait**: dispatches every LSP request and notification to a typed method
- **`Client`**: sends server-to-client notifications such as `textDocument/publishDiagnostics`, and requests such as `client/registerCapability`
- **Position encoding**: negotiates `utf-8` with clients that support it, with a UTF-16 fallback

## Part of csskit

This crate is part of the csskit project, a comprehensive CSS toolchain.

For more information, visit [csskit.rs].

## License

MIT

[Language Server Protocol]: https://microsoft.github.io/language-server-protocol/
[Full Documentation]: https://csskit.rs/docs/internal/lsp_service/
[csskit.rs]: https://csskit.rs/
