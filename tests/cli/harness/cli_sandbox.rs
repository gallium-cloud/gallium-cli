use crate::harness::{CONSOLE_URL, CliInvocation, Outcome};
use std::process::Stdio;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

pub struct CliSandbox {
    pub api: MockServer,
    home: TempDir,
}

impl CliSandbox {
    pub async fn start() -> Self {
        let api = MockServer::start().await;
        Mock::given(any())
            .respond_with(|request: &Request| {
                ResponseTemplate::new(404).set_body_string(format!(
                    "unexpected request: {} {} {}",
                    request.method,
                    request.url.path(),
                    String::from_utf8_lossy(&request.body)
                ))
            })
            .with_priority(u8::MAX)
            .expect(0)
            .named("mock server received unexpected request")
            .mount(&api)
            .await;
        Self {
            api,
            home: tempfile::tempdir().unwrap(),
        }
    }

    pub fn spawn(&self, args: &[&str]) -> CliInvocation {
        let mut child = Command::new(env!("CARGO_BIN_EXE_ga"))
            .env_clear()
            .env("HOME", self.home.path())
            .env("USERPROFILE", self.home.path())
            .args(["--api-url", &self.api.uri(), "--console-url", CONSOLE_URL])
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let mut stderr = child.stderr.take().unwrap();
        CliInvocation {
            stdin: child.stdin.take(),
            stdout: BufReader::new(child.stdout.take().unwrap()).lines(),
            stdout_read: String::new(),
            stderr: tokio::spawn(async move {
                let mut text = String::new();
                stderr.read_to_string(&mut text).await.unwrap();
                text
            }),
            child,
        }
    }

    pub async fn run(&self, args: &[&str]) -> Outcome {
        let mut running = self.spawn(args);
        running.close_stdin();
        running.finish().await
    }

    /// returns all the request bodies received by the mock server for a given route
    pub async fn requests_for_route(&self, route: &str) -> Vec<serde_json::Value> {
        self.api
            .received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|request| request.url.path() == route)
            .map(|request| request.body_json().unwrap())
            .collect()
    }
}
