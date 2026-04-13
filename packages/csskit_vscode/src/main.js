const vscode = require("vscode");
const { LanguageClient } = require("vscode-languageclient/node");
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");
const https = require("node:https");

const REPO = "csskit/csskit";

module.exports = { activate };

/**
 * @param {vscode.ExtensionContext} context
 * @returns {Promise<void>}
 */
async function activate(context) {
	const traceOutputChannel = vscode.window.createOutputChannel("csskit Language Server Trace");
	const storageDir = context.globalStorageUri.fsPath;
	await fsp.mkdir(storageDir, { recursive: true });

	const binaryPath = await ensureBinary(storageDir);

	const client = new LanguageClient(
		"csskit",
		"csskit Language Server",
		{
			run: {
				command: binaryPath,
				args: ["lsp"],
			},
			debug: {
				command: binaryPath,
				args: ["--debug", "lsp"],
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

/**
 * @returns {{ platform: string, arch: string, exe: string }}
 */
function platformInfo() {
	/** @type {string} */
	let platform;
	switch (process.platform) {
		case "darwin":
			platform = "darwin";
			break;
		case "linux":
			platform = "linux";
			break;
		case "win32":
			platform = "win32";
			break;
		default:
			throw new Error(`Unsupported platform: ${process.platform}`);
	}

	/** @type {string} */
	let arch;
	switch (process.arch) {
		case "arm64":
			arch = "arm64";
			break;
		case "x64":
			arch = "x64";
			break;
		default:
			throw new Error(`Unsupported architecture: ${process.arch}`);
	}

	const exe = process.platform === "win32" ? ".exe" : "";
	return { platform, arch, exe };
}

/**
 * @param {string} storageDir
 * @returns {Promise<string>}
 */
async function ensureBinary(storageDir) {
	const { platform, arch, exe } = platformInfo();
	const binaryPath = path.join(storageDir, `csskit${exe}`);
	const versionPath = path.join(storageDir, "version.txt");
	const assetName = `csskit-${platform}-${arch}`;

	const release = await fetchLatestRelease();
	const installedVersion = await readVersion(versionPath);

	if (installedVersion === release.tag_name && fs.existsSync(binaryPath)) {
		return binaryPath;
	}

	const asset = release.assets.find((a) => a.name === assetName);
	if (!asset) {
		throw new Error(`No asset found matching ${assetName}`);
	}

	await downloadFile(asset.browser_download_url, binaryPath);
	await fsp.chmod(binaryPath, 0o755);
	await fsp.writeFile(versionPath, release.tag_name, "utf-8");

	return binaryPath;
}

/**
 * @param {string} versionPath
 * @returns {Promise<string | null>}
 */
async function readVersion(versionPath) {
	try {
		const content = await fsp.readFile(versionPath, "utf-8");
		const trimmed = content.trim();
		return trimmed || null;
	} catch {
		return null;
	}
}

/**
 * @typedef {{ tag_name: string, assets: Array<{ name: string, browser_download_url: string }> }} GithubRelease
 */

/**
 * @returns {Promise<GithubRelease>}
 */
function fetchLatestRelease() {
	return new Promise((resolve, reject) => {
		const options = {
			hostname: "api.github.com",
			path: `/repos/${REPO}/releases/latest`,
			headers: {
				"User-Agent": "csskit-vscode",
				Accept: "application/vnd.github.v3+json",
			},
		};

		https
			.get(options, (res) => {
				if (res.statusCode !== 200) {
					reject(new Error(`GitHub API returned ${res.statusCode}`));
					res.resume();
					return;
				}
				let data = "";
				res.on("data", (chunk) => (data += chunk));
				res.on("end", () => {
					try {
						resolve(JSON.parse(data));
					} catch (e) {
						reject(e);
					}
				});
			})
			.on("error", reject);
	});
}

/**
 * @param {string} url
 * @param {string} dest
 * @returns {Promise<void>}
 */
function downloadFile(url, dest) {
	return new Promise((resolve, reject) => {
		/**
		 * @param {string} currentUrl
		 */
		function follow(currentUrl) {
			https
				.get(currentUrl, { headers: { "User-Agent": "csskit-vscode" } }, (res) => {
					if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
						follow(res.headers.location);
						res.resume();
						return;
					}
					if (res.statusCode !== 200) {
						reject(new Error(`Download failed with status ${res.statusCode}`));
						res.resume();
						return;
					}
					const file = fs.createWriteStream(dest);
					res.pipe(file);
					file.on("finish", () => file.close(() => resolve()));
					file.on("error", (e) => {
						fs.unlink(dest, () => {});
						reject(e);
					});
				})
				.on("error", reject);
		}
		follow(url);
	});
}
