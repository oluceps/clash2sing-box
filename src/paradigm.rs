// add items in line 59
// create new JSON object on outbound list
pub static PARADIGM: &str = r#"
{
  "dns": {
    "rules": [{ "geosite": "cn", "inbound": "tun-in", "server": "local" }],
    "servers": [
      {
        "address": "tls://8.8.4.4:853",
        "address_resolver": "local",
        "address_strategy": "prefer_ipv4",
        "detour": "direct",
        "tag": "google"
      },
      { "address": "223.6.6.6", "detour": "direct", "tag": "local" }
    ],
    "strategy": "prefer_ipv4"
  },
  "inbounds": [
    {
      "type": "tun",
      "inet4_address": "172.19.0.1/30",
      "auto_route": true,
      "strict_route": false,
      "sniff": true
    }
  ],
  "outbounds": [
    {
      "default": "auto",
      "outbounds": [
        "auto",
        "direct",
        "block"
      ],
      "tag": "select",
      "type": "selector"
    },
    {
      "type": "urltest",
      "tag": "auto",

      "outbounds": [

      ],
      "url": "http://www.gstatic.com/generate_204",
      "interval": "1m",
      "tolerance": 50
    },


    {
      "type": "direct",
      "tag": "direct"
    },
    {
      "type": "block",
      "tag": "block"
    },
    {
      "type": "dns",
      "tag": "dns-out"
    }
  ],
  "route": {
    "rules": [
      {
        "protocol": "dns",
        "outbound": "dns-out"
      },
      {
        "geosite": "cn",
        "geoip": [
          "private",
          "cn"
        ],
        "outbound": "direct"
      },
      {
        "geosite": "category-ads-all",
        "outbound": "block"
      }
    ],
    "auto_detect_interface": true
  }
}
"#;
