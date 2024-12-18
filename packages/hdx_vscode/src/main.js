const vscode = require("vscode");
const { LanguageClient } = require("vscode-languageclient/node");

module.exports = { activate };

/**
 * @param {vscode.ExtensionContext} context
 * @returns Promise<void>
 */
async function activate(context) {
	const traceOutputChannel = vscode.window.createOutputChannel("hdx Language Server Trace");
	const env = Object.assign({}, process.env);
	const client = new LanguageClient(
		"hdx",
		"hdx Language Server",
		{
			run: {
				command: env.HDX_SERVER_PATH || "hdx",
				args: ["lsp"],
				options: { env },
			},
			debug: {
				command: env.HDX_SERVER_PATH || "hdx",
				args: ["--debug", "lsp"],
				options: { env },
			},
		},
		{
			documentSelector: [{ scheme: "file", language: "css" }],
			diagnosticCollectionName: "hdx",
			traceOutputChannel,
		},
	);
	context.subscriptions.push(client.start());
}
