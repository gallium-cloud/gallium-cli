use crate::task_common::error::TaskError;
use futures_util::{SinkExt, StreamExt};
use snafu::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(clap::Parser)]
pub(crate) struct ProxyArguments {
    /// wss:// URL to proxy through
    #[command()]
    ws_url: String,
}

pub(crate) async fn proxy(args: &ProxyArguments) -> Result<(), TaskError> {
    let (ws_stream, _) = tokio_tungstenite::connect_async(&args.ws_url)
        .await
        .whatever_context::<_, TaskError>("websocket connection failed")?;

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
                    buffer[..n].to_vec(),
                ))
                .await?;
        }
        Ok::<(), tokio_tungstenite::tungstenite::Error>(())
    });
    let o = tokio::task::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(Ok(msg)) = stream_in.next().await {
            stdout.write_all(&msg.into_data()).await?;
            stdout.flush().await?;
        }
        Ok::<(), std::io::Error>(())
    });

    tokio::select! {
        result = i => {
            result
                .whatever_context::<_, TaskError>("stdin proxy task panicked")?
                .whatever_context::<_, TaskError>("writing to websocket")?;
        }
        result = o => {
            result
                .whatever_context::<_, TaskError>("stdout proxy task panicked")?
                .whatever_context::<_, TaskError>("writing to stdout")?;
        }
    };

    Ok(())
}
