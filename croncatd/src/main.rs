//!
//! The `croncatd` agent.
//!

use std::process::exit;

use croncat::{
    channels,
    config::ChainConfig,
    env,
    errors::Report,
    grpc::{GrpcQuerier, GrpcSigner},
    logging::{self, info},
    store::agent::LocalAgentStorage,
    system, tokio,
    tokio::runtime::Runtime,
};

use crate::cli::deposit_junox;

mod cli;
mod opts;

///
/// Start the `croncatd` agent.
///

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Get environment variables
    let env = env::load()?;
    let mut storage = LocalAgentStorage::new();

    // Setup tracing and error reporting
    logging::setup()?;

    // Get the CLI options, handle argument errors nicely
    let opts = cli::get_opts()
        .map_err(|e| {
            println!("{}", e);
            exit(1);
        })
        .unwrap();

    // If there ain't no no-frills...
    if !opts.no_frills {
        cli::print_banner();
    }
    info!("Starting croncatd...");
    match opts.cmd {
        opts::Command::RegisterAgent {
            payable_account_id,
            sender_name,
        } => {
            let key = storage.get_agent_signing_key(&sender_name)?;
            println!("key {:?}", key.public_key());

            println!("Account Id {:?}", payable_account_id);
            let signer = GrpcSigner::new(ChainConfig::new()?, key, env.croncat_addr).await?;
            let result = signer.register_agent(payable_account_id).await?;
            let log = result.log;
            println!("{log}");
        }
        opts::Command::UnregisterAgent { sender_name } => {
            let key = storage.get_agent_signing_key(&sender_name)?;
            let signer = GrpcSigner::new(ChainConfig::new()?, key, env.croncat_addr).await?;
            let result = signer.unregister_agent().await?;
            let log = result.log;
            println!("{log}");
        }
        opts::Command::Withdraw { sender_name } => {
            let key = storage.get_agent_signing_key(&sender_name)?;
            let signer = GrpcSigner::new(ChainConfig::new()?, key, env.croncat_addr).await?;
            let result = signer.withdraw_reward().await?;
            let log = result.log;
            println!("{log}");
        }
        opts::Command::Info => {
            let querier = GrpcQuerier::new(&env.croncat_addr, &ChainConfig::new()?).await?;
            let config = querier.query_config().await?;
            println!("{config}")
        }
        opts::Command::GetAgentStatus { account_id } => {
            let querier = GrpcQuerier::new(&env.croncat_addr, &ChainConfig::new()?).await?;
            let status = querier.get_agent(account_id).await?;
            println!("{status}")
        }
        opts::Command::Tasks { from_index, limit } => {
            let querier = GrpcQuerier::new(&env.croncat_addr, &ChainConfig::new()?).await?;
            let tasks = querier.get_tasks(from_index, limit).await?;
            println!("{tasks}")
        }
        opts::Command::GetAgentTasks { account_addr } => {
            let querier = GrpcQuerier::new(&env.croncat_addr, &ChainConfig::new()?).await?;
            let agent_tasks = querier.get_agent_tasks(account_addr).await?;
            println!("{agent_tasks}")
        }
        opts::Command::GenerateMnemonic { new_name } => storage.generate_account(new_name)?,
        opts::Command::UpdateAgent {
            payable_account_id,
            sender_name,
        } => {
            let key = storage.get_agent_signing_key(&sender_name)?;
            let signer = GrpcSigner::new(ChainConfig::new()?, key, env.croncat_addr).await?;
            let result = signer.update_agent(payable_account_id).await?;
            let log = result.log;
            println!("{log}");
        }
        opts::Command::DepositUjunox { account_id } => {
            let result = deposit_junox(&account_id).await?;
            println!("{:?}", result);
        }
        opts::Command::GetAgent { name } => storage.display_account(&name),
        _ => {
            // Create a channel to handle graceful shutdown and wrap receiver for cloning
            let (shutdown_tx, shutdown_rx) = channels::create_shutdown_channel();

            // Start the agent
            Runtime::new()
                .unwrap()
                .block_on(async { system::run(env, shutdown_tx, shutdown_rx).await })?;
        }
    }

    // Say goodbye if no no-frills
    if !opts.no_frills {
        println!("\n🐱 Cron Cat says: Goodbye / さようなら\n");
    }

    Ok(())
}
