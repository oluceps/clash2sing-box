use anyhow::Result;
use clap::{Parser, Subcommand};
use ctos::{ClashCfg, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

#[allow(unused)]
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
        #[clap(short, long, help = "location of sing-box profile to be append")]
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
            Command::Append { dst } => {
                let cfg = produce_cfg()?;
                let node_info = cfg.get_node_data_full()?;

                let mut dst_file = OpenOptions::new().read(true).open(dst)?;

                // backup
                let bkup_file_path = {
                    let p = PathBuf::from(dst);
                    p.parent().unwrap().join(format!(
                        "{}.backup",
                        p.file_name().unwrap().to_str().unwrap()
                    ))
                };

                fs::copy(dst, bkup_file_path)?;

                let mut dst_file_ctt: serde_json::Value = {
                    let mut t: String = String::new();
                    let _ = dst_file.read_to_string(&mut t);
                    serde_json::from_str(t.as_str())?
                };

                node_info.append_to(&mut dst_file_ctt);

                drop(dst_file);

                OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(dst)?
                    .write_all(serde_json::to_string_pretty(&dst_file_ctt)?.as_bytes())?;

                Ok(())
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    args.ayaya()
}
