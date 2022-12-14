中文 | [English](README.md)   

## 支持以下协议outbound的转换  
- [x]  Shadowsocks  
- [x]  http  
- [x]  socks  
- [x]  VMess  
- [x]  Trojan  
- [x]  Hysteria  
- [x]  VLESS          `VLESS` 已被弃用并且可能被移除。请考虑使用 Trojan 作为替代品。见[详情](https://www.v2fly.org/v5/config/proxy/vless.html)  
- [x]  ShadowsocksR     
- [ ]  TUIC           `sing-box` 暂不支持 `tuic`    
- [ ]  ShadowTLS      `Clash.Meta` 暂不支持 `Shadow-tls`   

---

解析`clash`订阅链接 并生成格式化后的`sing-box`最小配置文件  
```console
./ctos --subscribe "<URL>" -g -f -o ./config.json
```  

解析`clash`订阅链接 并输出`sing-box`节点信息至控制台  
```console  
./ctos --subscribe "<URL>"  
```

解析`clash`配置文件 并输出`sing-box`节点信息至控制台  
```console
./ctos --path <PATH TO config.yaml>  
```   

转换`clash`配置文件到`sing-box`最小配置文件  
```console
./ctos --path <PATH TO config.yaml> -g -f -o ./config.json   
```
   
## 命令选项
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

### Credits
+ [Clash](https://github.com/Dreamacro/clash)  
+ [Clash.Meta](https://github.com/MetaCubeX/Clash.Meta)  
+ [sing-box](https://github.com/SagerNet/sing-box)  
+ [json_value_merge](https://github.com/jmfiaschi/json_value_merge)
<br>
<br>
<br>
<br>

_Do You Hear The People Sing?_
