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

Parse clash subscribe link and generate pretty-printed indented minimal avaliable sing-box profile  
```console
./ctos --subscribe "<URL>" -g -f -o ./config.json
```  

Parse clash subscribe link  
```console  
./ctos --subscribe "<URL>"  
```

Parse clash `config.yaml`  
```console
./ctos --path <PATH TO config.yaml>  
```   

Converting clash profile to minimal sing-box profile  
```console
./ctos --path <PATH TO config.yaml> -g -f -o ./config.json   
```

## Commands  
```console
> ./ctos --help
Usage: ctos [OPTIONS]

Options:
  -p, --path <PATH>        Read path of clash format config.yaml file
  -s, --subscribe <URL>    Get clash subscription profile by url
  -f                       Output pretty-printed indented JSON
  -g, --gen-profile        Generate minimal avaliable sing-box JSON profile
  -o, --output <PATH>      Output sing-box JSON profile
  -h, --help               Print help information
  -V, --version            Print version information
```


## TODO

- [ ] Refactor, with more elegant converting

- [ ] Clash rule converting

- [ ] Subcommand `update`, to extend new content into config  

- [ ] Simple and easy-to-use web pannel  

- [ ] Auto update with systemd service, with NixOS module

### Credits
+ [Clash](https://github.com/Dreamacro/clash)  
+ [Clash.Meta](https://github.com/MetaCubeX/Clash.Meta)  
+ [sing-box](https://github.com/SagerNet/sing-box)  
+ [json_value_merge](https://github.com/jmfiaschi/json_value_merge)
