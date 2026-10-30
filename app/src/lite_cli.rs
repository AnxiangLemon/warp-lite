use clap::Parser;

#[derive(Debug, Default, Parser)]
#[command(name = "warp-lite", version = "0.1.0", about = "轻量本地终端")]
pub(super) struct Args {
    #[command(flatten)]
    pub app: warp_cli::AppArgs,
    #[arg(long = "dump-debug-info", hide = true)]
    pub dump_debug_info: bool,
    #[cfg(all(unix, feature = "local_tty"))]
    #[command(subcommand)]
    pub worker: Option<Worker>,
}

#[cfg(all(unix, feature = "local_tty"))]
#[derive(Debug, clap::Subcommand)]
pub(super) enum Worker {
    #[command(name = "terminal-server", hide = true)]
    TerminalServer(warp_cli::TerminalServerArgs),
}

impl Args {
    pub fn from_env() -> Self {
        #[cfg(target_family = "wasm")]
        {
            Self::default()
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Self::try_parse().unwrap_or_else(|error| {
                #[cfg(windows)]
                warp_util::windows::attach_to_parent_console();
                error.exit()
            })
        }
    }
}

#[cfg(test)]
#[path = "lite_cli_tests.rs"]
mod tests;
