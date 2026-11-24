use crate::harness::COMMAND_TIMEOUT;
use tokio::io::{BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout};
use tokio::task::JoinHandle;

#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct CliInvocation {
    pub child: Child,
    pub stdin: Option<ChildStdin>,
    pub stdout: Lines<BufReader<ChildStdout>>,
    pub stdout_read: String,
    pub stderr: JoinHandle<String>,
}

impl CliInvocation {
    pub fn close_stdin(&mut self) {
        self.stdin = None;
    }

    /// Wait for the binary to exit or the [`COMMAND_TIMEOUT`] to expire
    pub async fn finish(mut self) -> Outcome {
        let exit = tokio::time::timeout(COMMAND_TIMEOUT, async {
            while self.read_stdout_line().await.is_some() {}
            self.child.wait().await.unwrap()
        })
        .await;
        let Ok(status) = exit else {
            panic!(
                "timed out waiting for command. output:\n{}",
                self.stdout_read
            );
        };
        Outcome {
            code: status.code(),
            stdout: self.stdout_read,
            stderr: self.stderr.await.unwrap(),
        }
    }

    async fn read_stdout_line(&mut self) -> Option<String> {
        let line = self.stdout.next_line().await.unwrap()?;
        self.stdout_read.push_str(&line);
        self.stdout_read.push('\n');
        Some(line)
    }
}
