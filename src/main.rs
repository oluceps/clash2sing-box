use clap::Parser;
use futures::executor::block_on;
use reqwest::header::USER_AGENT;
use serde::Serialize;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{error::Error, fs};
use yaml_rust::{Yaml, YamlLoader};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
enum AvalProtocals {
    Socks {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        version: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        network: Option<String>,
        udp_over_tcp: bool,
    },
    HTTP {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<TLS>,
    },
    Shadowsocks {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        method: String,
        password: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        plugin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        plugin_opts: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        network: Option<String>,
        udp_over_tcp: bool,
        //      multiplex: Option<Multiplex>,
    },
    //VMess,
    Trojan {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        password: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<TLS>,
    },
    Hysteria {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        up: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        up_mbps: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        down: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        down_mbps: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        obfs: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        auth: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_str: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        recv_window_conn: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        recv_window: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_mtu_discovery: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<TLS>,
    },

    VMess {
        r#type: String,
        tag: String,
        server: String,
        server_port: u16,
        uuid: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        security: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        alter_id: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        global_padding: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        authenticated_length: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<TLS>,
    },
    //    ShadowTLS,
    //    ShadowsocksR,
    //    Tor,
    //    SSH,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
struct TLS {
    enable: bool,
    disable_sni: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_name: Option<String>,
    insecure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    alpn: Option<Vec<String>>,
    utls: UTLS,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
}

// NOTICE: utls could be use only while enable with_utls build tag
#[allow(unused)]
#[derive(Debug, Serialize)]
struct UTLS {
    enabled: bool,
    fingerprint: String,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
struct Multiplex {
    enable: bool,
    protocol: String,
    max_connections: u16,
    min_streams: u16,
    max_streams: u16,
}

fn convert_to_node_vec(
    yaml_data: &yaml_rust::Yaml,
) -> Result<(Vec<serde_json::Value>, Vec<String>), Box<dyn Error>> {
    let mut node_list: Vec<serde_json::Value> = vec![];
    let mut nodename_list: Vec<String> = vec![];

    for (index, per_node) in yaml_data["proxies"].clone().into_iter().enumerate() {
        let param_str = |eter: &str| match per_node[eter].clone().into_string() {
            Some(i) => i,
            None => panic!("{} not exist!", eter),
        };

        let param_int = |eter: &str| match per_node[eter].clone().as_i64() {
            Some(i) => i as u16,
            None => match per_node[eter].clone().into_string().unwrap().parse::<u16>() {
                Ok(i) => i,
                Err(_) => panic!("error on parsing {eter}"),
            },
        };

        let optional = |eter: &str| match per_node[eter].clone().into_string() {
            Some(i) => Some(i),
            _ => None,
        };

        let named = || {
            per_node["name"].clone().into_string().unwrap_or(format!(
                "{}-{}",
                per_node["type"].clone().into_string().unwrap(),
                index
            ))
        };

        let parse_tls = || {
            if !per_node["tls"].is_null() {
                Some(TLS {
                    enable: if !(per_node["sni"].is_null()
                        | per_node["alpn"].is_null()
                        | per_node["skip-cert-verify"].is_null())
                    {
                        true
                    } else {
                        false
                    },

                    disable_sni: if per_node["sni"].clone().into_string()
                        == Some("true".to_string())
                    {
                        true
                    } else {
                        false
                    },
                    server_name: if per_node["sni"].clone().into_string().is_some() {
                        Some(param_str("sni"))
                    } else {
                        None
                    },
                    insecure: false, // default false, turn on manual if needed
                    alpn: if per_node["alpn"].clone().into_string().is_some() {
                        Some(vec!["h2".to_string()])
                    } else {
                        None
                    },

                    // Default enable utls to prevent potential attack. See https://github.com/net4people/bbs/issues/129
                    utls: UTLS {
                        enabled: true,
                        fingerprint: "chrome".to_string(),
                    },

                    certificate_path: if let Some(i) = per_node["ca"].clone().into_string() {
                        Some(i)
                    } else {
                        None
                    },
                    certificate: if let Some(i) = per_node["ca_str"].clone().into_string() {
                        Some(i)
                    } else {
                        None
                    },
                })
            } else {
                None
            }
        };
        let tobe_node = match per_node["type"].clone().into_string().unwrap().as_str() {
            "ss" => AvalProtocals::Shadowsocks {
                r#type: "ss".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                method: param_str("cipher"),
                password: param_str("password"),
                plugin: optional("plugin"),
                plugin_opts: match per_node["plugin"].clone().into_string() {
                    Some(_) => Some(plugin_opts_to_string(per_node["plugin-opts"].clone())),
                    None => None,
                },
                network: match per_node["udp"].clone().into_string() {
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
                up: per_node["up"].clone().into_string(),
                up_mbps: None,
                down: per_node["down"].clone().into_string(),
                down_mbps: None,
                obfs: per_node["obfs"].clone().into_string(),
                auth: None,
                auth_str: per_node["auth_str"].clone().into_string(),
                recv_window_conn: Some(param_int("recv_window_conn").into()),
                recv_window: Some(param_int("recv_window").into()),
                disable_mtu_discovery: if per_node["sni"].clone().into_string()
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
                alter_id: if per_node["alertId"].clone().into_string().is_some() {
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
                .clone()
                .into_string()
                .unwrap()
                .as_str()
            {
                "ss" => "Shadowsocks",
                "trojan" => "Trojan",
                "socks5" => "Socks",
                "hysteria" => "Hysteria",
                "vmess" => "VMess",
                i => i,
            }]
            .clone(),
        );
        nodename_list.push(per_node["name"].clone().into_string().unwrap())
    }
    Ok((node_list, nodename_list))
}

fn read_yaml(yaml_path: PathBuf) -> yaml_rust::Yaml {
    let yaml_data = YamlLoader::load_from_str(
        &read_to_string(yaml_path).expect("offer the path or subscribe link. see --help "),
    );

    match yaml_data {
        Ok(i) => i[0].clone(),
        Err(e) => panic!("{e}"),
    }
}

fn plugin_opts_to_string(opts: Yaml) -> String {
    format!(
        "mode={};host={}",
        opts["mode"].clone().into_string().unwrap(),
        opts["host"].clone().into_string().unwrap()
    )
}

async fn get_subscribe(sublink: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(sublink)
        .header(USER_AGENT, "clash")
        .send()
        .await?;
    block_on(res.text())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    content: Option<String>,

    #[arg(short, long)]
    subscribe: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}
#[tokio::main]
async fn main() {
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
        None => YamlLoader::load_from_str(
            get_subscribe(&args.subscribe.unwrap())
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()[0]
            .clone(),
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
