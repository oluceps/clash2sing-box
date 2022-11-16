- [x]  Shadowsocks  
- [x]  http  
- [x]  socks  
- [x]  VMess  
- [x]  Trojan  
- [x]  Hysteria  
- [ ]  ~~ShadowTLS~~      `Clash.Meta` not support `Shadow-tls` yet  
- [ ]  ShadowsocksR     
- [ ]  ~~VLESS~~          `VLESS` had been abandoned officially. See [detail](https://www.v2fly.org/v5/config/proxy/vless.html)  

### to parse clash subscribe link  

```console  
./clash2sing-box --subscribe "<link>"  
```

### to parse clash `config.yaml`  

```console
./clash2sing-box --path <PATH_TO_config.yaml>  
```   


```
> ./clash2sing-box --help
Usage: clash2sing-box [OPTIONS]

Options:
  -p, --path <PATH>
  -c, --content <CONTENT>
  -s, --subscribe <SUBSCRIBE>
  -o, --output <OUTPUT>
  -h, --help                   Print help information
  -V, --version                Print version information
```

