use app_lib::cli::Cli;
use clap::Parser;

fn main() {
  app_lib::run(Cli::parse())
}
