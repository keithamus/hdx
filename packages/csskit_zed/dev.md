To install the dev version of the zed extension:

1. `rustup target add wasm32-wasip1`
2. Extensions > Install Dev Extension > Select the `pacakges/csskit_zed` folder
3. You may wish to disable the default css LSP's:
   ```json
   "language_servers": [
	  "!vscode-css-language-server",
	  "!tailwindcss-language-server",
	  "..."
   ]
   ```
