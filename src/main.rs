mod node;
mod paradigm;
mod util;
use clap::Parser;
use paradigm::PARADIGM;
use serde_json::{from_str, to_string, to_string_pretty, to_value};
use std::{fs::write, path::PathBuf};
use util::*;
use yaml_rust::YamlLoader;

impl Merge for Value {
    fn merge(&mut self, new_json_value: Value) {
        merge(self, &new_json_value);
    }
}

impl Convert for Yaml {
    fn convert(&self) -> Result<NodeData, Box<dyn std::error::Error>> {
        convert_to_node_vec(self)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Read path of clash format config.yaml file
    #[arg(short, long)]
    path: Option<String>,

    /// Get clash subscription profile by url
    #[arg(short, long, value_name = "URL")]
    subscribe: Option<String>,

    /// Output pretty-printed indented JSON
    #[arg(short = 'f')]
    format: bool,

    /// Generate minimal avaliable sing-box JSON profile
    #[arg(short, long = "gen-profile")]
    gen_profile: bool,

    /// Output sing-box JSON profile
    #[arg(short, long, value_name = "PATH")]
    output: Option<String>,
}
fn main() {
    let args = Args::parse();

    let yaml_path: Option<PathBuf> = match args.subscribe {
        Some(_) => None,
        None => match args.path {
            Some(i) => Some(PathBuf::from(i)),
            None => Some(PathBuf::from("./config.yaml".to_string())),
        },
    };

    let node_vec = (match yaml_path {
        Some(i) => read_yaml(i),
        None => {
            YamlLoader::load_from_str(get_subscribe(&args.subscribe.unwrap()).unwrap().as_str())
                .unwrap()[0]
                .to_owned()
        }
    })
    .convert();

    let valued_nodes_json = to_value(&match node_vec {
        Ok(ref i) => i.node_list.clone(),
        Err(e) => panic!("{}", e),
    })
    .unwrap();

    let valued_names_json = to_value(&match node_vec {
        Ok(ref i) => i.tag_list.clone(),
        Err(e) => panic!("{}", e),
    });

    if let Ok(ref i) = valued_names_json {
        println!("Node name list:\n\n{}\n", i.to_string());
    };

    match args.gen_profile {
        true => {
            let mut paradigm_deserialized: Value = from_str(PARADIGM).unwrap();
            paradigm_deserialized["outbounds"].merge(valued_nodes_json.clone());
            paradigm_deserialized["outbounds"][1]["outbounds"]
                .merge(valued_names_json.unwrap().clone());

            let j = match args.format {
                true => to_string_pretty(&paradigm_deserialized).unwrap(),
                false => to_string(&paradigm_deserialized).unwrap(),
            };

            if let Some(i) = args.output {
                write(&i, j.as_bytes()).unwrap();
                println!(
                    "\nMinimal avaliable sing-box config had been successful written into {}",
                    i
                )
            } else {
                println!("\nMinimal configuration:");
                println!("\n{}", j);
            }
        }
        false => {
            let j = match args.format {
                true => to_string_pretty(&valued_nodes_json).unwrap(),
                false => to_string(&valued_nodes_json).unwrap(),
            };

            println!("sing-box client outbound:\n\n\n{}", j);

            if let Some(i) = args.output {
                write(&i, j.as_bytes()).unwrap();
                println!("\n\n\nSuccessful written into {}", i)
            }
        }
    }
}
