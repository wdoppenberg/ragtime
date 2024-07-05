use std::net::IpAddr;
use std::process::ExitCode;
use clap::{Parser};
use clap_derive::{Args, Subcommand};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use ragtime::error::AppResult;
use ragtime::utils::{port_in_range, shutdown_signal};
use ragtime::router::RouterArgs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	command: Command,
}


#[derive(Subcommand)]
pub enum Command {
	/// Start RAG server
	Server(ServerArgs),
	
	/// Terminal user interface
	Tui
}

#[derive(Args)]
pub struct ServerArgs {
	#[clap(flatten)]
	pub router_args: RouterArgs,

	#[arg(value_parser = port_in_range)]
	#[clap(short, long, default_value = "3000")]
	pub port: u16,

	#[clap(long, default_value = "127.0.0.1")]
	pub host: IpAddr,
}

#[tokio::main]
async fn main() -> AppResult<ExitCode> {
	let cli = Cli::parse();

	match cli.command {
		Command::Server(args) => {
			let router = args.router_args.build_router()?;

			tracing_subscriber::registry()
			    .with(fmt::layer())
			    .with(EnvFilter::from_default_env())
			    .init();
			
			    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
			
		    tracing::info!("listening on {}", listener.local_addr()?);
			
		    axum::serve(listener, router)
		        .with_graceful_shutdown(shutdown_signal(None))
		        .await?;
		},
		Command::Tui => {
			unimplemented!()
		}
	}

	Ok(ExitCode::SUCCESS)
}
