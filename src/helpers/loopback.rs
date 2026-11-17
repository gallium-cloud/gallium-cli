use crate::task_common::error::TaskError;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, header};
use hyper_util::rt::TokioIo;
use snafu::ResultExt;
use std::convert::Infallible;
use std::sync::Mutex;
use tokio::net::TcpListener;

pub struct LoopbackListener {
    listener: TcpListener,
    port: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CallbackRequest {
    Callback {
        code: Option<String>,
        state: Option<String>,
    },
    Other,
}

impl LoopbackListener {
    pub async fn bind() -> Result<Self, TaskError> {
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .whatever_context::<_, TaskError>("bind a loopback port")?;
        let port = listener
            .local_addr()
            .whatever_context::<_, TaskError>("read the loopback port")?
            .port();
        Ok(Self { listener, port })
    }

    pub fn redirect_uri(&self) -> String {
        format!("http://127.0.0.1:{}/callback", self.port)
    }

    pub async fn wait_for_code(
        &self,
        expected_state: &str,
        done_url: &str,
    ) -> Result<String, TaskError> {
        loop {
            let (stream, _) = self
                .listener
                .accept()
                .await
                .whatever_context::<_, TaskError>("accept a loopback connection")?;
            let accepted = Mutex::new(None);
            let service = service_fn(|req| {
                let response = respond(&req, expected_state, done_url, &accepted);
                async move { Ok::<_, Infallible>(response) }
            });
            let _ = http1::Builder::new()
                .keep_alive(false)
                .serve_connection(TokioIo::new(stream), service)
                .await;
            if let Some(code) = accepted.into_inner().ok().flatten() {
                return Ok(code);
            }
        }
    }
}

fn respond<B>(
    req: &Request<B>,
    expected_state: &str,
    done_url: &str,
    accepted: &Mutex<Option<String>>,
) -> Response<Empty<Bytes>> {
    let location = match parse_callback(req) {
        CallbackRequest::Callback {
            code: Some(code),
            state: Some(state),
        } if state == expected_state => {
            if let Ok(mut accepted) = accepted.lock() {
                *accepted = Some(code);
            }
            done_url.to_string()
        }
        CallbackRequest::Callback { code: Some(_), .. } => {
            format!("{done_url}?error=state_mismatch")
        }
        CallbackRequest::Callback { code: None, .. } => format!("{done_url}?error=missing_code"),
        CallbackRequest::Other => return status(StatusCode::NOT_FOUND),
    };
    let mut response = status(StatusCode::FOUND);
    if let Ok(location) = header::HeaderValue::from_str(&location) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn status(status: StatusCode) -> Response<Empty<Bytes>> {
    let mut response = Response::new(Empty::new());
    *response.status_mut() = status;
    response
}

pub fn parse_callback<B>(req: &Request<B>) -> CallbackRequest {
    if req.method() != Method::GET || req.uri().path() != "/callback" {
        return CallbackRequest::Other;
    }
    let mut code = None;
    let mut state = None;
    let query = req.uri().query().unwrap_or("");
    for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
        match key.as_ref() {
            "code" => code = Some(value.into_owned()),
            "state" => state = Some(value.into_owned()),
            _ => {}
        }
    }
    CallbackRequest::Callback { code, state }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    fn request(method: &str, target: &str) -> Request<()> {
        Request::builder()
            .method(method)
            .uri(target)
            .body(())
            .unwrap()
    }

    #[test]
    fn parse_callback_query() {
        assert_eq!(
            parse_callback(&request("GET", "/callback?code=abc&state=s%20t")),
            CallbackRequest::Callback {
                code: Some("abc".into()),
                state: Some("s t".into()),
            }
        );
        assert_eq!(
            parse_callback(&request("GET", "/callback")),
            CallbackRequest::Callback {
                code: None,
                state: None,
            }
        );
    }

    #[test]
    fn other_requests_are_not_callbacks() {
        assert_eq!(
            parse_callback(&request("GET", "/favicon.ico")),
            CallbackRequest::Other
        );
        assert_eq!(
            parse_callback(&request("POST", "/callback?code=x")),
            CallbackRequest::Other
        );
    }

    async fn send(port: u16, target: &str) -> String {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        stream
            .write_all(format!("GET {target} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").as_bytes())
            .await
            .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).await.unwrap();
        response
    }

    #[tokio::test]
    async fn ignore_invalid_requests() {
        let listener = LoopbackListener::bind().await.unwrap();
        let port = listener.port;
        assert_eq!(
            listener.redirect_uri(),
            format!("http://127.0.0.1:{port}/callback")
        );

        let waiting = tokio::spawn(async move {
            listener
                .wait_for_code("expected", "https://console.example/cli/done")
                .await
        });

        let favicon = send(port, "/favicon.ico").await;
        assert!(favicon.starts_with("HTTP/1.1 404"), "{favicon}");

        let wrong_state = send(port, "/callback?code=stolen&state=other").await;
        assert!(
            wrong_state.contains("location: https://console.example/cli/done?error=state_mismatch"),
            "{wrong_state}"
        );

        let no_code = send(port, "/callback?state=expected").await;
        assert!(no_code.contains("?error=missing_code"), "{no_code}");

        let accepted = send(port, "/callback?code=the-code&state=expected").await;
        assert!(
            accepted.contains("location: https://console.example/cli/done\r\n"),
            "{accepted}"
        );

        let code = waiting.await.unwrap().unwrap();
        assert_eq!(code, "the-code");
    }
}
