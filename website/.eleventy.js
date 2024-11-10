const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const pluginRss = require("@11ty/eleventy-plugin-rss");
const generateSocialImages = require("@manustays/eleventy-plugin-generate-social-images");
const glob = require("glob");
const fs = require("fs");
const htmlmin = require("html-minifier");
const { build } = require("esbuild");
const postcss = require("postcss");
const postcssConfig = require("./postcss.config.js");
const { wasmLoader } = require("esbuild-plugin-wasm");
const { css } = require("@codemirror/lang-css");

const buildJS = (config = {}) => {
	return build({
		minify: process.NODE_ENV === "development" ? false : true,
		bundle: true,
		splitting: true,
		write: true,
		format: "esm",
		metafile: true,
		outdir: "_site/script",
		plugins: [wasmLoader()],
		...config,
	});
};

const buildCSS = (config = {}) => {
	for (const file of config.entryPoints) {
		const css = fs.readFileSync(file, "utf-8");
		let res = postcss(postcssConfig.plugins).process(css, { from: file, to: `_site/${file}` }).then(res => {
			fs.mkdirSync('_site/css', { recursive: true });
			fs.writeFileSync(`_site/${file}`, res.css);
		});
	}
};

module.exports = (eleventyConfig) => {
	eleventyConfig.addPlugin(css);

	const jsEntryPoints = glob.sync("src/*.[tj]s");
	eleventyConfig.addWatchTarget("src/*.[tj]s");

	const cssEntryPoints = glob.sync("css/*.css");
	eleventyConfig.addWatchTarget("css/*.css");

	buildJS({ entryPoints: jsEntryPoints });
	buildCSS({ entryPoints: cssEntryPoints });

	eleventyConfig.on("beforeWatch", (changedFiles) => {
		// Run me before --watch or --serve re-runs
		if (changedFiles.some((watchPath) => jsEntryPoints.includes(watchPath))) {
			buildJS({ entryPoints: jsEntryPoints });
		}
		if (changedFiles.some((watchPath) => cssEntryPoints.includes(watchPath))) {
			buildCSS({ entryPoints: cssEntryPoints });
		}
	});

	eleventyConfig.addTransform("htmlmin", function (content) {
		// Prior to Eleventy 2.0: use this.outputPath instead
		if (this.page.outputPath && this.page.outputPath.endsWith(".html")) {
			let minified = htmlmin.minify(content, {
				useShortDoctype: true,
				removeComments: true,
				collapseBooleanAttributes: true,
				collapseWhitespace: true,
			});
			return minified;
		}
		return content;
	});

	eleventyConfig.addPlugin(generateSocialImages, {
		hideTerminal: false,
		outputDir: "./_site/images/preview",
		urlPath: "https://hdxcss.dev/images/preview",
		siteName: "hdxcss.dev",
		titleColor: "",
		customFontFileName: "Inter-Black.ttf",
		customSVG: ``,
		bgGradient: ["#f8fafb", "#f8fafb"],
	});
	eleventyConfig.addPlugin(syntaxHighlight);
	eleventyConfig.addPlugin(pluginRss);
	eleventyConfig.addLiquidFilter("date_to_rfc3339", pluginRss.dateToRfc3339);
	eleventyConfig.addLiquidFilter("date_to_rfc822", pluginRss.dateToRfc822);

	eleventyConfig.ignores.add("js");
	eleventyConfig.ignores.add("css");
	eleventyConfig.ignores.add("fonts");
	eleventyConfig.ignores.add("images");
	eleventyConfig.ignores.add("examples");
	eleventyConfig.addPassthroughCopy("js");
	eleventyConfig.addPassthroughCopy("fonts");
	eleventyConfig.addPassthroughCopy("playground/*.wasm");
	eleventyConfig.addPassthroughCopy("images");
	eleventyConfig.addPassthroughCopy("examples");
	eleventyConfig.addPassthroughCopy("favicon.ico");
	eleventyConfig.addPassthroughCopy("favicon.png");
	return {
		dir: {
			layouts: "_layouts",
			includes: "_includes",
		},
	};
};
