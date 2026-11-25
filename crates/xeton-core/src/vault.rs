// xeton_core::vault — Encrypted Private Download Vault
//
// Implements 64 KiB chunked stream encryption with XChaCha20-Poly1305.
// Header format:
//   - Magic bytes: b"XETNVAULT\x01" (10 bytes)
//   - Salt: [u8; 32]
//   - Nonce prefix: [u8; 24]
// Each chunk (64 KiB plaintext + 16 bytes Poly1305 MAC) is authenticated independently,
// enabling zero-memory buffering and random seek/resume.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use thiserror::Error;

pub const VAULT_CHUNK_SIZE: usize = 64 * 1024; // 64 KiB
pub const VAULT_MAGIC: &[u8; 10] = b"XETNVAULT\x01";

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Authentication failed or corrupted ciphertext chunk")]
    CryptoError,

    #[error("Invalid vault file magic header")]
    InvalidHeader,
}

pub struct VaultSink {
    cipher: XChaCha20Poly1305,
    base_nonce: [u8; 24],
}

impl VaultSink {
    /// Initialize a new vault sink with a 256-bit symmetric key.
    pub fn new(key_bytes: &[u8; 32], base_nonce: [u8; 24]) -> Self {
        let key = Key::from_slice(key_bytes);
        let cipher = XChaCha20Poly1305::new(key);
        Self {
            cipher,
            base_nonce,
        }
    }

    fn derive_chunk_nonce(&self, chunk_index: u64) -> XNonce {
        let mut nonce = self.base_nonce;
        let index_bytes = chunk_index.to_be_bytes();
        // XOR chunk index into the final 8 bytes of the nonce
        for i in 0..8 {
            nonce[16 + i] ^= index_bytes[i];
        }
        *XNonce::from_slice(&nonce)
    }

    /// Encrypt a single chunk at a specific chunk index.
    pub fn encrypt_chunk(&self, chunk_index: u64, plaintext: &[u8]) -> Result<Vec<u8>, VaultError> {
        let nonce = self.derive_chunk_nonce(chunk_index);
        self.cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| VaultError::CryptoError)
    }

    /// Decrypt a single chunk at a specific chunk index.
    pub fn decrypt_chunk(&self, chunk_index: u64, ciphertext: &[u8]) -> Result<Vec<u8>, VaultError> {
        let nonce = self.derive_chunk_nonce(chunk_index);
        self.cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| VaultError::CryptoError)
    }

    /// Encrypt a plaintext file directly into an authenticated vault destination file.
    pub fn encrypt_file(&self, source_path: &Path, dest_path: &Path, salt: &[u8; 32]) -> Result<(), VaultError> {
        let mut src = File::open(source_path)?;
        let mut dst = File::create(dest_path)?;

        // Write Vault Header
        dst.write_all(VAULT_MAGIC)?;
        dst.write_all(salt)?;
        dst.write_all(&self.base_nonce)?;

        let mut buffer = vec![0u8; VAULT_CHUNK_SIZE];
        let mut chunk_index = 0u64;

        loop {
            let bytes_read = src.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let encrypted = self.encrypt_chunk(chunk_index, &buffer[..bytes_read])?;
            let chunk_len = (encrypted.len() as u32).to_be_bytes();
            dst.write_all(&chunk_len)?;
            dst.write_all(&encrypted)?;

            chunk_index += 1;
        }

        dst.flush()?;
        Ok(())
    }

    /// Decrypt an authenticated vault file directly back to plaintext.
    pub fn decrypt_file(&self, vault_path: &Path, dest_path: &Path) -> Result<(), VaultError> {
        let mut src = File::open(vault_path)?;
        let mut dst = File::create(dest_path)?;

        let mut magic = [0u8; 10];
        src.read_exact(&mut magic)?;
        if &magic != VAULT_MAGIC {
            return Err(VaultError::InvalidHeader);
        }

        let mut salt = [0u8; 32];
        src.read_exact(&mut salt)?;

        let mut nonce = [0u8; 24];
        src.read_exact(&mut nonce)?;

        let mut chunk_index = 0u64;
        let mut len_buf = [0u8; 4];

        while let Ok(()) = src.read_exact(&mut len_buf) {
            let chunk_len = u32::from_be_bytes(len_buf) as usize;
            let mut cipher_chunk = vec![0u8; chunk_len];
            src.read_exact(&mut cipher_chunk)?;

            let plaintext = self.decrypt_chunk(chunk_index, &cipher_chunk)?;
            dst.write_all(&plaintext)?;

            chunk_index += 1;
        }

        dst.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_chunk_encrypt_decrypt() {
        let key = [42u8; 32];
        let nonce = [7u8; 24];
        let vault = VaultSink::new(&key, nonce);

        let data = b"Xeton High Speed Private Download Chunk Payload";
        let encrypted = vault.encrypt_chunk(0, data).unwrap();
        assert_ne!(&encrypted[..], &data[..]);

        let decrypted = vault.decrypt_chunk(0, &encrypted).unwrap();
        assert_eq!(&decrypted[..], &data[..]);
    }
}
