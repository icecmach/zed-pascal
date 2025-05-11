use zed_extension_api as zed;

struct PascalExtension {}

impl zed::Extension for PascalExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("pasls")
            .ok_or_else(|| "pasls not installed.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["--lowercase_intrinsics".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PascalExtension);
