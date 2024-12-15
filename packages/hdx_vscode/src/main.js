const vscode = require("vscode");
const lc = require("vscode-languageclient/node");

module.exports = { activate, deactivate };

async function deactivate() {}

/**
 * @param {vscode.ExtensionContext} context
 * @returns Promise<HdxExtensionContext>
 */
async function activate(context) {
	console.error("Extension active");
	const hdxContext = new HdxExtensionContext(context);
	await hdxContext.activate();
	return hdxContext;
}

class HdxExtensionContext {
	/**
	 * @public
	 * @readonly
	 * @type lc.LanguageClient
	 */
	client;

	/**
	 * @public
	 * @readonly
	 * @type vscode.ExtensionContext
	 */
	context;

	constructor(context) {
		this.context = context;
	}

	async activate() {
		this.client = await this.createClient();
		this.client.start();
	}

	async deactivate() {
		return this.client?.stop();
	}

	/**
	 * @returns Promise<lc.LanguageClient>
	 */
	async createClient() {
		const traceOutputChannel = vscode.window.createOutputChannel("hdx Language Server Trace");
		const env = Object.assign({}, process.env);
		const serverOptions = {
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
		};
		const clientOptions = {
			documentSelector: [{ scheme: "file", language: "css" }],
			diagnosticCollectionName: "hdx",
			traceOutputChannel,
		};

		console.log({ serverOptions, clientOptions });
		return new lc.LanguageClient("hdx", "hdx Language Server", serverOptions, clientOptions);
	}
}
