use std::fs::read_to_string;
use std::path::Path;

use anyhow::{anyhow, Result};
use reqwest::header::USER_AGENT;
use yaml_rust::{Yaml, YamlLoader};

use crate::{helper::NodeInfo, ClashCfg, PerClashProxy};

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
        res.text().map_err(|e| anyhow!(e))
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

    /// get node data, include tag name and other factors
    pub fn get_node_data_full(&self) -> Result<NodeInfo> {
        let proxies = self.get_proxies()?;
        let tags = self.get_node_tags();

        let json_proxy_list: Vec<serde_json::Value> = proxies
            .clone()
            .into_iter()
            .map(|i| {
                serde_json::to_value(PerClashProxy::from(i).convert_to_singbox_def().unwrap())
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
