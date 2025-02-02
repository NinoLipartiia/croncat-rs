//!
//! `croncatd` CLI option builder.
//!

use croncat::utils::DEFAULT_AGENT_ID;
use enum_display::EnumDisplay;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "croncatd", about = "The croncat agent daemon.")]
pub struct Opts {
    /// Debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Whether to print nice little things like the banner and a goodbye
    #[structopt(long)]
    pub no_frills: bool,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: Command,

    /// Chain ID of the chain to connect to
    #[structopt(long, global = true, env = "CRONCAT_CHAIN_ID")]
    pub chain_id: Option<String>,

    /// ID of the agent config to use
    #[structopt(long, global = true, default_value = DEFAULT_AGENT_ID, env = "CRONCAT_AGENT")]
    pub agent: String,
}

#[derive(Debug, StructOpt, Clone, EnumDisplay)]
#[enum_display(case = "Kebab")]
pub enum Command {
    /// Registers an agent, placing them in the pending queue unless it's the first agent.
    Register { payable_account_id: Option<String> },

    /// Get the agent's supported bech32 accounts
    ListAccounts,

    /// Get the agent's status (pending/active)
    Status,

    /// Get the agent's tasks they're assigned to fulfill
    GetTasks,

    /// Unregisters the agent from being in the queue with other agents
    Unregister,

    /// Update the agent's configuration
    Update,

    /// Withdraw the agent's funds to the payable account ID
    Withdraw,

    /// Get contract's state
    // #[cfg(feature = "debug")]
    // GetState {
    //     from_index: Option<u64>,
    //     limit: Option<u64>,
    // },

    /// Show all task(s) information
    AllTasks {
        from_index: Option<u64>,
        limit: Option<u64>,
    },

    /// Starts the Croncat agent, allowing it to fulfill tasks
    Go {
        /// Allow agent to do tasks with queries, uses more computer resources
        #[structopt(long, short = "q")]
        with_queries: bool,
    },

    /// Generates a new keypair and agent account (good first step)
    GenerateMnemonic {
        /// The agent's name
        new_name: String,

        /// Recover agent from mnemonic phrase. Please do not use your own account!
        #[structopt(long)]
        mnemonic: Option<String>,
    },

    /// [SENSITIVE!] Shows all details about agents on this machine
    GetAgent {
        #[structopt(long, default_value = "agent", env = "CRONCAT_AGENT")]
        name: String,
    },

    /// Setup an agent as a system service (systemd)
    SetupService {
        #[structopt(long)]
        output: Option<String>,
    },
}
