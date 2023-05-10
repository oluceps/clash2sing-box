mod node;
mod paradigm;
mod util;
use clap::Parser;
use paradigm::PARADIGM;
use serde_json::{from_str, to_string, to_string_pretty, to_value};
use std::{fs::write, path::PathBuf};
use util::*;
use yaml_rust::YamlLoader;

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

    let yaml_path: Option<PathBuf> = if args.subscribe.is_some() {
        None
    } else {
        args.path
            .map_or(Some(PathBuf::from("./config.yaml".to_string())), |i| {
                Some(PathBuf::from(i))
            })
    };

    let node_vec = yaml_path
        .map_or(
            YamlLoader::load_from_str(get_subscribe(&args.subscribe.unwrap()).unwrap().as_str())
                .unwrap()[0]
                .to_owned(),
            |i| read_yaml(i),
        )
        .convert();

    let valued_nodes_json =
        to_value(node_vec.as_ref().map(|i| i.node_list.clone()).unwrap()).unwrap();

    let valued_names_json = to_value(node_vec.as_ref().map(|i| i.tag_list.clone()).unwrap());

    if let Ok(ref i) = valued_names_json {
        println!("Node name list:\n\n{}\n", i);
    };

    macro_rules! gen_json {
        ($v: ident) => {
            if args.format {
                to_string_pretty(&$v).unwrap()
            } else {
                to_string(&$v).unwrap()
            }
        };
    }

    if args.gen_profile {
        let mut paradigm_deserialized: Value = from_str(PARADIGM).unwrap();
        paradigm_deserialized["outbounds"].merge(valued_nodes_json);
        paradigm_deserialized["outbounds"][1]["outbounds"].merge(valued_names_json.unwrap());

        let j = gen_json!(paradigm_deserialized);
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
    } else {
        let j = gen_json!(valued_nodes_json);

        println!("sing-box client outbound:\n\n\n{}", j);

        if let Some(i) = args.output {
            write(&i, j.as_bytes()).unwrap();
            println!("\n\n\nSuccessful written into {}", i)
        }
    }
}
