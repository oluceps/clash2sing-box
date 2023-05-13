use anyhow::Result;
use clap::{Parser, Subcommand};
use ctos::ClashCfg;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,

    #[clap(short, long, help = "clash config file path")]
    source: Option<String>,

    #[clap(short, long, help = "clash subscription url")]
    url: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Show {
        #[clap(long, default_value = "false", help = "if show proxy name of all")]
        show_tags: bool,
    },
    Gen {
        #[clap(
            long,
            default_value = "./generated.json",
            help = "location of generated sing-box profile"
        )]
        output: String,
    },
    Append {
        #[clap(long, help = "location of sing-box profile to be append")]
        dst: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
