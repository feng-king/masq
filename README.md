项目依赖于 Masquerade 的rust实现 [masquerade](https://github.com/jromwu/masquerade).

目前client只实现了socks5入栈，而HTTP的传入有问题，会导致`"零溢"`事件:
```
thread 'main' has overflowed its stack
```
这应该是 [masquerade](https://github.com/jromwu/masquerade) 的问题,我还没能力解决.

## 食用方法
### 自行编译
需要rust编译工具链.请自行学习并使用.

`git clone` 项目到自己的主机. 
```
git clone <项目地址>
```
进入项目根目录
```
// 构建客户端client
cargo build --release -p masq_client
// 构建服务端server
cargo build --release -p masq_server
```
生成的可执行文件会在 `target/release/masq_client` 和 `target/release/masq_server`,
`Windows`则是 `target/release/masq_client.exe` 和 `target/release/masq_server.exe`.
目前,只能编译为release，debug版本同样会导致`"零溢"`事件.

### 运行
#### server
使用 `./masq_server --help` 输出帮助信息.
```
./masq_server --help

Usage: masq_server -b <listen> [--cert <cert>] [--key <key>]

masquerade proxy server. About : https://github.com/jromwu/masquerade

Options:
  -b, --listen      server address to listen. (address:port. 例:
                    0.0.0.0:443/[::]:443)
  --cert            your cert path. default "./exam.pem". (可以利用 acme.sh + 域名
                    获取免费证书，或通过 openssl 生成自签证书)
  --key             your key path. default "./exam.key".
  --help            display usage information
```
你需要准备证书`cert`和密钥`key`,最好是由可信的证书颁发机构签署的证书,
你可以使用`acme.sh`等工具免费颁发可信证书.输入命令以运行:
```
./masq_server -b 0.0.0.0:443 --cert /path/to/114514.com.pem --key /path/to/114514.com.key
```
假如你恰好使用`Linux`系统作为服务器，更巧的是使用`systemd`,那么我这里刚好有一个service文件供你使用，
你需要将你的证书`cert`和密钥`key`和编译好的server端可执行文件masq_server移动到`/usr/local/bin/`目录,
并赋予可执行文件执行权限,将项目根目录下的`masq.service`下载到服务器上，并将其中的证书`cert`和密钥`key`
改为你自己证书的名称并保存后，将文件移动至`/etc/systemd/system/`目录下,运行以下命令:
```
systemctl start masq
```
即可使server工作在443端口.
#### client
使用 `./masq_client --help` 输出帮助信息.
```
./masq_client --help

Usage: masq_client.exe -c <connect> [--socks5 <socks5>] [-s] [--ca <ca>]

masquerade proxy. About : https://github.com/jromwu/masquerade

Options:
  -c, --connect     server address to connect. ( address:port. 例:
                    1.1.1.1:443/[1:1:1:1:1:1:1:1]:443/example:443 )
  --socks5          socks5 listen address. default 127.0.0.1080. ( address:port.
                    例：0.0.0.0:1080/[::]:1080 )
  -s, --skip-verify whether or not skip verify server cert. (
                    当使用此tag，客户端会跳过服务端证书验证，允许非可信证书(自签证书)，一般来说这不安全。)
  --ca              specify the ca file to verify the server certificate. (
                    当服务端使用自签证书时，客户端可指定ca以验证服务端，避免中间人攻击。)
  --help            display usage information

```

你的服务器域名(或ip地址)是 114514.com ,监听 443 端口,你要设置本地socks5端口为 65535 (socks5默
认设置到 `127.0.0.1:1080`),且服务端证书为可信证书，那么你:
```
/path/to/masq_client --connect 114514.com:443 --socks5 127.0.0.1:65535
```
当服务端证书不可信，使用自签证书时，你可以选择跳过证书验证 `--skip-verify` 或指定自签证书
的ca `--ca`并验证证书,如果你恰好不小心同时使用这两个option,那么它会跳过证书验证.
```
/path/to/masq_client --connect 114514.com:443 --socks5 127.0.0.1:65535 --skip-verify

//or

/path/to/masq_client --connect 114514.com:443 --socks5 127.0.0.1:65535 --ca /path/to/ca

```