<p align="center">
  <picture>
    <img alt="csskit" src="./website/images/logo.svg" width="890">
  </picture>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/csskit">
		<img src="https://img.shields.io/npm/v/csskit.svg" alt="csskit on npm">
	</a><a href="https://github.com/csskit/csskit/actions/workflows/test.yml">
		<img src="https://img.shields.io/github/actions/workflow/status/csskit/csskit/test.yml" alt="recent builds">
	</a>
  <a href="https://github.com/csskit/csskit/blob/main/LICENSE">
		<img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
	</a>
	<a href="https://github.com/keithamus/homebrew-tap/blob/main/Formula/csskit.rb">
		<img src="https://img.shields.io/badge/dynamic/regex?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkeithamus%2Fhomebrew-tap%2Fmain%2FFormula%2Fcsskit.rb&search=%2Fv(%5Cd%2B%5C.%5Cd%2B%5C.%5Cd%2B)%2F&replace=v%241&label=brew" alt="csskit on Homebrew">
	</a><a href="https://aur.archlinux.org/packages/csskit-bin">
		<img src="https://img.shields.io/aur/version/csskit-bin" alt="csskit on the AUR">
	</a>
</p>

> [!WARNING]
> **Alpha Quality**: csskit is in active development and not yet ready for
> production use. APIs may change without notice.

**csskit** is a high-performance CSS toolchain written in Rust, inspired by
[oxc]. It provides blazingly fast parsing, minification, linting, formatting,
as well as IDE integration with an LSP.

## Features

- **Parser**: Spec-compliant CSS parser with full AST support
- **Minifier**: Aggressive minification for faster asset delivery
- **Formatting**: Beautify your CSS with consistent formatting
- **Transformations**: Transform modern syntax to support a wide variety of
  browsers.
- **LSP**: Language server integration for editors and IDEs
- **Highlighting**: Syntax highlighting support with Semantic Tokens

**[Visit csskit.rs for full documentation and guides →][docs]**

## Installation

### Node.js

```sh
npm install csskit
```

## Packages

### Rust Crates

| Crate                                         | Description                |
| --------------------------------------------- | -------------------------- |
| [css_ast](./crates/css_ast)                   | CSS Abstract Syntax Tree   |
| [css_lexer](./crates/css_lexer)               | CSS tokenizer/lexer        |
| [css_parse](./crates/css_parse)               | CSS parser                 |
| [csskit](./crates/csskit)                     | Main CLI and library       |
| [csskit_lsp](./crates/csskit_lsp)             | Language Server Protocol   |
| [csskit_transform](./crates/csskit_transform) | AST transformations        |
| [csskit_highlight](./crates/csskit_highlight) | Syntax highlighting        |
| [chromashift](./crates/chromashift)           | Color space conversions    |
| [css_feature_data](./crates/css_feature_data) | Browser compatibility data |

### Node.js Packages

| Package                                        | Description          |
| ---------------------------------------------- | -------------------- |
| [csskit](https://www.npmjs.com/package/csskit) | CLI tool for Node.js |

### Editor Extensions

| Package                             | Description      |
| ----------------------------------- | ---------------- |
| [VSCode](./packages/csskit_vscode/) | VSCode Extension |
| [Zed](/packages/csskit_zed)         | Zed Extension    |

## Acknowledgments

Previously known as `hdx`. Special thanks to [@sethvincent] for graciously
transferring the `csskit` name.

## License

MIT

[oxc]: https://github.com/Boshen/oxc
[docs]: https://csskit.rs
[@sethvincent]: https://github.com/sethvincent
