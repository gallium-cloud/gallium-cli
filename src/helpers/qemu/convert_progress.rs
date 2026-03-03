use duct::ReaderHandle;
use std::io;
use std::io::Read;
use std::process::Output;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

#[derive(Default)]
pub struct QemuConvertProgressProvider {
    completed: AtomicBool,
    progress: AtomicU16,
}

impl QemuConvertProgressProvider {
    pub fn read_progress(&self) -> u16 {
        self.progress.load(Ordering::SeqCst)
    }
}

pub fn report_progress(
    progress_provider: Arc<QemuConvertProgressProvider>,
    mut process_handle: ReaderHandle,
) -> io::Result<Option<Output>> {
    let mut chunk = String::new();
    let mut read_buf = [0_u8; 1];
    loop {
        match process_handle.read(&mut read_buf) {
            Ok(1..) => {
                let byte = read_buf[0];
                if byte == b'\r' || byte == b'\n' {
                    chunk.clear();
                } else {
                    chunk.push(byte as char);
                }
                if byte == b')'
                    && let Some(pct) = parse_progress(&chunk)
                {
                    progress_provider.progress.store(pct, Ordering::SeqCst);
                }
            }
            Ok(0) => {
                break;
            }
            Err(_e) => {
                // TODO: What circumstances can this happen? should we fail the import here?
                break;
            }
        }
    }

    let result = process_handle.try_wait().map(|out| out.cloned());

    progress_provider.completed.store(true, Ordering::SeqCst);

    result
}

fn parse_progress(line: &str) -> Option<u16> {
    let line = line.trim().strip_prefix('(')?.strip_suffix("%)")?;

    let parts = line.split_once('/')?;
    let mut pct: f32 = parts.0.parse().ok()?;
    pct *= 100.0;
    if (0.0..=10000.0).contains(&pct) {
        Some(pct as u16)
    } else {
        None
    }
}
