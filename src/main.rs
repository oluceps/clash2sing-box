use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use ctos::ClashCfg;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,

    #[clap(short, long, help = "clash config file path(url)")]
    source: Option<String>,

    #[clap(short, long, help = "clash subscription url")]
    url: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Show sing-box proxies info from clash profile")]
    Show {
        #[clap(
            long,
            short = 't',
            default_value = "false",
            help = "if show proxy name of all"
        )]
        show_tags: bool,
    },
    #[clap(about = "Generate sing-box profile from clash format")]
    Gen {
        #[clap(
            long,
            default_value = "./generated.json",
            help = "location of generated sing-box profile"
        )]
        output: String,
    },
    #[clap(about = "Append new clash proxies to existed sing-box profile")]
    Append {
        #[clap(long, help = "location of sing-box profile to be append")]
        dst: String,
    },
}

impl Args {
    fn ayaya(&self) -> Result<()> {
        match &self.cmd {
            Command::Show { show_tags } => {
                let source = self.source.clone().expect("checked");
                let cfg: ClashCfg = if self.source.as_ref().unwrap().starts_with("http") {
                    ClashCfg::new_from_subscribe_link(self.source.as_ref().unwrap().as_str())?
                } else {
                    ClashCfg::new_from_config_file(&source)?
                };
                let node_info = cfg.get_node_data_full()?;
                let proxy_str = node_info.proxies_string_pretty()?;
                println!("{proxy_str}");
                if *show_tags {
                    let tags = node_info.print_tags()?;
                    println!();
                    println!();
                    println!("{tags}");
                }
                Ok(())
            }
            Command::Gen { output } => Ok(()),
            Command::Append { dst } => Ok(()),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let _: () = match (&args.source, &args.url) {
        (Some(_), _) => (),
        (_, Some(_)) => (),
        _ => return Err(anyhow!("Either source or url need to provide")),
    };

    args.ayaya()
}
