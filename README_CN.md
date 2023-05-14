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

从 Clash 配置生成 sing-box 配置文件：
```console
./ctos -s "http://<...>" gen > config.json

# 或者
./ctos -s "./config.yaml" gen > config.json

```  
这将从现有的 Clash 配置生成简洁的 `sing-box` 格式配置文件。



解析 Clash 配置：

```console  
./ctos -s "./config.yaml" show

# 通过添加 `-t` 来显示代理名称列表
./ctos -s "./config.yaml" show -t

```

这将从 Clash 配置获取转换后的代理列表，并显示标签名称（如果添加了 --tags）。您可以手动将其附加到 `sing-box` 配置中。

解析`clash`配置文件 并输出`sing-box`节点信息至控制台  

## Nix Flake 支持

在已安装 Nix 的计算机上可以直接运行应用程序：

```console
nix run github:oluceps/clash2sing-box -- -s "<订阅链接>" show --tags
# 或执行其他操作

```
   
## 命令选项
```console
> ./ctos --help
用法：ctos [OPTIONS] <COMMAND>

命令：
  show    从 Clash 配置显示 sing-box 代理信息
  gen     从 Clash 配置生成 sing-box 配置
  append  将新的 Clash 代理追加到现有的 sing-box 配置中 [WIP]
  help    打印此消息或给定子命令的帮助

选项：
  -s, --source <SOURCE>  Clash 配置路径（URL）
  -h, --help             打印帮助
  -V, --version          打印版本

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
