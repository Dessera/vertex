#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[clap(subcommand)]
  command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
  /// Check environment
  Check,
}
