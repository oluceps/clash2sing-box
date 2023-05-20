use crate::{
    sb_def::{
        Http, Hysteria, Shadowsocks, Shadowsocksr, SingboxNodeDef, Socks, Trojan, VMess, Vless,
    },
    PerClashProxy,
};

impl PerClashProxy {
    pub fn convert_to_singbox_def(&self) -> Option<SingboxNodeDef> {
        let proxy_type = self.typed();

        let type_fixed = match proxy_type.as_str() {
            "ss" => "Shadowsocks",
            "trojan" => "Trojan",
            "socks5" => "Socks",
            "hysteria" => "Hysteria",
            "vmess" => "VMess",
            "ssr" => "Shadowsocksr",
            "vless" => "Vless",
            "tuic" => "",
            _ => "",
        };

        macro_rules! create {
            ($n:ident { $($f:ident: $e:expr),* $(,)? }) => {
                if type_fixed == stringify!($n) {
                    return Some(SingboxNodeDef::$n($n{$($f: $e),*}))
                }
            };
        }

        create!(Shadowsocks {
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
        });

        create!(Shadowsocksr {
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
        });

        create!(Socks {
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
        });

        create!(Http {
            r#type: "http".to_string(),
            tag: self.named(),
            server: self.str_param("server"),
            server_port: self.int_param("port"),
            username: self.optional_plugin("username"),
            password: self.optional_plugin("password"),
            tls: self.parse_tls(),
        });
        create!(Trojan {
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
        });

        create!(Hysteria {
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
        });
        create!(VMess {
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
        });

        create!(Vless {
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
        });
        None
    }
}
