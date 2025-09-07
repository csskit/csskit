use crate::{CliResult, GlobalConfig};
use clap::{Args, crate_version};
use csskit_lsp::{LSPService, Server};
use std::io::stderr;
use tracing::{level_filters::LevelFilter, trace};
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

/// Run the LSP server. It's unlikely you want to run this, but your IDE might!
#[derive(Debug, Args)]
pub struct Lsp;

impl Lsp {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let GlobalConfig { debug, .. } = config;
		let Lsp {} = self;
		let server = Server::new(LSPService::new(crate_version!()));
		let stderr_log =
			fmt::layer().with_writer(stderr).with_filter(if debug { LevelFilter::TRACE } else { LevelFilter::WARN });
		registry().with(stderr_log).with(server.tracer()).init();
		let thread = server.listen_stdio()?;
		trace!("Listening on stdin/stdout");
		thread.sender.join().expect("Couldn't start server").ok();
		Ok(())
	}
}
