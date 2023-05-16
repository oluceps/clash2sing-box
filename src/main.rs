use anyhow::Result;
use clap::{Parser, Subcommand};
use ctos::ClashCfg;
use todo_by::todo_by;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,

    #[clap(short, long, help = "clash config path (url)")]
    source: String,
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
        tags: bool,
    },
    #[clap(about = "Generate sing-box profile from clash format")]
    Gen {
        #[clap(long, help = "location of paradigm sing-box profile")]
        paradigm: Option<String>,
    },
    #[clap(about = "Append new clash proxies to existed sing-box profile")]
    Append {
        #[clap(long, help = "location of sing-box profile to be append")]
        dst: String,
    },
}

impl Args {
    fn ayaya(&self) -> Result<()> {
        let produce_cfg = || -> Result<ClashCfg> {
            let source = self.source.as_str();
            if self.source.as_str().starts_with("http") {
                return ClashCfg::new_from_subscribe_link(self.source.as_str());
            }
            ClashCfg::new_from_config_file(&source)
        };
        match &self.cmd {
            Command::Show { tags } => {
                let cfg = produce_cfg()?;
                let node_info = cfg.get_node_data_full()?;

                let proxy_str = node_info.proxies_string_pretty()?;
                println!("{proxy_str}");
                if *tags {
                    let tags = node_info.print_tags()?;
                    println!();
                    println!();
                    println!("{tags}");
                }
                Ok(())
            }
            Command::Gen { paradigm } => {
                let cfg = produce_cfg()?;
                let node_info = cfg.get_node_data_full()?;

                if let Some(i) = paradigm {
                    let mut prd: serde_json::Value =
                        serde_json::from_str(std::fs::read_to_string(i)?.as_str())?;
                    node_info.merge_to_value(&mut prd);
                    println!("{prd}")
                } else {
                    println!("{}", serde_json::to_string_pretty(&node_info.merge_min())?)
                };

                Ok(())
            }
            Command::Append { dst: _ } => {
                todo_by!("2023-05-20");
                Ok(())
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    args.ayaya()
}
