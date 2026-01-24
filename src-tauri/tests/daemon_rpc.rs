use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;
use tempfile::tempdir;

fn pick_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn spawn_daemon(port: u16, data_dir: &std::path::Path, token: &str) -> Child {
    let daemon = env!("CARGO_BIN_EXE_codex_monitor_daemon");
    Command::new(daemon)
        .arg("--listen")
        .arg(format!("127.0.0.1:{port}"))
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--token")
        .arg(token)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn daemon")
}

fn wait_for_port(port: u16, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        if Instant::now() > deadline {
            panic!("daemon did not start listening in time");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn rpc_call(
    reader: &mut BufReader<TcpStream>,
    writer: &mut TcpStream,
    id: u64,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let request = serde_json::json!({
        "id": id,
        "method": method,
        "params": params,
    });
    let payload = serde_json::to_string(&request).unwrap();
    writer.write_all(payload.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.flush().unwrap();

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            return Err("connection closed".to_string());
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(trimmed).unwrap();
        if let Some(resp_id) = value.get("id").and_then(|v| v.as_u64()) {
            if resp_id != id {
                continue;
            }
            if let Some(error) = value.get("error") {
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("remote error");
                return Err(message.to_string());
            }
            return Ok(value.get("result").cloned().unwrap_or(Value::Null));
        }
    }
}

#[test]
fn daemon_rpc_smoke() {
    let data_dir = tempdir().expect("tempdir");
    let port = pick_free_port();
    let token = "test-token";
    let mut child = spawn_daemon(port, data_dir.path(), token);

    wait_for_port(port, Duration::from_secs(5));

    let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    // auth
    rpc_call(
        &mut reader,
        &mut writer,
        1,
        "auth",
        serde_json::json!({"token": token}),
    )
    .expect("auth");

    // ping
    let ping = rpc_call(&mut reader, &mut writer, 2, "ping", Value::Null).unwrap();
    assert_eq!(ping.get("ok").and_then(|v| v.as_bool()), Some(true));

    // list workspaces
    let workspaces = rpc_call(&mut reader, &mut writer, 3, "list_workspaces", Value::Null).unwrap();
    assert!(workspaces.is_array());

    // workspace dir validation
    let workspace_dir = tempdir().expect("workspace dir");
    let is_dir = rpc_call(
        &mut reader,
        &mut writer,
        4,
        "is_workspace_path_dir",
        serde_json::json!({"path": workspace_dir.path().to_string_lossy()}),
    )
    .unwrap();
    assert_eq!(is_dir.as_bool(), Some(true));

    // add workspace (may fail if Codex is missing)
    let codex_bin = std::env::var("CODEX_BIN_FOR_TESTS").ok();
    let add_result = rpc_call(
        &mut reader,
        &mut writer,
        5,
        "add_workspace",
        serde_json::json!({
            "path": workspace_dir.path().to_string_lossy(),
            "codex_bin": codex_bin
        }),
    );

    if let Ok(value) = add_result {
        let id = value.get("id").and_then(|v| v.as_str()).unwrap_or("");
        assert!(!id.is_empty());
        let _ = rpc_call(
            &mut reader,
            &mut writer,
            6,
            "connect_workspace",
            serde_json::json!({"id": id}),
        )
        .unwrap();
    } else if let Err(err) = add_result {
        assert!(
            err.contains("Codex") || err.contains("codex"),
            "unexpected error: {err}"
        );
    }

    let _ = child.kill();
}
