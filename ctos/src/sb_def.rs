use serde::Serialize;

macro_rules! build {
    ($($body:ident),*) => {
        as_item! {
            #[allow(dead_code)]
            #[derive(Debug, Serialize)]
            pub enum SingboxNodeDef {
                $(
                    $body($body),
                )*
            }
        }
    };
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

#[derive(Debug, Serialize)]
pub struct Socks {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub udp_over_tcp: bool,
}

#[derive(Debug, Serialize)]
pub struct Http {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
}

#[derive(Debug, Serialize)]
pub struct Shadowsocks {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub method: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_opts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub udp_over_tcp: bool,
    //      multiplex: Option<Multiplex>,
}

#[derive(Debug, Serialize)]
pub struct Shadowsocksr {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub method: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs_param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Trojan {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
}

#[derive(Debug, Serialize)]
pub struct Hysteria {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window_conn: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_mtu_discovery: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
}

#[derive(Debug, Serialize)]
pub struct VMess {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alter_id: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_padding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated_length: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}
#[derive(Debug, Serialize)]
pub struct Vless {
    pub r#type: String,
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}

#[derive(Debug, Serialize)]
pub struct Tuic {
    pub r#type: String,
    pub tag: String,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct Tls {
    pub enabled: bool,
    pub disable_sni: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    pub insecure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    pub utls: Utls,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<RealityOpts>,
}

// NOTICE: utls could be use only while enable with_utls build tag
#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct Utls {
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

// v2ray transport in sing-box
#[derive(Debug, Serialize)]
pub struct Transport {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RealityOpts {
    pub enabled: bool,
    pub public_key: Option<String>,
    pub short_id: Option<String>,
}

build! { Socks, Http, Shadowsocks, Shadowsocksr, Trojan, Hysteria, VMess, Vless, Tuic }
