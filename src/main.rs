mod node;
use clap::Parser;
use reqwest::header::USER_AGENT;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{error::Error, fs};
use yaml_rust::{Yaml, YamlLoader};

use crate::node::*;

fn convert_to_node_vec(
    yaml_data: &yaml_rust::Yaml,
) -> Result<(Vec<serde_json::Value>, Vec<String>), Box<dyn Error>> {
    let mut node_list: Vec<serde_json::Value> = vec![];
    let mut nodename_list: Vec<String> = vec![];

    for (index, per_node) in yaml_data["proxies"].clone().into_iter().enumerate() {
        let param_str = |eter: &str| match per_node[eter].to_owned().into_string() {
            Some(i) => i,
            None => panic!("{} not exist!", eter),
        };

        let param_int = |eter: &str| match per_node[eter].to_owned().as_i64() {
            Some(i) => i as u16,
            None => match per_node[eter]
                .to_owned()
                .into_string()
                .unwrap()
                .parse::<u16>()
            {
                Ok(i) => i,
                Err(_) => panic!("error on parsing {eter}"),
            },
        };

        let optional = |eter: &str| match per_node[eter].to_owned().into_string() {
            Some(i) => match i.as_str() {
                "obfs" => Some("obfs-local".to_string()),
                "v2ray-plugin" => Some("v2ray-plugin".to_string()),
                &_ => None,
            },
            _ => None,
        };

        let named = || {
            per_node["name"].to_owned().into_string().unwrap_or(format!(
                "{}-{}",
                per_node["type"].to_owned().into_string().unwrap(),
                index
            ))
        };

        let parse_tls = || {
            if !per_node["tls"].is_null() {
                Some(TLS {
                    enabled: if !(per_node["sni"].is_null()
                        | per_node["alpn"].is_null()
                        | per_node["skip-cert-verify"].is_null())
                    {
                        true
                    } else {
                        false
                    },

                    disable_sni: if per_node["sni"].to_owned().into_string()
                        == Some("true".to_string())
                    {
                        true
                    } else {
                        false
                    },
                    server_name: if per_node["sni"].to_owned().into_string().is_some() {
                        Some(param_str("sni"))
                    } else {
                        None
                    },
                    insecure: false, // default false, turn on manual if needed
                    alpn: if per_node["alpn"].to_owned().into_string().is_some() {
                        Some(vec!["h2".to_string()])
                    } else {
                        None
                    },

                    // Default enable utls to prevent potential attack. See https://github.com/net4people/bbs/issues/129
                    utls: UTLS {
                        enabled: true,
                        fingerprint: "chrome".to_string(),
                    },

                    certificate_path: if let Some(i) = per_node["ca"].to_owned().into_string() {
                        Some(i)
                    } else {
                        None
                    },
                    certificate: if let Some(i) = per_node["ca_str"].to_owned().into_string() {
                        Some(i)
                    } else {
                        None
                    },
                })
            } else {
                None
            }
        };
        let tobe_node = match per_node["type"].to_owned().into_string().unwrap().as_str() {
            "ss" => AvalProtocals::Shadowsocks {
                r#type: "shadowsocks".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                method: param_str("cipher"),
                password: param_str("password"),
                plugin: optional("plugin"),
                plugin_opts: match per_node["plugin"].to_owned().into_string() {
                    Some(_) => Some(plugin_opts_to_string(per_node["plugin-opts"].to_owned())),
                    None => None,
                },
                network: match per_node["udp"].to_owned().into_string() {
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

            "ssr" => AvalProtocals::Shadowsocksr {
                r#type: "shadowsocks".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                method: param_str("cipher"),
                password: param_str("password"),
                obfs: Some(param_str("obfs")),
                obfs_param: Some(param_str("obfs-param")),
                protocol: Some(param_str("protocol")),
                protocol_param: Some(param_str("protocol-param")),
                network: match per_node["udp"].to_owned().into_string() {
                    Some(_) => None,
                    _ => Some("tcp".to_string()),
                },
            },

            "socks5" => AvalProtocals::Socks {
                r#type: "socks".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                version: 5,
                username: optional("username"),
                password: optional("username"),
                network: if !per_node["udp"].is_null() {
                    Some("udp".to_string())
                } else {
                    None
                },
                udp_over_tcp: false,
            },

            "http" => AvalProtocals::HTTP {
                r#type: "http".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                username: optional("username"),
                password: optional("password"),
                tls: parse_tls(),
            },

            "trojan" => AvalProtocals::Trojan {
                r#type: "trojan".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                password: param_str("password"),
                network: if !per_node["udp"].is_null() {
                    None
                } else {
                    Some("tcp".to_string())
                },
                tls: parse_tls(),
            },

            "hysteria" => AvalProtocals::Hysteria {
                r#type: "hysteria".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                up: per_node["up"].to_owned().into_string(),
                up_mbps: None,
                down: per_node["down"].to_owned().into_string(),
                down_mbps: None,
                obfs: per_node["obfs"].to_owned().into_string(),
                auth: None,
                auth_str: per_node["auth_str"].to_owned().into_string(),
                recv_window_conn: Some(param_int("recv_window_conn").into()),
                recv_window: Some(param_int("recv_window").into()),
                disable_mtu_discovery: if per_node["sni"].to_owned().into_string()
                    == Some("true".to_string())
                {
                    Some(true)
                } else {
                    None
                },
                tls: parse_tls(),
            },

            "vmess" => AvalProtocals::VMess {
                r#type: "vmess".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                uuid: param_str("uuid"),
                security: None,
                alter_id: if per_node["alertId"].to_owned().into_string().is_some() {
                    Some(param_int("alertId").into())
                } else {
                    Some(0)
                },
                global_padding: None,
                authenticated_length: None,
                network: if !per_node["udp"].is_null() {
                    None
                } else {
                    Some("tcp".to_string())
                },
                tls: parse_tls(),
            },

            &_ => todo!(),
        };

        //        let a = processed_node::new();
        node_list.push(
            serde_json::to_value(&tobe_node).unwrap()[match per_node["type"]
                .to_owned()
                .into_string()
                .unwrap()
                .as_str()
            {
                "ss" => "Shadowsocks",
                "trojan" => "Trojan",
                "socks5" => "Socks",
                "hysteria" => "Hysteria",
                "vmess" => "VMess",
                "ssr" => "Shadowsocksr",
                i => i,
            }]
            .to_owned(),
            // TYPE FROM CLASH => STRUCT NAME
        );
        nodename_list.push(per_node["name"].to_owned().into_string().unwrap())
    }
    Ok((node_list, nodename_list))
}

fn read_yaml(yaml_path: PathBuf) -> yaml_rust::Yaml {
    let yaml_data = YamlLoader::load_from_str(
        &read_to_string(yaml_path).expect("offer the path or subscribe link. see --help "),
    );

    match yaml_data {
        Ok(i) => i[0].to_owned(),
        Err(e) => panic!("{e}"),
    }
}

fn plugin_opts_to_string(opts: Yaml) -> String {
    format!(
        "mode={};host={}",
        opts["mode"].to_owned().into_string().unwrap(),
        opts["host"].to_owned().into_string().unwrap()
    )
}

fn get_subscribe(sublink: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(sublink).header(USER_AGENT, "clash").send()?;
    res.text()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Read path of clash format config.yaml file
    #[arg(short, long)]
    path: Option<String>,

    /// (unimplement)Read content of clash format proxies list
    #[arg(short, long)]
    content: Option<String>,

    /// Get clash subscription profile by url
    #[arg(short, long)]
    subscribe: Option<String>,

    /// output sing-box json profile
    #[arg(short, long)]
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

    let node_vec = convert_to_node_vec(&match yaml_path {
        Some(i) => read_yaml(i),
        None => {
            YamlLoader::load_from_str(get_subscribe(&args.subscribe.unwrap()).unwrap().as_str())
                .unwrap()[0]
                .to_owned()
        }
    });

    if let Ok(ref i) = node_vec {
        println!("node name list:\n{:?}\n\n\n", i.1);
    };

    let j = serde_json::to_string(
        &serde_json::to_value(&match node_vec {
            Ok(i) => i.0,
            Err(e) => panic!("{}", e),
        })
        .unwrap(),
    )
    .unwrap();

    println!("{}", j);

    if let Some(i) = args.output {
        fs::write(&i, j.as_bytes()).unwrap();
    }
}
