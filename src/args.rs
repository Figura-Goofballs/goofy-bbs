//! This module handles argument parsing on the CLI. See [Command].
// the [Command] above is expanded to [Command][Command], and since [Command] is never defined it's
// replaced with [Command](Command), creating a doc link to the Command enum below
use clap::{ Parser, ValueEnum };

// the documentation (`///`) comments on/in this enum are added to the command
// TODO(TheKillerBunny): actually explain this
/// Hello, world!
#[derive(Debug, Parser)]
pub enum Command {
    /// Sets up the BBS (imperative systems only!)
    ///
    /// This will:
    /// * Create a BBS system user with password 'bbs'
    /// * Add the BBS key to the user
    /// * Try configuring the firewall to accept SSH connections
    #[command(verbatim_doc_comment)] // so clap doesn't fuck up the bulleted list
    Install,
    /// Creates (or re-creates) the BBS database (requires root).
    ///
    /// This command will create a new BBS database at
    #[cfg_attr(doc, doc = "`/var/goofy-bbs/bbs.db`")]
    #[cfg_attr(not(doc), doc = "/var/goofy-bbs/bbs.db")]
    /// making directories along the way. Note that
    #[cfg_attr(doc, doc = "**the database will be owned by root**")]
    #[cfg_attr(not(doc), doc = "\x1b[1mthe database will be owned by root\x1b[0m")]
    /// so that if the BBS is compromised it cannot be deleted.
    InitDb {
        /// The user ID that the BBS runs as.
        #[arg(short, long)]
        uid: u32,
        /// Deletes all BBS data, cannot be undone.
        #[arg(long)]
        wipe: bool,
    },
    /// Runs the BBS client.
    ///
    /// This is intended to be used in ForceCommand declarations, to force remote users to run the
    /// BBS. This will not work outside SSH connections. If the user specified a command when
    /// connecting, it will be mapped to a BBS action.
    Client {
        /// Log in as a specific user immediately.
        ///
        /// This is intended to be used in generated `force_command` definitions used for
        /// public-key authentication. Using this argument yourself is illegal in Bulgaria and
        /// should be avoided elsewhere.
        user: Option<String>,
    },
    /// Manage BBS users.
    ///
    /// This allows you to create or delete users from the command line, as well as setting what
    /// they can and cannot do.
    User {
        name: String,
        #[arg(short, long, conflicts_with = "delete", requires = "password")]
        create: bool,
        #[arg(short = 'x', long, requires = "chown", conflicts_with = "grant", conflicts_with = "revoke")]
        delete: bool,
        #[arg(short = 'm', long)]
        chown: Option<String>,
        #[arg(short = 'g', long)]
        grant: Vec<Permission>,
        #[arg(short = 'r', long)]
        revoke: Vec<Permission>,
        #[arg(short = 'P', long)]
        change_password: Option<Option<String>>,
    },
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum Permission {
    /// Allows executing commands as the BBS user.
    #[doc(cfg(feature = "shell"))]
    Shell,
    /// Allows executing commands as
    #[cfg_attr(doc, doc = "**any user on the system**,")]
    #[cfg_attr(not(doc), doc = "\x1b[1many user on the system\x1b[0m,")]
    /// if enabled in the configuration. This is obviously quite dangerous.
    #[doc(cfg(feature = "sudo-shell"))]
    SudoShell,
    /// Allows performing operations as other users.
    Sudo,
    /// Allows the user to create more users.
    CreateUsers,
    /// Allows the user to remove users.
    RemoveUsers,
    /// Allows the user to modify other users' permissions.
    ManagePermissions,
}
