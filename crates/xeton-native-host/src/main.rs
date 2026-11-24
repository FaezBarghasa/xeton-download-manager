use std::io::{self, Read, Write};
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde_json::Value;

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

        let response = serde_json::json!({
            "status": "success",
            "received": request
        });

        let resp_bytes = serde_json::to_vec(&response)?;
        stdout.write_u32::<NativeEndian>(resp_bytes.len() as u32)?;
        stdout.write_all(&resp_bytes)?;
        stdout.flush()?;
    }

    Ok(())
}
