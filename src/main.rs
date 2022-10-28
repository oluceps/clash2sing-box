use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

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
        plugin: String,
        plugin_opts: String,
        network: String,
        udp_over_tcp: bool,
        multiplex: Option<Multiplex>,
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

fn convert_to_node_vec(yaml_data: &yaml_rust::Yaml) -> Vec<serde_json::Value> {
    // single node: yaml_test["proxies"][n]
    let mut node_list: Vec<serde_json::Value> = vec![];

    for (index, single_node) in yaml_data["proxies"].clone().into_iter().enumerate() {
        println!("{:?}", single_node["server"]);

        let param_str = |eter: &str| single_node[eter].clone().into_string().unwrap();

        let param_int = |eter: &str| single_node[eter].clone().into_i64().unwrap() as u16;

        let tobe_node = match single_node["type"].clone().into_string().unwrap().as_str() {
            "ss" => AvalProtocals::Shadowsocks {
                r#type: "ss".to_string(),
                tag: format!("ss-{index}"),
                server: param_str("server"),
                server_port: param_int("port"),
                method: param_str("cipher"),
                password: param_str("password"),
                plugin: param_str("plugin"),
                plugin_opts: "".to_string(),
                network: "".to_string(),
                udp_over_tcp: false,
                multiplex: Some(Multiplex {
                    enable: false,
                    protocol: "smux".to_string(),
                    max_connections: 0,
                    min_streams: 0,
                    max_streams: 0,
                }),
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
    node_list
}

fn read_yaml() -> yaml_rust::Yaml {
    let yaml_path = PathBuf::from("./config.yaml");

    let yaml_data = YamlLoader::load_from_str(
        &read_to_string(yaml_path).expect("Should have been able to read the file"),
    );

    yaml_data.unwrap()[0].clone()
}

// fn to_json(node_list: Vec<AvalProtocals>) ->

#[allow(unused)]
fn main() {
    let node_list = convert_to_node_vec(&read_yaml());

    //    for i in node_list {
    //        println!("{:?}", i)
    //    }

    let j = serde_json::to_string(&serde_json::to_value(&node_list).unwrap()).unwrap();

    println!("{}", j)

    //   for i in j {
    //        println!("{:#}", i.get("Shadowsocks").unwrap())
    //    }

    // TODO: write to json
}
