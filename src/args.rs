#![deny(missing_doc)]
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Command {
    /// Sets up the BBS (imperative systems only!)
    ///
    /// This will:
    /// • Create a BBS system user with password 'bbs'
    /// • Add the BBS key to the user
    /// • Try configuring the firewall to accept SSH connections
    #[command(verbatim_doc_comment)] // so clap doesn't fuck up the bulleted list
    Install,
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
}
