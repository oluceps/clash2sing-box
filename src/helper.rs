use anyhow::{anyhow, Result};
use yaml_rust::Yaml;

use crate::{
    sb_def::{Tls, Transport, Utls},
    PerClashProxy, PARADIGM,
};

#[allow(unused)]
impl PerClashProxy {
    pub(super) fn str_param(&self, s: &str) -> String {
        self.0[s].to_owned().into_string().unwrap()
    }
    pub(super) fn int_param(&self, s: &str) -> u16 {
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
    pub(super) fn optional_plugin(&self, s: &str) -> Option<String> {
        self.0[s]
            .to_owned()
            .into_string()
            .map(|i| match i.as_str() {
                "obfs" => "obfs-local".to_string(),
                "v2ray-plugin" => "v2ray-plugin".to_string(),
                i => i.to_string(),
            })
    }
    pub(super) fn named(&self) -> String {
        self.0["name"]
            .to_owned()
            .into_string()
            .unwrap_or("unnamed".to_string())
    }

    pub(super) fn typed(&self) -> String {
        self.0["type"]
            .to_owned()
            .into_string()
            .unwrap_or("unknown".to_string())
    }

    pub(super) fn parse_tls(&self) -> Option<Tls> {
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
    pub(super) fn parse_transport(&self) -> Option<Transport> {
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

    pub(super) fn plugin_opts_to_string(opts: Yaml) -> String {
        format!(
            "obfs={};obfs-host={}",
            opts["mode"].to_owned().into_string().unwrap(),
            opts["host"].to_owned().into_string().unwrap()
        )
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

#[derive(Debug)]
pub struct NodeInfo {
    pub list: Vec<serde_json::Value>,
    pub tags: Vec<String>,
}

#[allow(unused)]
impl NodeInfo {
    pub fn sum_proxies(&self) -> serde_json::Value {
        serde_json::to_value(self.list.clone()).unwrap()
    }
    pub fn sum_tags(&self) -> serde_json::Value {
        serde_json::to_value(self.tags.clone()).unwrap()
    }
    pub fn print_tags(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.sum_tags()).map_err(|e| anyhow!(e))
    }
    pub fn proxies_string(&self) -> Result<String> {
        serde_json::to_string(&self.sum_proxies()).map_err(|e| anyhow!(e))
    }

    pub fn proxies_string_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.sum_proxies()).map_err(|e| anyhow!(e))
    }

    pub fn merge_to_value(&self, outer: &mut serde_json::Value) {
        outer["outbounds"].merge(self.sum_proxies());
        outer["outbounds"][1]["outbounds"].merge(self.sum_tags());
    }

    pub fn merge_min(&self) -> serde_json::Value {
        let mut parad: serde_json::Value = serde_json::from_str(PARADIGM).unwrap();
        self.merge_to_value(&mut parad);
        parad
    }
}
