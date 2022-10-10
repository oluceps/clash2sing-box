use serde::{Deserialize, Serialize};

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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct tls {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct multiplex {
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
    tls: tls,
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
    multiplex: multiplex,
}

fn main() {
    println!("Hello, world!");
}
