use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[derive(clap::Parser)]
pub(crate) struct ProxyArguments {
    /// wss:// URL to proxy through
    #[command()]
    ws_url: String,
}

pub(crate) async fn proxy(args: ProxyArguments) {
    let request = url::Url::parse(&args.ws_url)
        .expect("valid url")
        .into_client_request()
        .expect("valid ws url");
    let (ws_stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .expect("websocket connection");

    let (mut stream_out, mut stream_in) = ws_stream.split();

    let i = tokio::task::spawn(async move {
        let mut stdin = tokio::io::stdin();
        let mut buffer = [0; 4096];
        while let Ok(n) = stdin.read(&mut buffer[..]).await {
            if n == 0 {
                break;
            }
            stream_out
                .send(tokio_tungstenite::tungstenite::protocol::Message::binary(
                    &mut buffer[..n],
                ))
                .await
                .expect("write to stream");
        }
    });
    let o = tokio::task::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(Ok(msg)) = stream_in.next().await {
            stdout
                .write_all(&msg.into_data())
                .await
                .expect("write to stdout");
        }
    });

    tokio::select! {
        _ = i => println!("EOF on stdin"),
        _ = o => println!("EOF on websocket in"),
    };
}
