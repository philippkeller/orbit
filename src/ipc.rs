//! Local control socket so `orbit --remote …` can drive a running instance.

use std::sync::mpsc::{Receiver, SyncSender};

use anyhow::Result;

use crate::config;

/// Commands accepted over the control socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtlCmd {
    DeleteCurrent,
    NotifyNowPlaying,
}

pub enum CtlReply {
    Ok,
    Err(String),
}

pub struct CtlRequest {
    pub cmd: CtlCmd,
    pub reply: SyncSender<CtlReply>,
}

/// Background listener that forwards control commands to the App tick loop.
pub struct CtlServer {
    rx: Receiver<CtlRequest>,
}

impl CtlServer {
    /// Bind the control socket. Returns `None` if another Orbit already owns it
    /// or the socket cannot be created.
    pub fn bind() -> Option<Self> {
        #[cfg(unix)]
        {
            unix::bind()
        }
        #[cfg(not(unix))]
        {
            None
        }
    }

    pub fn poll(&self) -> Vec<CtlRequest> {
        self.rx.try_iter().collect()
    }
}

impl Drop for CtlServer {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(config::control_socket_path());
    }
}

/// Send a control command to the running Orbit instance and wait for the reply.
pub fn send_cmd(cmd: &str) -> Result<()> {
    #[cfg(unix)]
    {
        unix::send_cmd(cmd)
    }
    #[cfg(not(unix))]
    {
        let _ = cmd;
        anyhow::bail!("orbit --remote is only supported on macOS and Linux")
    }
}

#[cfg(unix)]
mod unix {
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::PathBuf;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use anyhow::{bail, Context, Result};

    use super::*;

    fn socket_path() -> PathBuf {
        config::control_socket_path()
    }

    pub(super) fn bind() -> Option<CtlServer> {
        let path = socket_path();
        // Another live instance already owns the socket.
        if UnixStream::connect(&path).is_ok() {
            return None;
        }
        let _ = std::fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok()?;
        }
        let listener = UnixListener::bind(&path).ok()?;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || serve(listener, path, tx));
        Some(CtlServer { rx })
    }

    fn serve(listener: UnixListener, path: PathBuf, tx: mpsc::Sender<CtlRequest>) {
        for stream in listener.incoming() {
            let Ok(stream) = stream else {
                continue;
            };
            let tx = tx.clone();
            thread::spawn(move || handle_client(stream, tx));
        }
        let _ = std::fs::remove_file(path);
    }

    fn handle_client(stream: UnixStream, tx: mpsc::Sender<CtlRequest>) {
        let Ok(cloned) = stream.try_clone() else {
            return;
        };
        let mut reader = BufReader::new(cloned);
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let cmd = match line.trim() {
            "delete-current" => CtlCmd::DeleteCurrent,
            "notify-now-playing" => CtlCmd::NotifyNowPlaying,
            other => {
                let mut stream = stream;
                let _ = writeln!(
                    stream,
                    "err unknown command: {other} (restart Orbit if you just upgraded)"
                );
                return;
            }
        };
        let (reply_tx, reply_rx) = mpsc::sync_channel(1);
        if tx
            .send(CtlRequest {
                cmd,
                reply: reply_tx,
            })
            .is_err()
        {
            return;
        }
        let reply = reply_rx
            .recv_timeout(Duration::from_secs(30))
            .unwrap_or_else(|_| CtlReply::Err("timed out waiting for Orbit".into()));
        let mut stream = stream;
        match reply {
            CtlReply::Ok => {
                let _ = writeln!(stream, "ok");
            }
            CtlReply::Err(e) => {
                let _ = writeln!(stream, "err {e}");
            }
        }
    }

    pub(super) fn send_cmd(cmd: &str) -> Result<()> {
        let path = socket_path();
        let mut stream = UnixStream::connect(&path).with_context(|| {
            format!(
                "Orbit is not running (no control socket at {})",
                path.display()
            )
        })?;
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        writeln!(stream, "{cmd}")?;
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line = line.trim_end();
        if line == "ok" {
            Ok(())
        } else if let Some(msg) = line.strip_prefix("err ") {
            bail!("{msg}")
        } else if line.is_empty() {
            bail!("no reply from Orbit")
        } else {
            bail!("unexpected reply from Orbit: {line}")
        }
    }
}
