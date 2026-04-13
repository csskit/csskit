use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const REPO: &str = "csskit/csskit";
const VERSION_FILE: &str = "version.txt";

fn binary_name(platform: zed::Os) -> &'static str {
	match platform {
		zed::Os::Windows => "csskit.exe",
		zed::Os::Mac | zed::Os::Linux => "csskit",
	}
}

fn asset_name(platform: zed::Os, arch: zed::Architecture) -> Result<String> {
	Ok(format!(
		"csskit-{platform}-{arch}",
		platform = match platform {
			zed::Os::Mac => "darwin",
			zed::Os::Linux => "linux",
			zed::Os::Windows => "win32",
		},
		arch = match arch {
			zed::Architecture::Aarch64 => "arm64",
			zed::Architecture::X8664 => "x64",
			zed::Architecture::X86 => return Err("x86 (32-bit) is not supported".into()),
		},
	))
}

fn needs_download(installed_version: Option<&str>, release_version: &str, binary_exists: bool) -> bool {
	match installed_version {
		Some(v) if v == release_version && binary_exists => false,
		_ => true,
	}
}

fn find_asset<'a>(assets: &'a [zed::GithubReleaseAsset], expected: &str) -> Result<&'a zed::GithubReleaseAsset> {
	assets.iter().find(|a| a.name == expected).ok_or_else(|| format!("no asset found matching {expected}"))
}

fn installed_version() -> Option<String> {
	let version = fs::read_to_string(VERSION_FILE).ok()?;
	let version = version.trim();
	if version.is_empty() {
		None
	} else {
		Some(version.to_string())
	}
}

struct CsskitExtension;

impl CsskitExtension {
	fn language_server_binary_path(&self, language_server_id: &LanguageServerId) -> Result<String> {
		let (platform, arch) = zed::current_platform();
		let binary_path = binary_name(platform).to_string();

		zed::set_language_server_installation_status(
			language_server_id,
			&zed::LanguageServerInstallationStatus::CheckingForUpdate,
		);

		let release =
			zed::latest_github_release(REPO, zed::GithubReleaseOptions { require_assets: true, pre_release: false })?;

		let binary_exists = fs::metadata(&binary_path).is_ok_and(|m| m.is_file());

		if !needs_download(installed_version().as_deref(), &release.version, binary_exists) {
			return Ok(binary_path);
		}

		let asset = find_asset(&release.assets, &asset_name(platform, arch)?)?;

		zed::set_language_server_installation_status(
			language_server_id,
			&zed::LanguageServerInstallationStatus::Downloading,
		);

		let tmp_path = format!("{binary_path}.tmp");
		let _ = fs::remove_file(&tmp_path);

		zed::download_file(&asset.download_url, &tmp_path, zed::DownloadedFileType::Uncompressed)
			.map_err(|e| format!("failed to download file: {e}"))?;

		zed::make_file_executable(&tmp_path)?;

		fs::rename(&tmp_path, &binary_path).map_err(|e| format!("failed to install binary: {e}"))?;

		let _ = fs::write(VERSION_FILE, &release.version);

		Ok(binary_path)
	}
}

impl zed::Extension for CsskitExtension {
	fn new() -> Self {
		Self
	}

	fn language_server_command(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<zed::Command> {
		let settings = zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

		let mut args = vec![];

		if let Some(settings) = settings.settings {
			let is_debug = settings.get("debug").and_then(|value| value.as_bool()).unwrap_or(false);

			if is_debug {
				args.push("--debug".to_string());
			}
		}

		args.push("lsp".to_string());

		Ok(zed::Command {
			command: self.language_server_binary_path(language_server_id)?,
			args,
			env: Default::default(),
		})
	}
}

zed::register_extension!(CsskitExtension);

#[cfg(test)]
mod tests {
	use super::*;
	use zed_extension_api::{self as zed};

	#[test]
	fn test_binary_name() {
		assert_eq!(binary_name(zed::Os::Mac), "csskit");
		assert_eq!(binary_name(zed::Os::Linux), "csskit");
		assert_eq!(binary_name(zed::Os::Windows), "csskit.exe");
	}

	#[test]
	fn test_asset_name() {
		assert_eq!(asset_name(zed::Os::Mac, zed::Architecture::Aarch64).unwrap(), "csskit-darwin-arm64");
		assert_eq!(asset_name(zed::Os::Mac, zed::Architecture::X8664).unwrap(), "csskit-darwin-x64");
		assert_eq!(asset_name(zed::Os::Linux, zed::Architecture::Aarch64).unwrap(), "csskit-linux-arm64");
		assert_eq!(asset_name(zed::Os::Linux, zed::Architecture::X8664).unwrap(), "csskit-linux-x64");
		assert_eq!(asset_name(zed::Os::Windows, zed::Architecture::Aarch64).unwrap(), "csskit-win32-arm64");
		assert_eq!(asset_name(zed::Os::Windows, zed::Architecture::X8664).unwrap(), "csskit-win32-x64");
	}

	#[test]
	fn test_asset_name_x86_unsupported() {
		let result = asset_name(zed::Os::Linux, zed::Architecture::X86);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "x86 (32-bit) is not supported");
	}

	#[test]
	fn test_needs_download_no_installed_version() {
		assert!(needs_download(None, "v1.0.0", false));
		assert!(needs_download(None, "v1.0.0", true));
	}

	#[test]
	fn test_needs_download_version_mismatch() {
		assert!(needs_download(Some("v1.0.0"), "v2.0.0", true));
		assert!(needs_download(Some("v1.0.0"), "v2.0.0", false));
	}

	#[test]
	fn test_needs_download_version_matches_but_no_binary() {
		assert!(needs_download(Some("v1.0.0"), "v1.0.0", false));
	}

	#[test]
	fn test_needs_download_version_matches_and_binary_exists() {
		assert!(!needs_download(Some("v1.0.0"), "v1.0.0", true));
	}

	#[test]
	fn test_find_asset_found() {
		let assets = vec![
			zed::GithubReleaseAsset {
				name: "csskit-darwin-arm64".to_string(),
				download_url: "https://example.com/darwin-arm64".to_string(),
			},
			zed::GithubReleaseAsset {
				name: "csskit-linux-x64".to_string(),
				download_url: "https://example.com/linux-x64".to_string(),
			},
		];

		let asset = find_asset(&assets, "csskit-linux-x64").unwrap();
		assert_eq!(asset.download_url, "https://example.com/linux-x64");
	}

	#[test]
	fn test_find_asset_not_found() {
		let assets = vec![zed::GithubReleaseAsset {
			name: "csskit-darwin-arm64".to_string(),
			download_url: "https://example.com/darwin-arm64".to_string(),
		}];

		let result = find_asset(&assets, "csskit-win32-x64");
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "no asset found matching csskit-win32-x64");
	}
}
