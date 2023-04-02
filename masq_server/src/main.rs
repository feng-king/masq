use masquerade_proxy::server::Server;

use std::error::Error;

use argh::FromArgs;



#[derive(FromArgs)]
/// masquerade proxy server. About : https://github.com/jromwu/masquerade
struct Args {
    /// server address to listen. (address:port. 例: 0.0.0.0:443/[::]:443)
    #[argh(option, short = 'b')]
    listen: String,

    /// your cert path. default "./exam.pem". (可以利用 acme.sh + 域名 获取免费证书,或通过 openssl 生成自签证书)
    #[argh(option, default = "String::from(\"./exam.pem\")")]
    cert: String,

    /// your key path. default "./exam.key". 
    #[argh(option, default = "String::from(\"./exam.key\")")]
    key: String,
    
    /// the congestion control for quic connection. default cubic. (设定quic流控,可选: reno,bbr,cubic)
    #[argh(option, short = 'C', default = "String::from(\"cubic\")")]
    congestion_control: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    let op: Args = argh::from_env();
    
    let mut server = Server::new();
    server.bind(&op.listen).await?;

    server.run(&op.cert, &op.key, &op.congestion_control).await
}