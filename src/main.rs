use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum AvalProtocals {
    Socks {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        version: u16,
        username: String,
        password: String,
        network: Option<String>,
        udp_over_tcp: bool,
    },
    HTTP {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        username: String,
        password: String,
        tls: TLS,
    },
    Shadowsocks {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        method: String,
        password: String,
        plugin: Option<String>,
        plugin_opts: Option<String>,
        network: Option<String>,
        udp_over_tcp: bool,
        //      multiplex: Option<Multiplex>,
    },
    //VMess,
    //    Trojan,
    //    Hysteria,
    //    ShadowTLS,
    //    ShadowsocksR,
    //    VLESS,
    //    Tor,
    //    SSH,
}

// TODO: TLS
#[allow(unused)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TLS {}

#[allow(unused)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Multiplex {
    enable: bool,
    protocol: String,
    max_connections: u16,
    min_streams: u16,
    max_streams: u16,
}

fn convert_to_node_vec(
    yaml_data: &yaml_rust::Yaml,
) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let mut node_list: Vec<serde_json::Value> = vec![];

    for (index, single_node) in yaml_data["proxies"].clone().into_iter().enumerate() {
        let param_str = |eter: &str| match single_node[eter].clone().into_string() {
            Some(i) => i,
            None => panic!("{} not exist!", eter),
        };

        let param_int = |eter: &str| single_node[eter].clone().into_i64().unwrap() as u16;

        let tobe_node = match single_node["type"].clone().into_string().unwrap().as_str() {
            "ss" => AvalProtocals::Shadowsocks {
                r#type: "ss".to_string(),
                tag: format!("ss-{index}"),
                server: param_str("server"),
                server_port: param_int("port"),
                method: param_str("cipher"),
                password: param_str("password"),
                plugin: match single_node["plugin"].clone().into_string() {
                    Some(_) => Some(param_str("plugin")),
                    _ => None,
                },
                plugin_opts: match single_node["plugin"].clone().into_string() {
                    Some(_) => Some(plugin_opts_to_string(single_node["plugin-opts"].clone())),
                    None => None,
                },
                network: match single_node["udp"].clone().into_string() {
                    Some(_) => None,
                    _ => Some("tcp".to_string()),
                },
                udp_over_tcp: false,
                //                multiplex: Some(Multiplex {
                //                    enable: false,
                //                    protocol: "smux".to_string(),
                //                    max_connections: 0,
                //                    min_streams: 0,
                //                    max_streams: 0,
                //                }),
            },

            "socks5" => AvalProtocals::Socks {
                r#type: "socks".to_string(),
                tag: format!("socks-{index}"),
                server: param_str("server"),
                server_port: param_int("port"),
                version: 5,
                username: param_str("username"),
                password: param_str("password"),
                network: if !single_node["udp"].is_null() {
                    Some("udp".to_string())
                } else {
                    None
                },
                udp_over_tcp: false,
            },

            "http" => AvalProtocals::HTTP {
                r#type: "http".to_string(),
                tag: format!("http-{index}"),
                server: param_str("server"),
                server_port: param_int("port"),
                username: param_str("username"),
                password: param_str("password"),
                tls: TLS {},
            },
            &_ => todo!(),
        };

        //        let a = processed_node::new();
        node_list.push(
            serde_json::to_value(&tobe_node).unwrap()[match single_node["type"]
                .clone()
                .into_string()
                .unwrap()
                .as_str()
            {
                "ss" => "Shadowsocks",
                i => i,
            }]
            .clone(),
        );
    }
    Ok(node_list)
}

fn read_yaml(yaml_path: PathBuf) -> yaml_rust::Yaml {
    let yaml_data = YamlLoader::load_from_str(
        &read_to_string(yaml_path).expect("Should have been able to read the file"),
    );

    yaml_data.unwrap()[0].clone()
}

fn plugin_opts_to_string(opts: Yaml) -> String {
    format!(
        "mode={};host={}",
        opts["mode"].clone().into_string().unwrap(),
        opts["host"].clone().into_string().unwrap()
    )
}

#[allow(unused)]
fn main() {
    let app = App::new("clash2sing-box")
        .about("convert clash proxy list to sing-box")
        .arg(Arg::with_name("path").required(false))
        .get_matches();

    let yaml_path: PathBuf = match app.value_of("path") {
        Some(i) => i.into(),
        _ => PathBuf::from("./config.yaml"),
    };

    let j = serde_json::to_string(
        &serde_json::to_value(&match convert_to_node_vec(&read_yaml(yaml_path)) {
            Ok(i) => i,
            Err(e) => panic!("{}", e),
        })
        .unwrap(),
    )
    .unwrap();

    println!("{}", j)

    // TODO: write to json
}
