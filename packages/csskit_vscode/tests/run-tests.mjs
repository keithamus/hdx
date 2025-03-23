import Mocha from "mocha";
import * as path from "path";
import { glob } from "tinyglobby";

import { runTests } from "@vscode/test-electron";

async function main() {
	try {
		const extensionDevelopmentPath = path.resolve(__dirname, "../");
		const extensionTestsPath = __filename;
		await runTests({ extensionDevelopmentPath, extensionTestsPath });
	} catch {
		console.error("Failed to run tests");
		process.exit(1);
	}
}

export async function run() {
	const mocha = new Mocha({ ui: "bdd" });
	const testsRoot = path.resolve(__dirname, "../out/tests/");
	const files = await glob("*.test.js", { cwd: testsRoot });
	for (const file of files) {
		mocha.addFile(path.resolve(testsRoot, file));
	}
	await new Promise((resolve, reject) => {
		mocha.run((failures) => {
			if (failures > 0) {
				reject(new Error(`${failures} tests failed.`));
			} else {
				resolve();
			}
		});
	});
}

main();
