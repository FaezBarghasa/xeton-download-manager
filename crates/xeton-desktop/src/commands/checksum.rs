use std::fs::File;
use std::io::Read;
use openssl::hash::{Hasher, MessageDigest};
use xeton_models::{ChecksumAlgorithm, ChecksumRequest, ChecksumResult};

#[tauri::command]
pub async fn calculate_checksum(request: ChecksumRequest) -> Result<ChecksumResult, String> {
    tokio::task::spawn_blocking(move || {
        let digest = match request.algorithm {
            ChecksumAlgorithm::Md5 => MessageDigest::md5(),
            ChecksumAlgorithm::Sha1 => MessageDigest::sha1(),
            ChecksumAlgorithm::Sha256 => MessageDigest::sha256(),
            ChecksumAlgorithm::Sha512 => MessageDigest::sha512(),
        };

        let mut file = File::open(&request.file_path).map_err(|e| format!("Failed to open file: {e}"))?;
        let mut hasher = Hasher::new(digest).map_err(|e| format!("Hasher error: {e}"))?;
        let mut buffer = [0u8; 65536]; // 64 KB buffer

        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| format!("Read error: {e}"))?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]).map_err(|e| format!("Hash error: {e}"))?;
        }

        let hash_bytes = hasher.finish().map_err(|e| format!("Finish error: {e}"))?;
        let hash_hex: String = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect();

        let matches = request.expected_hash.as_ref().map(|expected| {
            expected.trim().eq_ignore_ascii_case(hash_hex.trim())
        });

        Ok(ChecksumResult {
            file_path: request.file_path,
            hash: hash_hex,
            algorithm: request.algorithm,
            matches,
        })
    })
    .await
    .map_err(|e| format!("Task error: {e}"))?
}
