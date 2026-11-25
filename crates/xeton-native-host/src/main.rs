use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde_json::Value;
use std::io::{self, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn forward_to_desktop(payload: &Value) -> Result<String, String> {
    let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
    let mut stream = TcpStream::connect("127.0.0.1:18512")
        .await
        .map_err(|e| format!("Xeton desktop not listening on 127.0.0.1:18512: {e}"))?;

    let req = format!(
        "POST /api/download HTTP/1.1\r\nHost: 127.0.0.1:18512\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(req.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    stream.write_all(&body).await.map_err(|e| e.to_string())?;

    let mut resp_buf = Vec::new();
    stream
        .read_to_end(&mut resp_buf)
        .await
        .map_err(|e| e.to_string())?;
    let resp_str = String::from_utf8_lossy(&resp_buf);
    if resp_str.starts_with("HTTP/1.1 200") || resp_str.starts_with("HTTP/1.0 200") {
        Ok("queued".to_string())
    } else {
        Err(format!(
            "Desktop bridge rejected request: {}",
            resp_str.lines().next().unwrap_or("")
        ))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let length = match stdin.read_u32::<NativeEndian>() {
            Ok(len) => len as usize,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };

        let mut buffer = vec![0u8; length];
        stdin.read_exact(&mut buffer)?;

        let request: Value = serde_json::from_slice(&buffer)?;

        let response = match forward_to_desktop(&request).await {
            Ok(status) => serde_json::json!({ "status": status }),
            Err(err) => serde_json::json!({ "status": "error", "error": err }),
        };

        let resp_bytes = serde_json::to_vec(&response)?;
        stdout.write_u32::<NativeEndian>(resp_bytes.len() as u32)?;
        stdout.write_all(&resp_bytes)?;
        stdout.flush()?;
    }

    Ok(())
}
