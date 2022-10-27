use serde::{Deserialize, Serialize};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum AvalProtocals {
    Socks(Socks),
    HTTP(HTTP),
    Shadowsocks(Shadowsocks),
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Socks {
    r#type: String,
    tag: String,
    server: String,
    server_port: u16,
    version: u16,
    username: String,
    password: String,
    network: Option<String>,
    udp_over_tcp: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct HTTP {
    r#type: String,
    tag: String,
    server: String,
    server_port: u16,
    username: String,
    password: String,
    tls: TLS,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Shadowsocks {
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
}

#[allow(unused)]
trait CouldBeConvert {
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for Socks {
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for HTTP {
    fn convert(&self, yaml_data: yaml_rust::Yaml) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for Shadowsocks {
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
    let node_list: Vec<AvalProtocals> = vec![];

    for (index, single_node) in yaml_test["proxies"].clone().into_iter().enumerate() {
        println!("{:?}", single_node["server"]);

        let param_str = |eter: &str| single_node[eter].clone().into_string().unwrap();

        let param_int = |eter: &str| single_node[eter].clone().into_i64().unwrap() as u16;

        let tobe_node = match single_node["type"].clone().into_string().unwrap().as_str() {
            "ss" => AvalProtocals::Shadowsocks(Shadowsocks {
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
            }),

            "socks5" => AvalProtocals::Socks(Socks {
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
            }),

            "http" => AvalProtocals::HTTP(HTTP {
                r#type: "http".to_string(),
                tag: format!("http-{index}"),
                server: param_str("server"),
                server_port: param_int("port"),
                username: param_str("username"),
                password: param_str("password"),
                tls: TLS {},
            }),
            &_ => todo!(),
        };

        //        let a = processed_node::new();
        println!("{:?}", tobe_node)
    }

    // TODO: read yaml file
    // TODO: for proxies list
    // TODO: match attrset[type]
    // TODO: convert
    // TODO: write to json
}
