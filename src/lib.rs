pub mod clash_cfg;
pub mod helper;
pub mod paradigm;
pub mod sb_def;
pub mod utils;

use paradigm::PARADIGM;
pub use serde_json::{from_str, to_string, to_string_pretty, to_value};
pub use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct ClashCfg(Yaml);

#[derive(Debug, Clone)]
pub struct SingboxCfg(serde_json::Value);

impl Into<ClashCfg> for Yaml {
    fn into(self) -> ClashCfg {
        ClashCfg(self)
    }
}

#[derive(Clone, Debug)]
struct PerClashProxy(Yaml);

impl From<Yaml> for PerClashProxy {
    fn from(value: Yaml) -> Self {
        Self(value)
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
    //     );

    //     assert!(cfg.is_ok());
    // }

    #[test]
    fn get_metainfo() {
        // with public subscription
        let cfg = ClashCfg::new_from_config_file("./tests/clash_test.yml").unwrap();

        dbg!(cfg.get_node_tags());
        dbg!(cfg.get_node_types());
    }

    #[test]
    fn gen_proxies() {
        // with public subscription
        let cfg = ClashCfg::new_from_config_file("./tests/clash_test.yml").unwrap();

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

    #[test]
    fn merge_config() {
        // with public subscription
        let cfg = ClashCfg::new_from_config_file("./tests/clash_test.yml").unwrap();

        println!(
            "{}",
            serde_json::to_string_pretty(&cfg.get_node_data_full().unwrap().merge_min()).unwrap()
        );

        assert!(
            serde_json::to_string_pretty(&cfg.get_node_data_full().unwrap().merge_min()).is_ok()
        )
    }
    #[test]
    fn append_cfg() {
        // with public subscription
        let cfg = ClashCfg::new_from_config_file("./tests/clash_test.yml").unwrap();

        let mut v: serde_json::Value = serde_json::from_str(
            std::fs::read_to_string("./tests/prd.json")
                .as_ref()
                .unwrap(),
        )
        .unwrap();

        serde_json::to_string_pretty(&cfg.get_node_data_full().unwrap().append_to(&mut v)).unwrap();
    }
}
