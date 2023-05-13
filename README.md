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
  -s, --source <SOURCE>  clash config file path(url)
  -u, --url <URL>        clash subscription url
  -h, --help             Print help
  -V, --version          Print version
```

## Nix flake support

Try run application directly on machine with nix installed:

```bash
nix run github:olucep/clash2sing-box -- -s "<subscribe link>" show --tags
# or any other actions
```


## TODO

- [ ] Clash rule converting

- [ ] Subcommand `append`, to extend new content into config  

- [ ] Simple and easy-to-use web pannel  

- [ ] Auto update with systemd service, with NixOS module

- [ ] Convert to [dae](https://github.com/daeuniverse/dae) config

### Credits
+ [Clash](https://github.com/Dreamacro/clash)  
+ [Clash.Meta](https://github.com/MetaCubeX/Clash.Meta)  
+ [sing-box](https://github.com/SagerNet/sing-box)  
+ [json_value_merge](https://github.com/jmfiaschi/json_value_merge)
