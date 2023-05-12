pub mod paradigm;
pub mod sb_def;

use anyhow::{anyhow, Result};
// use paradigm::PARADIGM;
use reqwest::header::USER_AGENT;
use sb_def::{
    Http, Hysteria, Shadowsocks, Shadowsocksr, SingboxNodeDef, Socks, Tls, Transport, Trojan, Utls,
    VMess, Vless,
};
pub use serde_json::{from_str, to_string, to_string_pretty, to_value};
pub use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};
use std::{io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct ClashCfg(Yaml);

#[derive(Debug, Clone)]
pub struct SingboxCfg(serde_json::Value);

impl Into<ClashCfg> for Yaml {
    fn into(self) -> ClashCfg {
        ClashCfg(self)
    }
}

impl ClashCfg {
    pub fn new_from_subscribe_link(link: &str) -> Result<Self> {
        Self::to_yaml_data(link, |source| Self::get_subscribe(source).unwrap()).map(|i| i.into())
    }

    pub fn new_from_config_file(p: impl AsRef<Path> + Copy) -> Result<Self> {
        Self::to_yaml_data(p, |source| {
            read_to_string(source)
                .map_err(|_| anyhow!("error on reading clash config"))
                .unwrap()
        })
        .map(|i| i.into())
    }

    pub fn new_from_plain_text(t: &str) -> Result<Self> {
        Ok(YamlLoader::load_from_str(t.as_ref())?.remove(0).into())
    }

    pub fn to_yaml_data<S, F>(source: S, f: F) -> Result<Yaml>
    where
        S: AsRef<Path>,
        for<'a> F: FnOnce(S) -> String,
    {
        let raw_config = YamlLoader::load_from_str(f(source).as_ref())?.remove(0);
        Ok(raw_config)
    }

    pub fn get_subscribe(link: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(link).header(USER_AGENT, "clash").send()?;
        Ok(res.text()?)
    }

    pub fn get_proxies(&self) -> Result<&Yaml> {
        Ok(&self.0["proxies"])
    }

    /// get all tags
    pub fn get_node_tags(&self) -> Vec<String> {
        let proxies = self.get_proxies().unwrap();

        // TODO: opt
        let tags: Vec<String> = proxies
            .clone()
            .into_iter()
            .map(|yml| {
                yml["name"]
                    .clone()
                    .into_string()
                    .unwrap_or("unnamed".to_string())
            })
            .collect();

        tags
    }

    pub fn get_node_types(&self) -> Vec<String> {
        let proxies = self.get_proxies().unwrap();

        // TODO: opt
        let types: Vec<String> = proxies
            .clone()
            .into_iter()
            .map(|yml| {
                yml["type"]
                    .clone()
                    .into_string()
                    .unwrap_or("unknown".to_string())
            })
            .collect();

        types
    }

    pub fn type_converter(ts: Vec<String>) -> Vec<String> {
        ts.into_iter()
            .map(|i| match i.as_str() {
                "ss" => "Shadowsocks",
                "trojan" => "Trojan",
                "socks5" => "Socks",
                "hysteria" => "Hysteria",
                "vmess" => "VMess",
                "ssr" => "Shadowsocksr",
                "vless" => "Vless",
                "tuic" => "",
                _ => "",
            })
            .map(|i| i.to_string())
            .collect()
    }

    /// get node data, include tag name and other factors
    pub fn get_node_data_full(&self) -> Result<NodeInfo> {
        let proxies = self.get_proxies()?;
        let tags = self.get_node_tags();

        let json_proxy_list: Vec<serde_json::Value> = proxies
            .clone()
            .into_iter()
            .map(|i| {
                serde_json::to_value(PerClashProxy::from(i).convert_to_singbox_def())
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(_, v)| serde_json::from_value::<serde_json::Value>(v.clone()).unwrap())
                    .collect::<serde_json::Value>()
            })
            .map(|i| i[0].clone())
            .collect();

        Ok(NodeInfo {
            list: json_proxy_list,
            tags,
        })
    }

    pub fn get_rules(&self) {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct PerClashProxy(Yaml);

impl From<Yaml> for PerClashProxy {
    fn from(value: Yaml) -> Self {
        Self(value)
    }
}

impl PerClashProxy {
    pub fn str_param(&self, s: &str) -> String {
        self.0[s].to_owned().into_string().unwrap()
    }
    pub fn int_param(&self, s: &str) -> u16 {
        self.0[s].to_owned().as_i64().map_or_else(
            || {
                self.0[s]
                    .to_owned()
                    .into_string()
                    .unwrap()
                    .parse::<u16>()
                    .unwrap()
            },
            |i| i as u16,
        )
        // 0
    }
    pub fn optional_plugin(&self, s: &str) -> Option<String> {
        self.0[s]
            .to_owned()
            .into_string()
            .map(|i| match i.as_str() {
                "obfs" => "obfs-local".to_string(),
                "v2ray-plugin" => "v2ray-plugin".to_string(),
                i => i.to_string(),
            })
    }
    pub fn named(&self) -> String {
        self.0["name"]
            .to_owned()
            .into_string()
            .unwrap_or("unnamed".to_string())
    }

    pub fn typed(&self) -> String {
        self.0["type"]
            .to_owned()
            .into_string()
            .unwrap_or("unknown".to_string())
    }

    pub fn parse_tls(&self) -> Option<Tls> {
        if !self.0["tls"].is_badvalue() {
            return Some(Tls {
                enabled: !(self.0["tls"].is_badvalue()
                    & self.0["sni"].is_badvalue()
                    & self.0["alpn"].is_badvalue()
                    & self.0["skip-cert-verify"].is_badvalue()
                    & self.0["servername"].is_badvalue()),

                disable_sni: self.0["sni"].to_owned().into_string() == Some("true".to_string()),

                server_name: match self.0["sni"].to_owned().into_string() {
                    Some(_) => Some(self.str_param("sni")),
                    None => self.0["servername"]
                        .to_owned()
                        .into_string()
                        .map(|_| self.str_param("servername")),
                },

                // Default to be false, turn on manually if needed
                insecure: false,

                alpn: self.0["alpn"]
                    .to_owned()
                    .into_string()
                    .map(|_| vec!["h2".to_string()]),

                // Default enable utls to prevent potential attack.
                // See https://github.com/net4people/bbs/issues/129
                utls: Utls {
                    enabled: true,
                    fingerprint: "chrome".to_string(),
                },

                certificate_path: self.0["ca"].to_owned().into_string(),
                certificate: self.0["ca_str"].to_owned().into_string(),
            });
        }
        None
    }
    fn parse_transport(&self) -> Option<Transport> {
        match self.0["network"].to_owned().into_string() {
            Some(i) => match i.as_str() {
                "http" => Some(Transport {
                    r#type: "http".to_string(),
                    host: None,
                    path: self.0["http-opts"]["path"][0].to_owned().into_string(),
                    method: self.0["http-ops"]["method"].to_owned().into_string(),
                    header: None,
                    max_early_data: None,
                    early_data_header_name: None,
                    service_name: None,
                }),
                "ws" => Some(Transport {
                    r#type: "ws".to_string(),
                    host: None,
                    path: self.0["ws-opts"]["path"].to_owned().into_string(),
                    method: None,
                    header: None,
                    max_early_data: match self.0["ws-opts"]["max-early-data"]
                        .to_owned()
                        .into_string()
                    {
                        Some(i) => match i.parse::<u32>() {
                            Ok(i) => Some(i),
                            Err(_) => None,
                        },
                        None => None,
                    },
                    early_data_header_name: self.0["ws-opts"]["early-data-header-name"]
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
                    service_name: self.0["grpc-opts"]["grpc-service-name"]
                        .to_owned()
                        .into_string(),
                }),
                &_ => todo!(),
            },
            None => None,
        }
    }

    fn plugin_opts_to_string(opts: Yaml) -> String {
        format!(
            "obfs={};obfs-host={}",
            opts["mode"].to_owned().into_string().unwrap(),
            opts["host"].to_owned().into_string().unwrap()
        )
    }
    fn convert_to_singbox_def(&self) -> SingboxNodeDef {
        let proxy_type = self.typed();

        match proxy_type.as_str() {
            "ss" => SingboxNodeDef::Shadowsocks(Shadowsocks {
                r#type: "shadowsocks".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                method: self.str_param("cipher"),
                password: self.str_param("password"),
                plugin: self.optional_plugin("plugin"),
                plugin_opts: self.0["plugin"]
                    .to_owned()
                    .into_string()
                    .map(|_| Self::plugin_opts_to_string(self.0["plugin-opts"].to_owned())),
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
                udp_over_tcp: false,
            }),
            "ssr" => SingboxNodeDef::Shadowsocksr(Shadowsocksr {
                r#type: "shadowsocksr".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                method: self.str_param("cipher"),
                password: self.str_param("password"),
                obfs: Some(self.str_param("obfs")),
                obfs_param: Some(self.str_param("obfs-param")),
                protocol: Some(self.str_param("protocol")),
                protocol_param: Some(self.str_param("protocol-param")),
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
            }),

            "socks5" => SingboxNodeDef::Socks(Socks {
                r#type: "socks".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                version: 5,
                username: self.optional_plugin("username"),
                password: self.optional_plugin("username"),
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
                udp_over_tcp: false,
            }),

            "http" => SingboxNodeDef::Http(Http {
                r#type: "http".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                username: self.optional_plugin("username"),
                password: self.optional_plugin("password"),
                tls: self.parse_tls(),
            }),
            "trojan" => SingboxNodeDef::Trojan(Trojan {
                r#type: "trojan".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                password: self.str_param("password"),
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
                tls: self.parse_tls(),
            }),

            "hysteria" => SingboxNodeDef::Hysteria(Hysteria {
                r#type: "hysteria".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                up: self.0["up"].to_owned().into_string(),
                up_mbps: None,
                down: self.0["down"].to_owned().into_string(),
                down_mbps: None,
                obfs: self.0["obfs"].to_owned().into_string(),
                auth: None,
                auth_str: self.0["auth_str"].to_owned().into_string(),
                recv_window_conn: if self.0["recv_window_conn"].is_badvalue() {
                    None
                } else {
                    Some(self.int_param("recv_window_conn").into())
                },
                recv_window: if self.0["recv_window"].is_badvalue() {
                    None
                } else {
                    Some(self.int_param("recv_window").into())
                },
                disable_mtu_discovery: if self.0["sni"].to_owned().into_string()
                    == Some("true".to_string())
                {
                    Some(true)
                } else {
                    None
                },
                tls: self.parse_tls(),
            }),
            "vmess" => SingboxNodeDef::VMess(VMess {
                r#type: "vmess".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                uuid: self.str_param("uuid"),
                security: Some("auto".to_string()),
                alter_id: if self.0["alertId"].to_owned().into_string().is_some() {
                    Some(self.int_param("alertId"))
                } else {
                    Some(0)
                },
                global_padding: None,
                authenticated_length: None,
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
                tls: self.parse_tls(),
                transport: self.parse_transport(),
            }),

            "vless" => SingboxNodeDef::Vless(Vless {
                r#type: "vless".to_string(),
                tag: self.named(),
                server: self.str_param("server"),
                server_port: self.int_param("port"),
                uuid: self.str_param("uuid"),
                network: match self.0["udp"].as_bool() {
                    Some(true) => None,
                    _ => Some("tcp".to_string()),
                },
                tls: self.parse_tls(),
                packet_encoding: None,
                transport: self.parse_transport(),
            }),
            &_ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct NodeInfo {
    pub list: Vec<serde_json::Value>,
    pub tags: Vec<String>,
}

impl NodeInfo {
    fn sum_proxies(&self) -> serde_json::Value {
        serde_json::to_value(self.list.clone()).unwrap()
    }
    fn sum_tags(&self) -> serde_json::Value {
        serde_json::to_value(self.tags.clone()).unwrap()
    }
    fn proxies_string(&self) -> Result<String> {
        serde_json::to_string(&self.sum_proxies()).map_err(|e| anyhow!(e))
    }

    fn proxies_string_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.sum_proxies()).map_err(|e| anyhow!(e))
    }
}

pub trait Merge {
    fn merge(&mut self, new_json_value: serde_json::Value);
}
impl Merge for serde_json::Value {
    fn merge(&mut self, new_json_value: serde_json::Value) {
        merge(self, &new_json_value);
    }
}
pub fn merge(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(ref mut a), &serde_json::Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(serde_json::Value::Null), v);
            }
        }
        (serde_json::Value::Array(ref mut a), &serde_json::Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (serde_json::Value::Array(ref mut a), &serde_json::Value::Object(ref b)) => {
            a.extend([serde_json::Value::Object(b.clone())]);
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self {
            list: vec![],
            tags: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ClashCfg;

    // #[test]
    // fn get_from_link() {
    //     // with public subscription
    //     let cfg = ClashCfg::new_from_subscribe_link(
    //         "https://raw.githubusercontent.com/aiboboxx/clashfree/main/clash.yml",
    //     )
    //     .unwrap();

    //     dbg!(cfg);
    // }

    #[test]
    fn get_metainfo() {
        // with public subscription
        let cfg = ClashCfg::new_from_subscribe_link(
            "https://raw.githubusercontent.com/aiboboxx/clashfree/main/clash.yml",
        )
        .unwrap();

        dbg!(cfg.get_node_tags());
        dbg!(cfg.get_node_types());
    }

    #[test]
    fn gen_proxies() {
        // with public subscription
        let cfg = ClashCfg::new_from_subscribe_link(
            "https://raw.githubusercontent.com/aiboboxx/clashfree/main/clash.yml",
        )
        .unwrap();

        println!(
            "{}",
            cfg.get_node_data_full()
                .unwrap()
                .proxies_string_pretty()
                .unwrap()
        ); //
    }

    #[test]
    fn vmess_without_network_opt() {
        static YAML_TESTCASE_STR: &str = r#"proxies:
    - {"server":"server","port": 2,"uuid":"uuid","udp":true,"name":"name","type":"vmess","alterId":0}
          "#;

        let vmess_data = ClashCfg::new_from_plain_text(YAML_TESTCASE_STR);

        assert!(vmess_data
            .unwrap()
            .get_node_data_full()
            .unwrap()
            .proxies_string_pretty()
            .is_ok())
    }
}
