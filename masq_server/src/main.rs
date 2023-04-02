use masquerade_proxy::server::Server;

use std::env::set_var;
use std::error::Error;

use argh::FromArgs;



#[derive(FromArgs)]
/// masquerade proxy server. About : https://github.com/jromwu/masquerade
struct Args {
    /// server address to listen. (address:port. 例: 0.0.0.0:443/[::]:443)
    #[argh(option, short = 'b')]
    listen: String,

    /// your cert path. default "./exam.pem". (可以利用 acme.sh + 域名 获取免费证书，或通过 openssl 生成自签证书)
    #[argh(option, default = "String::from(\"./exam.pem\")")]
    cert: String,

    /// your key path. default "./exam.key". 
    #[argh(option, default = "String::from(\"./exam.key\")")]
    key: String,

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    let op: Args = argh::from_env();
    
    let mut server = Server::new();
    server.bind(&op.listen).await?;

    server.run(&op.cert, &op.key).await
}