use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Review,
    New,
    Init,
    Compact,
    Undo,
    Diff,
    Mention,
    Status,
    Mcp,
    Logout,
    #[strum(serialize = "exit", serialize = "e")]
    Quit,
    Feedback,
    #[cfg(debug_assertions)]
    TestApproval,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => "send logs to maintainers",
            SlashCommand::New => "start a new chat during a conversation",
            SlashCommand::Init => "create an AGENTS.md file with instructions for Codex",
            SlashCommand::Compact => "summarize conversation to prevent hitting the context limit",
            SlashCommand::Review => "review my current changes and find issues",
            SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit => "exit Codex",
            SlashCommand::Diff => "show git diff (including untracked files)",
            SlashCommand::Mention => "mention a file",
            SlashCommand::Status => "show current session configuration and token usage",
            SlashCommand::Model => "choose what model and reasoning effort to use",
            SlashCommand::Approvals => "choose what Codex can do without approval",
            SlashCommand::Mcp => "list configured MCP tools",
            SlashCommand::Logout => "log out of Codex",
            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => "test approval request",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        match self {
            SlashCommand::Model => "model",
            SlashCommand::Approvals => "approvals",
            SlashCommand::Review => "review",
            SlashCommand::New => "new",
            SlashCommand::Init => "init",
            SlashCommand::Compact => "compact",
            SlashCommand::Undo => "undo",
            SlashCommand::Diff => "diff",
            SlashCommand::Mention => "mention",
            SlashCommand::Status => "status",
            SlashCommand::Mcp => "mcp",
            SlashCommand::Logout => "logout",
            SlashCommand::Quit => "quit",
            SlashCommand::Feedback => "feedback",
            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => "test-approval",
        }
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Approvals
            | SlashCommand::Review
            | SlashCommand::Logout => false,
            SlashCommand::Diff
            | SlashCommand::Mention
            | SlashCommand::Status
            | SlashCommand::Mcp
            | SlashCommand::Feedback
            | SlashCommand::Quit => true,

            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => true,
        }
    }

    /// Additional slash names that map to this command.
    pub fn aliases(self) -> &'static [&'static str] {
        match self {
            SlashCommand::Quit => &["exit", "e"],
            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => &[],
            _ => &[],
        }
    }

    /// Return true if `name` matches this command's canonical name or an alias.
    pub fn matches_name(self, name: &str) -> bool {
        if self.command() == name {
            return true;
        }
        self.aliases().contains(&name)
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter().map(|c| (c.command(), c)).collect()
}

/// Resolve a slash command name (including aliases) to the corresponding command.
pub fn resolve_slash_command(name: &str) -> Option<SlashCommand> {
    SlashCommand::iter().find(|cmd| cmd.matches_name(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn resolve_slash_command_supports_aliases() {
        assert_eq!(resolve_slash_command("quit"), Some(SlashCommand::Quit));
        assert_eq!(resolve_slash_command("exit"), Some(SlashCommand::Quit));
        assert_eq!(resolve_slash_command("e"), Some(SlashCommand::Quit));
        assert_eq!(resolve_slash_command("unknown"), None);
    }

    #[test]
    fn from_str_includes_aliases() {
        assert_eq!(SlashCommand::from_str("exit"), Ok(SlashCommand::Quit));
        assert_eq!(SlashCommand::from_str("e"), Ok(SlashCommand::Quit));
    }
}
