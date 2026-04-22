//! HTTPS GET client with custome TLS server name based on hyper-tls
//!
//! First parameter is the URL to GET.
//! Second parameter is the TLS server name to negociate.
use bytes::Bytes;
use http_body_util::BodyExt;

use http_body_util::Empty;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::io::{self, AsyncWriteExt as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    let (Some(url), Some(server_name)) = (args.next(), args.next()) else {
        println!("Usage: client <url> <server_name>");
        return Ok(());
    };

    let https = HttpsConnector::new().with_tls_server_name(server_name);

    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    let mut res = client.get(url.parse()?).await?;

    println!("Status:\n{}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    while let Some(frame) = res.body_mut().frame().await {
        let frame = frame?;

        if let Some(d) = frame.data_ref() {
            io::stdout().write_all(d).await?;
        }
    }

    Ok(())
}
