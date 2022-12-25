use crate::node::*;
use reqwest::header::USER_AGENT;
pub use serde_json::{to_value, Value};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
pub use yaml_rust::{Yaml, YamlLoader};
pub trait Merge {
    fn merge(&mut self, new_json_value: Value);
}

pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (Value::Array(ref mut a), &Value::Object(ref b)) => {
            a.extend([Value::Object(b.clone())]);
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub struct NodeData {
    pub node_list: Vec<Value>,
    pub tag_list: Vec<String>,
}

pub trait Convert {
    fn convert(&self) -> Result<NodeData, Box<dyn Error>>;
}

pub fn convert_to_node_vec(yaml_data: &Yaml) -> Result<NodeData, Box<dyn Error>> {
    let mut node_list: Vec<Value> = vec![];
    let mut tag_list: Vec<String> = vec![];

    for per_node in yaml_data["proxies"].clone().into_iter() {
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

        let named = || match per_node["name"].to_owned().into_string() {
            Some(i) => i,
            None => panic!("clash config error: no name could be found!"),
        };

        let parse_tls = || {
            if !per_node["tls"].is_null() {
                Some(TLS {
                    enabled: !(per_node["sni"].is_null()
                        | per_node["alpn"].is_null()
                        | per_node["skip-cert-verify"].is_null()
                        | per_node["servername"].is_null()),

                    disable_sni: per_node["sni"].to_owned().into_string()
                        == Some("true".to_string()),

                    server_name: match per_node["sni"].to_owned().into_string() {
                        Some(_) => Some(param_str("sni")),
                        None => match per_node["servername"].to_owned().into_string() {
                            Some(_) => Some(param_str("servername")),
                            None => None,
                        },
                    },

                    // Default to be false, turn on manually if needed
                    insecure: false,

                    alpn: per_node["alpn"]
                        .to_owned()
                        .into_string()
                        .map(|_| vec!["h2".to_string()]),

                    // Default enable utls to prevent potential attack.
                    // See https://github.com/net4people/bbs/issues/129
                    utls: UTLS {
                        enabled: true,
                        fingerprint: "chrome".to_string(),
                    },

                    certificate_path: per_node["ca"].to_owned().into_string(),
                    certificate: per_node["ca_str"].to_owned().into_string(),
                })
            } else {
                None
            }
        };

        let parse_transport = || match per_node["network"].to_owned().into_string() {
            Some(i) => match i.as_str() {
                "http" => Some(Transport {
                    r#type: "http".to_string(),
                    host: None,
                    path: per_node["http-opts"]["path"][0].to_owned().into_string(),
                    method: per_node["http-ops"]["method"].to_owned().into_string(),
                    header: None,
                    max_early_data: None,
                    early_data_header_name: None,
                    service_name: None,
                }),
                "ws" => Some(Transport {
                    r#type: "ws".to_string(),
                    host: None,
                    path: per_node["ws-opts"]["path"].to_owned().into_string(),
                    method: None,
                    header: None,
                    max_early_data: match per_node["ws-opts"]["max-early-data"]
                        .to_owned()
                        .into_string()
                    {
                        Some(i) => match i.to_owned().parse::<u32>() {
                            Ok(i) => Some(i),
                            Err(_) => None,
                        },
                        None => None,
                    },
                    early_data_header_name: per_node["ws-opts"]["early-data-header-name"]
                        .to_owned()
                        .into_string(),
                    service_name: None,
                }),

                "grpc" => Some(Transport {
                    r#type: "grpc".to_string(),
                    host: None,
                    path: None,
                    method: None,
                    header: None,
                    max_early_data: None,
                    early_data_header_name: None,
                    service_name: per_node["grpc-opts"]["grpc-service-name"]
                        .to_owned()
                        .into_string(),
                }),
                &_ => todo!(),
            },
            None => todo!(),
        };

        let tobe_node = match per_node["type"].to_owned().into_string().unwrap().as_str() {
            "ss" => AvalProtocols::Shadowsocks {
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
                // multiplex: Some(Multiplex {
                //     enable: false,
                //     protocol: "smux".to_string(),
                //     max_connections: 0,
                //     min_streams: 0,
                //     max_streams: 0,
                // }),
            },

            "ssr" => AvalProtocols::Shadowsocksr {
                r#type: "shadowsocksr".to_string(),
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

            "socks5" => AvalProtocols::Socks {
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

            "http" => AvalProtocols::HTTP {
                r#type: "http".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                username: optional("username"),
                password: optional("password"),
                tls: parse_tls(),
            },

            "trojan" => AvalProtocols::Trojan {
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

            "hysteria" => AvalProtocols::Hysteria {
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

            "vmess" => AvalProtocols::VMess {
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
                transport: parse_transport(),
            },

            "vless" => AvalProtocols::Vless {
                r#type: "vless".to_string(),
                tag: named(),
                server: param_str("server"),
                server_port: param_int("port"),
                uuid: param_str("uuid"),
                network: if !per_node["udp"].is_null() {
                    None
                } else {
                    Some("tcp".to_string())
                },
                tls: parse_tls(),
                packet_encoding: None,
                transport: parse_transport(),
            },
            &_ => todo!(),
        };

        node_list.push(
            to_value(&tobe_node).unwrap()[match per_node["type"]
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
                "vless" => "Vless",
                "tuic" => continue,
                _ => continue,
            }]
            .to_owned(),
            // TYPE FROM CLASH => STRUCT NAME
        );
        tag_list.push(per_node["name"].to_owned().into_string().unwrap())
    }
    Ok(NodeData {
        node_list,
        tag_list,
    })
}
pub fn read_yaml(yaml_path: PathBuf) -> yaml_rust::Yaml {
    let yaml_data = YamlLoader::load_from_str(
        &read_to_string(yaml_path).expect("offer the path or subscribe link. see --help "),
    );

    match yaml_data {
        Ok(i) => i[0].to_owned(),
        Err(e) => panic!("{e}"),
    }
}

pub fn plugin_opts_to_string(opts: Yaml) -> String {
    format!(
        "mode={};host={}",
        opts["mode"].to_owned().into_string().unwrap(),
        opts["host"].to_owned().into_string().unwrap()
    )
}

pub fn get_subscribe(sublink: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(sublink).header(USER_AGENT, "clash").send()?;
    res.text()
}
