# css_parse

A recursive-descent CSS parser with generic cursor sinks and rich diagnostics.

📖 **[Full Documentation](https://csskit.rs/docs/internal/css_parse/)**

## Features

- **Recursive-descent parser**: Clean, maintainable parsing architecture
- **Generic cursor sinks**: Flexible output handling for different use cases
- **Rich diagnostics**: Detailed error messages with source locations
- **Reusable grammar helpers**: Shared parsing utilities for common CSS patterns
- **Error recovery**: Continues parsing after errors to find multiple issues

## Optional Features

- `miette` - Enables rich diagnostic output with miette integration
- `serde` - Enables serialization/deserialization support
- `fancy` - Enables fancy diagnostic output (includes miette/fancy-no-backtrace)

## Part of csskit

This crate is part of the csskit project, a comprehensive CSS toolchain.

For more information, visit [csskit.rs](https://csskit.rs/).

## License

MIT
