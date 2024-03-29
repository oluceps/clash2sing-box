[中文(zh-cn)](README_CN.md) | English  

## Outbound converting Supports  
- [x]  Shadowsocks  
- [x]  http  
- [x]  socks  
- [x]  VMess  
- [x]  Trojan  
- [x]  Hysteria  
- [x]  ShadowsocksR     
- [x]  VLESS          `VLESS` had been abandoned. Consider use `Trojan` instead. See [detail](https://www.v2fly.org/v5/config/proxy/vless.html)  
- [ ]  TUIC           `sing-box` not support `tuic` yet  
- [ ]  ShadowTLS      `Clash.Meta` not support `Shadow-tls` yet  

---

Generate minimal sing-box profile from clash profile (local path or url)  
```console
./ctos -s "http://<...>" gen > config.json

# or
./ctos -s "./config.yaml" gen > config.json
```  
This will generate minimal avaliable sing-box format profile
from exist clash config.

---

Parse clash config 
```console  
./ctos -s "./config.yaml" show  

# show proxies name list by adding `-t`
./ctos -s "./config.yaml" show -t 
```
This will get converted proxies list from clash profile,
and tag names (if `--tags` added).  
You could manualy append it into sing-box config.

---

Append to existed config  
```console  
./ctos -s "./config.yaml" append --dst ./config.json  
```

## web app

demo: https://ctos.magicb.uk

![pic](./.github/web.png)


### self host

```console
git clone https://github.com/oluceps/clash2sing-box.git
cd clash2sing-box/web
trunk serve --open
```

## Commands  
```console
> ./ctos --help
Usage: ctos [OPTIONS] <COMMAND>

Commands:
  show    Show sing-box proxies info from clash profile
  gen     Generate sing-box profile from clash format
  append  Append new clash proxies to existed sing-box profile [WIP]
  help    Print this message or the help of the given subcommand(s)

Options:
  -s, --source <SOURCE>  clash config path (url)
  -h, --help             Print help
  -V, --version          Print version
```

## Nix flake support

Try run application directly on machine with nix installed:

```bash
nix run github:oluceps/clash2sing-box -- -s "<subscribe link>" show --tags
# or any other actions
```


## TODO

- [ ] Clash rule converting

- [x] Subcommand `append`, to extend new content into config  

- [x] Simple and easy-to-use web app  

- [ ] Auto update with systemd service, with NixOS module

### Credits
+ [Dreamacro/Clash](https://github.com/Dreamacro/clash)  
+ [MetaCubeX/Clash.Meta](https://github.com/MetaCubeX/Clash.Meta)  
+ [SagerNet/sing-box](https://github.com/SagerNet/sing-box)  
+ [jmfiaschi/json_value_merge](https://github.com/jmfiaschi/json_value_merge)
+ [thedodd/trunk](https://github.com/thedodd/trunk)  
+ [caddyserver/caddy](https://github.com/caddyserver/caddy)  
+ [yewstack/yew](https://github.com/yewstack/yew)  
