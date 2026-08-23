// Resolves the native addon (csskit_napi).
//
// It ships in the platform `csskit-<platform>` packages. A local `./csskit.node` wins if present.

import { createRequire } from 'node:module';

const requireFrom = createRequire(import.meta.url);

function platformPackage() {
	const { platform, arch } = process;
	if (platform === 'linux') {
		if (arch === 'x64') return 'csskit-linux-x64';
		if (arch === 'arm64') return 'csskit-linux-arm64';
	} else if (platform === 'darwin') {
		if (arch === 'x64') return 'csskit-darwin-x64';
		if (arch === 'arm64') return 'csskit-darwin-arm64';
	} else if (platform === 'win32') {
		if (arch === 'x64') return 'csskit-win32-x64';
		if (arch === 'arm64') return 'csskit-win32-arm64';
	}
	return null;
}

function loadAddon() {
	const pkg = platformPackage();
	const candidates = pkg ? [`${pkg}/csskit.node`, './csskit.node'] : ['./csskit.node'];
	for (const spec of candidates) {
		try {
			return requireFrom(spec);
		} catch {
			// try the next candidate
		}
	}
	throw new Error(
		`csskit: no native addon for ${process.platform}-${process.arch}. ` +
			`Install the '${pkg || 'csskit-<platform>'}' package, or use the WebAssembly build at 'csskit/bundle'.`,
	);
}

const addon = loadAddon();

export default addon;
