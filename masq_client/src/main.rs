use masquerade_proxy::client::Socks5Client;
// use tokio::macros;
use std::error::Error;
use std::env::set_var;

use argh::FromArgs;


#[derive(FromArgs)]
/// masquerade proxy. About : https://github.com/jromwu/masquerade
struct Args {
    /// server address to connect. ( address:port. 例: 1.1.1.1:443/[1:1:1:1:1:1:1:1]:443/example:443 )
    #[argh(option, short = 'c')]
    connect: String,

    /// socks5 listen address. default 127.0.0.1080. ( address:port. 例：0.0.0.0:1080/[::]:1080 )
    #[argh(option, default = "String::from(\"127.0.0.1:1080\")")]
    socks5: String,

    // /// http listen address. default none. ( address:port. 例：0.0.0.0:8080/[::1]:8080 )
    // #[argh(option, default = "String::from(\"none\")")]
    // http: String,

    /// whether or not skip verify server cert. ( 当使用此tag，客户端会跳过服务端证书验证，允许非可信证书(自签证书)，一般来说这不安全。)
    #[argh(switch, short = 's')]
    skip_verify: bool,

    /// specify the ca file to verify the server certificate. ( 当服务端使用自签证书时，客户端可指定ca以验证服务端，避免中间人攻击。)
    #[argh(option, default = "String::from(\"none\")")]
    ca: String,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let op: Args = argh::from_env();
    // if op.http!="none" {
    //     let mut http_client = Http1Client::new();
    //     http_client.bind(&op.http).await?;
    //     http_client.run(&op.connect, op.skip_verify, &op.ca).await?;
    //     println!("oooooooooo");
    //     let mut socks_client = Http1Client::new();
    //     println!("pppppppppp");
    //     socks_client.bind(&op.http).await?;
    //     socks_client.run(&op.connect, op.skip_verify, &op.ca).await?;
    // } else {
    //     let mut socks_client = Socks5Client::new();
    //     socks_client.bind(&op.socks5).await?;
    //     socks_client.run(&op.connect, op.skip_verify, &op.ca).await?;
    // }
    let mut socks_client = Socks5Client::new();
    socks_client.bind(&op.socks5).await?;
    socks_client.run(&op.connect, op.skip_verify, &op.ca).await
    // Ok(())
}