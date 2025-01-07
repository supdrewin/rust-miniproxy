# miniproxy

使用Rust实现的简易代理，同时支持HTTP，HTTPS和SOCKS5协议。本项目仅用于学习交流。

## 如何使用

> 首先安装Rust，如何安装请移步[官网](https://www.rust-lang.org/learn/get-started)

本代理分为两部分：`minilocal`和`miniserver`。
`miniserver`运行于网络服务器上，`minilocal`运行于本地。

a. 先通过环境变量设置日志级别(Powershell)

``` pwsh
$env:RUST_LOG="mini=info"
```

b. 服务器上部署`miniserver`，启动的时候会随机产生一个base64编码的密码

``` pwsh
cargo r -r --bin miniserver -- -c config/server.json
```

c. 本地启动`minilocal`，通过`-P`绑定从服务器上回显的base64码

``` pwsh
cargo r -r --bin minilocal -- -c config/local.json
```

c. 进行系统代理设定，代理地址为`127.0.0.0:9998`，或者也可以设置自动代理，PAC文件地址为`http://127.0.0.1:9998/pac`。
   本代理同时支持HTTP，HTTPS和SOCKS5协议。

## 原理及教程

尚在编写中，文档可见[docs](./docs)
