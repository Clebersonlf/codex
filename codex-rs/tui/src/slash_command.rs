use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, AsRefStr, IntoStaticStr)]
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
    Quit,
    Feedback,
    #[cfg(debug_assertions)]
    TestApproval,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        self.spec().description
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.spec().canonical
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        self.spec().available_during_task
    }

    /// Additional slash names that map to this command.
    pub fn aliases(self) -> &'static [&'static str] {
        self.spec().aliases
    }

    /// Return true if `name` matches this command's canonical name or an alias.
    pub fn matches_name(self, name: &str) -> bool {
        if self.command() == name {
            return true;
        }
        self.aliases().contains(&name)
    }

    fn spec(self) -> &'static CommandSpec {
        // Small enum: linear scan is fine and keeps init simple.
        for s in SPECS {
            if s.cmd == self {
                return s;
            }
        }
        // Exhaustive over enum; unreachable.
        unreachable!("Unknown SlashCommand variant: {}", self.command());
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SPECS.iter().map(|s| (s.canonical, s.cmd)).collect()
}

/// Resolve a slash command name (including aliases) to the corresponding command.
pub fn resolve_slash_command(name: &str) -> Option<SlashCommand> {
    NAME_TO_CMD.get(name).copied()
}

impl std::str::FromStr for SlashCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAME_TO_CMD.get(s).copied().ok_or(())
    }
}

/// Central spec for all slash commands. Keeps metadata in one place.
struct CommandSpec {
    cmd: SlashCommand,
    canonical: &'static str,
    aliases: &'static [&'static str],
    description: &'static str,
    available_during_task: bool,
}

// Presentation order matches enum order.
static SPECS: &[CommandSpec] = &[
    CommandSpec {
        cmd: SlashCommand::Model,
        canonical: "model",
        aliases: &[],
        description: "choose what model and reasoning effort to use",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Approvals,
        canonical: "approvals",
        aliases: &[],
        description: "choose what Codex can do without approval",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Review,
        canonical: "review",
        aliases: &[],
        description: "review my current changes and find issues",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::New,
        canonical: "new",
        aliases: &[],
        description: "start a new chat during a conversation",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Init,
        canonical: "init",
        aliases: &[],
        description: "create an AGENTS.md file with instructions for Codex",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Compact,
        canonical: "compact",
        aliases: &[],
        description: "summarize conversation to prevent hitting the context limit",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Undo,
        canonical: "undo",
        aliases: &[],
        description: "ask Codex to undo a turn",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Diff,
        canonical: "diff",
        aliases: &[],
        description: "show git diff (including untracked files)",
        available_during_task: true,
    },
    CommandSpec {
        cmd: SlashCommand::Mention,
        canonical: "mention",
        aliases: &[],
        description: "mention a file",
        available_during_task: true,
    },
    CommandSpec {
        cmd: SlashCommand::Status,
        canonical: "status",
        aliases: &[],
        description: "show current session configuration and token usage",
        available_during_task: true,
    },
    CommandSpec {
        cmd: SlashCommand::Mcp,
        canonical: "mcp",
        aliases: &[],
        description: "list configured MCP tools",
        available_during_task: true,
    },
    CommandSpec {
        cmd: SlashCommand::Logout,
        canonical: "logout",
        aliases: &[],
        description: "log out of Codex",
        available_during_task: false,
    },
    CommandSpec {
        cmd: SlashCommand::Quit,
        canonical: "quit",
        aliases: &["exit"],
        description: "exit Codex",
        available_during_task: true,
    },
    CommandSpec {
        cmd: SlashCommand::Feedback,
        canonical: "feedback",
        aliases: &[],
        description: "send logs to maintainers",
        available_during_task: true,
    },
    #[cfg(debug_assertions)]
    CommandSpec {
        cmd: SlashCommand::TestApproval,
        canonical: "test-approval",
        aliases: &[],
        description: "test approval request",
        available_during_task: true,
    },
];

static NAME_TO_CMD: Lazy<HashMap<&'static str, SlashCommand>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, SlashCommand> = HashMap::new();
    for spec in SPECS {
        m.insert(spec.canonical, spec.cmd);
        for a in spec.aliases {
            m.insert(*a, spec.cmd);
        }
    }
    m
});

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
