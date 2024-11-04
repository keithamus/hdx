import { DOMParser } from "jsr:@b-fuze/deno-dom";

const ucfirst = (name: string) => name[0].toUpperCase() + name.slice(1);
const camel = (name: string) => name.replace(/([_-\s]\w)/g, (n) => n.slice(1).toUpperCase());
const pascal = (name: string) => ucfirst(camel(name));
const snake = (name: string) => name.replace(/([_-\s]\w)/g, (n) => `_${n.slice(1)}`).toLowerCase();

// Some properties should have lifetime annotations. It's a little tricky to detect which ones
// so it's easier just to hardcode these as a list...
const requiresAllocatorLifetime = new Map([
]);

// Some properties should be enums but they have complex grammars that aren't worth attempting to
// parse so let's just hardcode a list...
const enumOverrides = new Map([
	["animation", new Set(["AnimationName"])],
	["ui", new Set(["Cursor"])],
	["overscroll", new Set(["OverscrollBehavior"])],
]);

// Ignore properties from some specs as they've moved around or are very rough
const ignore = new Map([
	// https://drafts.csswg.org/css-ui-4/#changes-22-12-2017
	// Moved the box-sizing and text-overflow properties to [CSS-SIZING-3] and [CSS-OVERFLOW-4] respectively.
	["ui", new Set(["box-sizing", "text-overflow"])],
	// CSS Shapes [CSS-SHAPES-2] define the shape-inside property that aligns contents along the edge of a possibly non-rectangular wrapping area.
	// (Round-Display just extends to add the `display` keyword which is specified in shapes-2 anyway)
	["round-display", new Set(["shape-inside"])],
	[
		"background",
		new Set([
			// https://drafts.csswg.org/css-backgrounds-4/#background-layers
			// The name of this property is discussed in issue https://github.com/w3c/csswg-drafts/issues/9083.
			"background-tbd",
			// https://drafts.csswg.org/css-borders-4/#intro
			//  This module is currently maintained as a diff against the parts related to borders and box
			//  decorations of CSS Backgrounds and Borders Module Level 3 [CSS3BG]. We will fold in the text
			//  once it’s all formatted up and in CR again, as this will reduce the effort of keeping them in
			//  sync (source diffs will be accurate in reflecting the differences).
			// (IOW these are all defined in CSS Borders 4)
			"border-color",
			"border-top-color",
			"border-right-color",
			"border-bottom-color",
			"border-left-color",
			"border-style",
			"border-top-style",
			"border-right-style",
			"border-bottom-style",
			"border-left-style",
			"border-width",
			"border-top-width",
			"border-right-width",
			"border-bottom-width",
			"border-left-width",
			"border",
			"border-top",
			"border-right",
			"border-bottom",
			"border-left",
			"border-radius",
			"border-top-radius",
			"border-right-radius",
			"border-bottom-radius",
			"border-left-radius",
			"box-shadow",
		]),
	],
]);

async function getIndex() {
	try {
		return JSON.parse(await Deno.readTextFile("./.index-cache.json"));
	} catch {}
	const url = "https://api.github.com/repos/w3c/csswg-drafts/git/trees/main";
	console.log(`Fetching ${url}...`);
	const res = await fetch(url);
	const json = await res.json();
	const index = json.tree.reduce((acc: Record<string, number>, { path, type }) => {
		if (type == "tree" && path.startsWith("css-")) {
			let parts = path.split(/-/g).slice(1);
			let i = Number(parts.pop());
			const index = parts.join("-");
			acc[index] ||= [];
			acc[index].push(i);
		}
		return acc;
	}, {});
	await Deno.writeTextFile("./.index-cache.json", JSON.stringify(index, null, 2));
	return index;
}

async function fetchSpec(name: string, ver: number) {
	try {
		return await Deno.readTextFile(`./.${name}-${ver}-cache.txt`);
	} catch {}
	const url = `https://drafts.csswg.org/css-${name}-${ver}/`;
	console.log(`Fetching ${url}...`);
	const res = await fetch(url);
	const text = await res.text();
	await Deno.writeTextFile(`./.${name}-${ver}-cache.txt`, text);
	return text;
}

async function getSpec(name: string, index: Record<string, number[]>) {
	const types = new Map();
	let url = "";
	let title = "";
	for (const i of index[name]) {
		url = `https://drafts.csswg.org/css-${name}-${i}/`;
		const document = new DOMParser().parseFromString(await fetchSpec(name, i), "text/html");
		const propertyIndexHeader = document.querySelectorAll("#property-index");
		if (!propertyIndexHeader) {
			console.error(`${name}-${i} has no properties`);
			continue;
		}
		const index = document.querySelectorAll("#property-index + .big-element-wrapper table.index");
		if (index.length != 1) {
			console.error(`saw ${index.length} index tables in ${name}-${i}. Refusing to go further`);
			continue;
		}
		title = document.querySelector("h1")?.textContent || "";
		const propTables = [...document.querySelectorAll("table.propdef")]
			.flatMap((table) => {
				const newTable = Object.fromEntries(
					[...table.querySelectorAll("tr")].map((e) => [
						snake(e.querySelector("th").textContent.trim().slice(0, -1)),
						e.querySelector("td").textContent.trim(),
					]),
				);
				const names = newTable.name.split(/\s*,\s*/g);
				return names.map((name) => ({ ...newTable, name }));
			})
			.filter((e) => !e.new_values);
		for (const table of propTables) {
			if (!ignore.get(name)?.has(table.name)) {
				types.set(table.name, table);
			}
		}
	}

	const typeDefs = [...types.values()].map((table) => {
		let dataType = "struct";
		const enums = enumOverrides.get(name);
		if (
			enums?.has(table.name) ||
			/[^\|]\|[^\|]/.test(table.value.replace(/(?:\[[^\]]+\])g/, "").replace(/(?:<[^>]+>)g/, ""))
		) {
			dataType = "enum";
		}
		let trail = dataType == "enum" ? " {}" : ";";
		let generics = "";
		const lifetimes = requiresAllocatorLifetime.get(name);
		if (lifetimes?.has(table.name) || table.value.includes("<string>") || table.value.includes("<image>")) {
			generics = "<'a>";
		}
		return `
// ${url}#${table.name == "--*" ? "defining-variables" : table.name}
#[value(" ${table.value} ")]
#[initial("${table.initial}")]
#[applies_to("${table.applies_to}")]
#[inherited("${table.inherited.toLowerCase()}")]
#[percentages("${table.percentages.toLowerCase()}")]
#[canonical_order("${table.canonical_order.toLowerCase()}")]
#[animation_type("${table.animation_type?.toLowerCase() ?? "not animatable"}")]
pub ${dataType} ${table.name == "--*" ? "Custom" : pascal(table.name)}${generics}${trail}`;
	});

	if (typeDefs.length == 0) return "";
	return `mod impls;
pub mod types;

use impls::*;

/*
 * ${url}
 * ${title}
 */
${typeDefs.join("\n")}
`;
}

(async (name) => {
	const index = await getIndex();
	if (!name || !index[name]) {
		throw new Error("supply a working draft name");
	}
	const rs = await getSpec(name, index);
	// console.log(rs);
	if (!rs) {
		try {
			await Deno.remove(`../../crates/hdx_ast/src/css/values/${snake(name)}/`, { recursive: true });
		} catch {}
	} else {
		await Deno.mkdir(`../../crates/hdx_ast/src/css/values/${snake(name)}/`, { recursive: true });
		await Deno.writeTextFile(`../../crates/hdx_ast/src/css/values/${snake(name)}/mod.rs`, rs);
	}
})(...Deno.args);
