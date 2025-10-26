# csskit

A comprehensive CSS toolchain for formatting, linting, minifying, and building CSS.

📖 **[Full Documentation](https://csskit.rs/)**

## Purpose

csskit is a complete CSS toolchain that provides formatting, linting,
minification, transpilation, bundling, and analysis in a single binary. It aims
to be "CSS's missing toolchain" - providing everything you need without
reaching into external ecosystems. Developed to make writing CSS more fun,
productive, and easier.

### Features

- **💅 Format**: Opinionated code formatting to keep your CSS consistent
- **🔍 Lint**: Find mistakes and get helpful suggestions to write better CSS
- **⚡ Minify**: Blazing fast compression that optimizes your CSS for production
- **🔄 Transpile**: Use modern CSS features with automatic browser compatibility
- **📦 Bundle**: Smart bundling that combines and optimizes your CSS files
- **🔬 Analyze**: Deep insights into your CSS with color analysis and usage reports
- **💻 LSP**: integrates with Zed and VSCode for real-time guidance and code completion.

## Usage

```bash
# Format CSS files
csskit fmt styles.css

# Lint CSS
csskit lint styles.css

# Minify CSS
csskit min styles.css -o styles.min.css

# Build CSS bundles
csskit build src/main.css -o dist/bundle.css

# Analyze colors
csskit colors styles.css

# Start LSP server
csskit lsp
```

## License

MIT
