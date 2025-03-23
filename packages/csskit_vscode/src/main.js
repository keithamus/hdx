const vscode = require("vscode");
const { LanguageClient } = require("vscode-languageclient/node");

module.exports = { activate };

/**
 * @param {vscode.ExtensionContext} context
 * @returns Promise<void>
 */
async function activate(context) {
	const traceOutputChannel = vscode.window.createOutputChannel("csskit Language Server Trace");
	const env = Object.assign({}, process.env);
	const client = new LanguageClient(
		"csskit",
		"csskit Language Server",
		{
			run: {
				command: env.CSSKIT_SERVER_PATH || "csskit",
				args: ["lsp"],
				options: { env },
			},
			debug: {
				command: env.CSSKIT_SERVER_PATH || "csskit",
				args: ["--debug", "lsp"],
				options: { env },
			},
		},
		{
			documentSelector: [{ scheme: "file", language: "css" }],
			diagnosticCollectionName: "csskit",
			traceOutputChannel,
		},
	);
	context.subscriptions.push(client.start());
}
