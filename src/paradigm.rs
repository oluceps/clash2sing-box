// add items in line 59
// create new JSON object on outbound list
pub static PARADIGM: &str = r#"
{
  "dns": {
    "servers": [
      {
        "tag": "google",
        "address": "tls:8.8.8.8"
      },
      {
        "tag": "local",
        "address": "223.5.5.5",
        "detour": "direct"
      },
      {
        "tag": "block",
        "address": "rcode:success"
      }
    ],
    "rules": [
      {
        "geosite": "category-ads-all",
        "server": "block",
        "disable_cache": true
      },
      {
        "domain": "mydomain.com",
        "geosite": "cn",
        "server": "local"
      }
    ],
    "strategy": "ipv4_only"
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
