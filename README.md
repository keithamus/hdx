<p align="center">
  <picture>
    <img alt="HDX" src="./logo.png" width="890">
  </picture>
</p>

## CSS Compiler

hydroxide (hdx) is a suite of high performance tools for CSS, written in Rust,
inspired by [the oxidation compiler][1].

## Goals

The goal of this project is to provide a high quality set of tools for writing
native CSS and shipping said CSS into production the best way possible. This
means:

- Preventing mistakes at author time (parsing & linting).
- Advising best practices and highlighting pitfalls (linting).
- Allow consistent homogeneous code to be written (formatting).
- Allowing the authorship of modern CSS that is downsampled for browser support
  ("transpiling").
- Provides integration with IDEs (LSP).
- Producing the smallest available artefacts (minification & bundling).
- Provide a way for authors to migrate from alternative authoring formats (such
  as SCSS).

## Roadmap

This project is in the very early stages, and has yet to meet any of its goals.
Here is a rough order of operations to get there:

- [x] Write a suitable lexer which:
  - [x] Passes the [romainmenke/css-tokenizer-tests][2] test suite.
- [ ] Write a suitable parser which:
  - [ ] Passes the [postcss/postcss-parser-tests][3] test suite.
  - [x] Produces an AST
    - [x] with JSON output
    - [x] with Visitors
    - [x] with CSS output
  - [x] Produces a usable AST for:
    - [x] [960.gs][4]
    - [x] [animate][5]
    - [x] [blueprint][6]
    - [x] [bootstrap 4][7]
    - [x] [bootstrap 5][7]
    - [x] [font awesome][8]
    - [x] [foundation][9]
    - [x] [inuit][10]
    - [x] [mini.css][11]
    - [x] [openprops][12]
    - [x] [pure][13]
    - [x] [reset (Eric Myer's)][14]
    - [x] [tailwind][15]
    - [x] [primer css][16]
  - [ ] Benchmarks faster than (or as fast as) comparative parsers
  - [ ] Is capable of interpreting SCSS files
    - [ ] Produces a usable AST for:
      - [ ] all [bootstrap][7] scss files
      - [ ] all [foundation][9] scss files
      - [ ] all [mini.css][11] scss files
      - [ ] all [primer css][16] scss files
      - [ ] all [bourbon][17] scss files
- [x] Write a suitable minifier which:
  - [x] Removes trivia such as whitespace/comments
  - [ ] Minifies/Folds/Downsamples complex CSS into shorter values:
    - [x] Minifies hex colours into their smallest representation
    - [ ] Folds colour values into shorter hex
    - [ ] Can downsample css values to shorter, e.g. `0px` -> `0`
    - [ ] Can combine multiple properties into a shorthand e.g. `background`
    - [ ] Can remove duplicate properties
  - [ ] Produces a usable CSS for:
    - [x] [960.gs][4]
    - [x] [animate][5]
    - [x] [blueprint][6]
    - [x] [bootstrap 4][7]
    - [x] [bootstrap 5][7]
    - [ ] [font awesome][8]
    - [ ] [foundation][9]
    - [x] [inuit][10]
    - [ ] [mini.css][11]
    - [ ] [openprops][12]
    - [x] [pure][13]
    - [x] [reset (Eric Myer's)][14]
    - [ ] [tailwind][15]
    - [ ] [primer css][16]
  - [ ] Produces minified CSS smaller than (or as small as) comparative
        minifiers for
    - [x] [960.gs][4]
    - [ ] [animate][5]
    - [ ] [blueprint][6]
    - [ ] [bootstrap 4][7]
    - [ ] [bootstrap 5][7]
    - [ ] [font awesome][8]
    - [ ] [foundation][9]
    - [ ] [inuit][10]
    - [ ] [mini.css][11]
    - [ ] [openprops][12]
    - [x] [pure][13]
    - [x] [reset (Eric Myer's)][14]
    - [ ] [tailwind][15]
    - [ ] [primer css][16]
  - [ ] Benchmarks faster than (or as fast as) comparative minifiers
- [ ] Write the formatter
  - [ ] Provide the formatter as an npm package
- [ ] Write the linter
  - [ ] Write a visitor architecture for the AST
  - [ ] The linter highlights basic parser errors
  - [ ] The linter reports on duplicate CSS
  - [ ] The linter reports on empty CSS
  - [ ] The linter reports on non-standard or vendor prefixed CSS
  - [ ] The linter can safely fix violations
  - [ ] The linter can provide explanations / documentation for every violation
        including guidance on how to fix.
  - [ ] Provides all of the rules that comparable linters do
    - [ ] Provides all core rules from [stylelint][18].
  - [ ] Provide the linter as an npm package
- [ ] Write the transpiler which:
  - [ ] Can vendor prefix new properties
  - [ ] Can convert new colour formats like oklch to rgb/hex
  - [ ] Can convert nested css to flattened
  - [ ] Can convert SCSS to CSS
    - [ ] Can convert the following SCSS to CSS:
      - [ ] all [bootstrap][7] scss files
      - [ ] all [foundation][9] scss files
      - [ ] all [mini.css][11] scss files
      - [ ] all [primer css][16] scss files
      - [ ] all [bourbon][17] scss files
- [ ] Write the bundler which:
  - [ ] Produces a single file from `@import`s
  - [ ] Produces a usable single file for:
    - [ ] [bootstrap][5]
    - [ ] [foundation][9]
    - [ ] [mini.css][11]
    - [ ] [openprops][6]
    - [ ] [primer css][16]
  - [ ] Can do more advanced compression techniques
    - [ ] Mangle variable names
    - [ ] Remove variable names to reduce output size
    - [ ] Add variable names to reduce output size
  - [ ] Resolves URL imports
    - [ ] Produces lockfiles
- [ ] Write the LSP
  - [ ] Integrate the linter into the LSP
  - [ ] Integrate the formatter into the LSP
  - [ ] Integrate with VSCode (extension?)
  - [ ] Integrate with nvim (nvim-lsp?)

[1]: https://github.com/Boshen/oxc
[2]: https://github.com/romainmenke/css-tokenizer-tests
[3]: https://github.com/postcss/postcss-parser-tests
[4]: https://github.com/nathansmith/960-grid-system
[5]: https://github.com/animate-css/animate.css
[6]: https://github.com/joshuaclayton/blueprint-css
[7]: https://github.com/twbs/bootstrap
[8]: https://github.com/FortAwesome/Font-Awesome
[9]: https://github.com/foundation/foundation-sites/tree/develop/scss
[10]: https://github.com/inuitcss/inuitcss
[11]: https://github.com/Chalarangelo/mini.css/
[12]: https://github.com/argyleink/open-props
[13]: https://github.com/pure-css/pure/
[14]: https://meyerweb.com/eric/tools/css/reset/
[15]: https://github.com/tailwindlabs/tailwindcss
[16]: https://github.com/primer/css
[17]: https://github.com/thoughtbot/bourbon
[18]: https://github.com/stylelint/stylelint
