const css = require("eleventy-postcss-extension");
const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const pluginRss = require("@11ty/eleventy-plugin-rss");
const generateSocialImages = require("@manustays/eleventy-plugin-generate-social-images");
const glob = require("glob");
const htmlmin = require("html-minifier");
const { build } = require("esbuild");
const { wasmLoader } = require("esbuild-plugin-wasm");

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

module.exports = (eleventyConfig) => {
	eleventyConfig.addPlugin(css);

	const entryPoints = glob.sync("src/*.[tj]s");
	eleventyConfig.addWatchTarget("src/*.[tj]s");

	buildJS({ entryPoints });

	eleventyConfig.on("beforeWatch", (changedFiles) => {
		// Run me before --watch or --serve re-runs
		if (changedFiles.some((watchPath) => entryPoints.includes(watchPath))) {
			buildJS({ entryPoints });
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
