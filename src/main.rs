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
struct Tls {}

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
    tls: Tls,
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
    multiplex: Multiplex,
}

#[allow(unused)]
trait CouldBeConvert {
    fn convert(&self, yaml_data: Vec<yaml_rust::Yaml>) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for SOCKS {
    fn convert(&self, yaml_data: Vec<yaml_rust::Yaml>) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for HTTP {
    fn convert(&self, yaml_data: Vec<yaml_rust::Yaml>) -> () {}
}

#[allow(unused)]
impl CouldBeConvert for Shadowsocks {
    fn convert(&self, yaml_data: Vec<yaml_rust::Yaml>) -> () {}
}

fn main() {
    let yaml_test = YamlLoader::load_from_str("
proxies:
  - { name: 香港--01, type: ss, server: com, port: 4002, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
  - { name: 香港--02, type: ss, server: com, port: 4003, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
  - { name: 香港--03, type: ss, server: com, port: 4012, cipher: aes-256-gcm, password: '114514', plugin: obfs, plugin-opts: { mode: http, host: microsoft.com }, udp: true }
").unwrap();
    println!("{:?}", yaml_test);
}
