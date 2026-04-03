import syntaxHighlight from "@11ty/eleventy-plugin-syntaxhighlight";
import { alert } from "@mdit/plugin-alert";
import path from "path";
import pluginRss, { dateToRfc3339, dateToRfc822 } from "@11ty/eleventy-plugin-rss";
import generateSocialImages from "@manustays/eleventy-plugin-generate-social-images";
import { globSync } from "glob";
import fs from "fs";
import htmlmin from "html-minifier";
import { build } from "esbuild";
import postcss from "postcss";
import postcssConfig from "./postcss.config.js";
import { wasmLoader } from "esbuild-plugin-wasm";

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
		postcss(postcssConfig.plugins)
			.process(css, { from: file, to: `_site/${file}` })
			.then((res) => {
				fs.mkdirSync("_site/css", { recursive: true });
				fs.writeFileSync(`_site/${file}`, res.css);
			});
	}
};

export default (eleventyConfig) => {
	const jsEntryPoints = globSync("script/*.[tj]s").map((p) => path.relative(process.cwd(), p));
	eleventyConfig.addWatchTarget("script/*.[tj]s");

	const cssEntryPoints = globSync("css/*.css").map((p) => path.relative(process.cwd(), p));
	eleventyConfig.addWatchTarget("css/*.css");

	buildJS({ entryPoints: jsEntryPoints });
	buildCSS({ entryPoints: cssEntryPoints });

	eleventyConfig.on("beforeWatch", (changedFiles) => {
		changedFiles = changedFiles.map((p) => path.relative(process.cwd(), p));
		// Run me before --watch or --serve re-runs
		if (changedFiles.some((watchPath) => jsEntryPoints.includes(watchPath))) {
			buildJS({ entryPoints: jsEntryPoints });
		}
		if (changedFiles.some((watchPath) => cssEntryPoints.includes(watchPath))) {
			buildCSS({ entryPoints: cssEntryPoints });
		}
	});

	eleventyConfig.addTransform("htmlmin", function (content) {
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

	eleventyConfig.amendLibrary("md", (mdLib) => mdLib.use(alert));

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
	eleventyConfig.addLiquidFilter("date_to_rfc3339", dateToRfc3339);
	eleventyConfig.addLiquidFilter("date_to_rfc822", dateToRfc822);

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
