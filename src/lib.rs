pub mod paradigm;
pub mod sb_def;

use anyhow::{anyhow, Result};
// use paradigm::PARADIGM;
use reqwest::header::USER_AGENT;
pub use serde_json::{from_str, to_string, to_string_pretty, to_value};
use std::path::Path;
pub use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct ClashCfg(Yaml);

#[derive(Debug, Clone)]
pub struct SingboxCfg(serde_json::Value);

impl SingboxCfg {}

impl Into<ClashCfg> for Yaml {
    fn into(self) -> ClashCfg {
        ClashCfg(self)
    }
}

impl ClashCfg {
    pub fn new_from_subscribe_link(link: &str) -> Result<Self> {
        Self::to_yaml_data(link, |source| Self::get_subscribe(source)).map(|i| i.into())
    }

    pub fn new_from_config_file(p: impl AsRef<Path> + Copy) -> Result<Self> {
        Self::to_yaml_data(p, |source| {
            read_to_string(source).map_err(|_| anyhow!("error on reading clash config"))
        })
        .map(|i| i.into())
    }

    pub fn to_yaml_data<S, F>(source: S, f: F) -> Result<Yaml>
    where
        S: AsRef<Path>,
        for<'a> F: FnOnce(S) -> Result<String>,
    {
        let raw_config = YamlLoader::load_from_str(f(source).as_ref().unwrap())?.remove(0);
        Ok(raw_config)
    }

    pub fn get_subscribe(link: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(link).header(USER_AGENT, "clash").send()?;
        Ok(res.text()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::ClashCfg;

    #[test]
    fn get_from_link() {
        // with public subscription
        let cfg = ClashCfg::new_from_subscribe_link(
            "https://raw.githubusercontent.com/aiboboxx/clashfree/main/clash.yml",
        )
        .unwrap();

        dbg!(cfg);
    }

    //   #[test]
    //   fn vmess_without_network_opt() {
    //       static YAML_TESTCASE_STR: &str = r#"proxies:
    // - {"server":"server","port": 2,"uuid":"uuid","udp":true,"name":"name","type":"vmess","alterId":0}
    //       "#;

    //       let vmess_data = &YamlLoader::load_from_str(YAML_TESTCASE_STR).unwrap()[0];

    //       assert!(vmess_data.convert().is_ok());
    //   }
}
