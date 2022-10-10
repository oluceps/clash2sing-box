use serde::{Deserialize, Serialize};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
enum AvalProtocals {
    SOCKS,
    HTTP,
    Shadowsocks,
    VMess,
    Trojan,
    Hysteria,
    ShadowTLS,
    ShadowsocksR,
    VLESS,
    Tor,
    SSH,
}

// TODO: TLS
#[allow(unused)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TLS {}

#[allow(unused)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Multiplex {
    enable: bool,
    protocal: String,
    max_connections: u16,
    min_streams: u16,
    max_streams: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SOCKS {
    tag: String,
    server: String,
    server_port: u16,
    version: u16,
    username: String,
    password: String,
    network: String,
    udp_over_tcp: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct HTTP {
    tag: String,
    server: String,
    server_port: u16,
    username: String,
    password: String,
    tls: TLS,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Shadowsocks {
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
}

#[allow(unused)]
trait CouldBeConvert {
    fn new() -> Self;

    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for SOCKS {
    fn new() -> SOCKS {
        SOCKS {
            tag: "".to_string(),
            server: "".to_string(),
            server_port: 0,
            version: 0,
            username: "".to_string(),
            password: "".to_string(),
            network: "".to_string(),
            udp_over_tcp: false,
        }
    }
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for HTTP {
    fn new() -> HTTP {
        HTTP {
            tag: "".to_string(),
            server: "".to_string(),
            server_port: 0,
            username: "".to_string(),
            password: "".to_string(),
            tls: TLS {},
        }
    }
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for Shadowsocks {
    fn new() -> Shadowsocks {
        Shadowsocks {
            tag: "".to_string(),
            server_port: 0,
            server: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            plugin: "".to_string(),
            plugin_opts: "".to_string(),
            network: "".to_string(),
            udp_over_tcp: false,
            multiplex: Some(
                Multiplex {
                    enable: false,
                    protocal: "smux".to_string(),
                    max_connections: 0,
                    min_streams: 0,
                    max_streams: 0,
                }),
        }
    }
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
fn main() {
    let yaml_str =
        "
proxies:
  - { name: 香港--01, type: ss, server: com, port: 4002, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
  - { name: 香港--02, type: ss, server: com, port: 4003, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
  - { name: 香港--03, type: ss, server: com, port: 4012, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
";
    let yaml_test = &YamlLoader::load_from_str(yaml_str).unwrap()[0];
    // single node: yaml_test["proxies"][n]
    let node_list: Vec<i8> = vec![];

    for single_node in yaml_test["proxies"].clone() {
        println!("{:?}", single_node["server"]);

        let protocols = "ss".to_string();
        let processed_node = match single_node["type"].clone().into_string() {
            protocal => Shadowsocks::new(),
        }.convert(single_node);
    }


    // TODO: read yaml file
    // TODO: for proxies list
    // TODO: match attrset[type]
    // TODO: convert
    // TODO: write to json
}
