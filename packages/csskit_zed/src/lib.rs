use std::env;
use std::fs;
use zed_extension_api::{self as zed, Result};

// The bin at ~Library/Application Support/Zed/extensions/work/{NAME}
// this does not need to be a constant, and may choose to resolve it in the #language_server_binary_path method
const CSSKIT_BIN_PATH: &str = "csskit";

struct CsskitExtension;

impl CsskitExtension {
	fn language_server_binary_path(
		&mut self,
		language_server_id: &zed_extension_api::LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<String> {
		if let Ok(path) = env::var("CSSKIT_SERVER_PATH") {
			if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
				return Ok(path.to_string());
			}
		}

		if let Some(path) = worktree.which("csskit") {
			return Ok(path);
		}

		zed::set_language_server_installation_status(
			language_server_id,
			&zed::LanguageServerInstallationStatus::CheckingForUpdate,
		);

		let release = zed::github_release_by_tag_name("keithamus/csskit", "canary")?;

		let (platform, arch) = zed::current_platform();
		let asset_name = format!(
			"csskit-{platform}-{arch}",
			platform = match platform {
				zed::Os::Mac => "darwin",
				zed::Os::Linux => "linux",
				zed::Os::Windows => "win32",
			},
			arch = match arch {
				zed::Architecture::Aarch64 => "arm64",
				zed::Architecture::X8664 => "x64",
				_ => return Err(format!("unsupported architecture: {arch:?}")),
			},
		);

		let asset = release
			.assets
			.iter()
			.find(|asset| asset.name == asset_name)
			.ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

		if !fs::metadata(&asset_name).map_or(false, |stat| stat.is_file()) {
			zed::set_language_server_installation_status(
				language_server_id,
				&zed::LanguageServerInstallationStatus::Downloading,
			);

			zed::download_file(&asset.download_url, CSSKIT_BIN_PATH, zed::DownloadedFileType::Uncompressed)
				.map_err(|e| format!("failed to download file: {e}"))?;

			zed::make_file_executable(CSSKIT_BIN_PATH).map_err(|e| format!("failed to make file executable: {e}"))?;
		}

		Ok(CSSKIT_BIN_PATH.to_string())
	}
}

impl zed::Extension for CsskitExtension {
	fn new() -> Self {
		Self
	}

	fn language_server_command(
		&mut self,
		language_server_id: &zed_extension_api::LanguageServerId,
		worktree: &zed_extension_api::Worktree,
	) -> zed_extension_api::Result<zed_extension_api::Command> {
		let settings = zed_extension_api::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

		let mut args = vec![];

		if let Some(settings) = settings.settings {
			let is_debug = settings.get("debug").and_then(|value| value.as_bool()).unwrap_or(false);

			if is_debug {
				args.push("--debug".to_string());
			}
		}

		args.push("lsp".to_string());

		Ok(zed::Command {
			command: self.language_server_binary_path(language_server_id, worktree)?,
			args,
			env: Default::default(),
		})
	}
}

zed::register_extension!(CsskitExtension);
