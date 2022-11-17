use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum AvalProtocals {
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
pub struct TLS {
    pub enabled: bool,
    pub disable_sni: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    pub insecure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    pub utls: UTLS,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

// NOTICE: utls could be use only while enable with_utls build tag
#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct UTLS {
    pub enabled: bool,
    pub fingerprint: String,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct Multiplex {
    pub enable: bool,
    pub protocol: String,
    pub max_connections: u16,
    pub min_streams: u16,
    pub max_streams: u16,
}
