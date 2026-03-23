//! VirusTotal integration
//!
//! This module provides:
//! - SHA-256 hashing of process executables via `/proc/<pid>/exe`
//! - VirusTotal v3 API querying to get a security verdict

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Read;

/// VirusTotal scan report, serialised to JSON for the frontend
#[derive(Serialize, Debug, Clone)]
pub struct VTReport {
    /// SHA-256 hash of the executable (lowercase hex)
    pub hash: String,
    /// Overall verdict
    pub verdict: String,
    /// Number of engines that flagged as malicious
    pub malicious: u64,
    /// Number of engines that flagged as suspicious
    pub suspicious: u64,
    /// Number of engines that returned undetected
    pub undetected: u64,
    /// Total number of engines that scanned the file
    pub total: u64,
    /// Direct link to the VirusTotal report page
    pub permalink: String,
}

/// Reads the process executable at `/proc/<pid>/exe` and computes its SHA-256 hash.
///
/// # Errors
/// Returns an error string if:
/// - The process executable cannot be read (e.g. permission denied for root processes)
/// - An I/O error occurs while reading
pub fn hash_executable(pid: u32) -> Result<String, String> {
    let exe_path = format!("/proc/{}/exe", pid);

    let mut file = std::fs::File::open(&exe_path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            format!(
                "Permission denied reading process {} executable. \
                 You can only scan processes you own.",
                pid
            )
        } else {
            format!("Cannot read process executable (PID {}): {}", pid, e)
        }
    })?;

    let mut hasher = Sha256::new();
    // Read in 64 KiB chunks to avoid loading the whole binary into memory
    let mut buffer = vec![0u8; 65536];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| format!("Read error while hashing PID {}: {}", pid, e))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash_bytes = hasher.finalize();
    Ok(format!("{:x}", hash_bytes))
}

/// Queries the VirusTotal v3 API for a file by its SHA-256 hash.
///
/// # Arguments
/// * `hash`    - SHA-256 hex string (64 chars, lowercase)
/// * `api_key` - VirusTotal API key
///
/// # Errors
/// Returns a user-friendly error string for common HTTP errors.
pub async fn check_virustotal(hash: &str, api_key: &str) -> Result<VTReport, String> {
    if api_key.trim().is_empty() {
        return Err("Please enter your VirusTotal API key.".to_string());
    }

    let url = format!("https://www.virustotal.com/api/v3/files/{}", hash);

    let client = reqwest::Client::builder()
        .user_agent("neohtop/1.2.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .header("x-apikey", api_key.trim())
        .send()
        .await
        .map_err(|e| format!("Network error: {}. Check your internet connection.", e))?;

    match response.status().as_u16() {
        200 => {
            let json: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse VirusTotal response: {}", e))?;

            let stats = &json["data"]["attributes"]["last_analysis_stats"];
            let malicious = stats["malicious"].as_u64().unwrap_or(0);
            let suspicious = stats["suspicious"].as_u64().unwrap_or(0);
            let undetected = stats["undetected"].as_u64().unwrap_or(0);
            let harmless = stats["harmless"].as_u64().unwrap_or(0);
            let total = malicious + suspicious + undetected + harmless;

            let verdict = if malicious > 0 {
                "malicious".to_string()
            } else if suspicious > 0 {
                "suspicious".to_string()
            } else if total > 0 {
                "clean".to_string()
            } else {
                "unknown".to_string()
            };

            Ok(VTReport {
                hash: hash.to_string(),
                verdict,
                malicious,
                suspicious,
                undetected,
                total,
                permalink: format!("https://www.virustotal.com/gui/file/{}", hash),
            })
        }

        401 => {
            Err("Invalid API key. Please check your VirusTotal API key and try again.".to_string())
        }

        404 => {
            // Hash not in VT database — file has never been submitted
            Ok(VTReport {
                hash: hash.to_string(),
                verdict: "unknown".to_string(),
                malicious: 0,
                suspicious: 0,
                undetected: 0,
                total: 0,
                permalink: format!("https://www.virustotal.com/gui/file/{}", hash),
            })
        }

        429 => Err(
            "Rate limit exceeded. VirusTotal free tier allows 4 requests/minute. \
             Please wait a moment and try again."
                .to_string(),
        ),

        status => Err(format!(
            "VirusTotal API returned unexpected status: HTTP {}",
            status
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── hash_executable ──────────────────────────────────────────────────────

    /// The current process always owns its own executable, so hashing it must
    /// succeed and return a valid 64-character lowercase hex SHA-256 digest.
    #[test]
    fn test_hash_self_process_succeeds() {
        let pid = std::process::id();
        let result = hash_executable(pid);
        assert!(
            result.is_ok(),
            "Hashing own process should succeed: {:?}",
            result
        );
    }

    /// SHA-256 output must be exactly 64 hexadecimal characters (256 bits).
    #[test]
    fn test_hash_output_is_valid_sha256_hex() {
        let pid = std::process::id();
        let hash = hash_executable(pid).expect("Hashing own process should not fail");
        assert_eq!(
            hash.len(),
            64,
            "SHA-256 hex digest must be exactly 64 characters"
        );
        assert!(
            hash.chars().all(|c| c.is_ascii_hexdigit()),
            "SHA-256 hex digest must contain only hex characters [0-9a-f], got: {}",
            hash
        );
        assert!(
            hash == hash.to_lowercase(),
            "SHA-256 hex digest must be lowercase, got: {}",
            hash
        );
    }

    /// Hashing the same binary twice must produce identical results
    /// (deterministic — no salt or timestamp involved).
    #[test]
    fn test_hash_is_deterministic() {
        let pid = std::process::id();
        let hash_a = hash_executable(pid).expect("First hash should succeed");
        let hash_b = hash_executable(pid).expect("Second hash should succeed");
        assert_eq!(
            hash_a, hash_b,
            "SHA-256 of the same binary must be identical across calls"
        );
    }

    /// PID 0 has no associated executable on Linux; hashing it must return an
    /// error rather than panicking or producing a garbage result.
    #[test]
    fn test_hash_pid_zero_returns_error() {
        let result = hash_executable(0);
        assert!(result.is_err(), "Hashing PID 0 should return an error");
    }

    /// An obviously invalid PID (u32::MAX) must return an error gracefully.
    #[test]
    fn test_hash_nonexistent_pid_returns_error() {
        let result = hash_executable(u32::MAX);
        assert!(
            result.is_err(),
            "Hashing a non-existent PID should return an error"
        );
        // Error message should be user-friendly, not a raw OS error code
        let err = result.unwrap_err();
        assert!(!err.is_empty(), "Error message must not be empty");
    }

    // ── check_virustotal (offline / input-validation tests) ──────────────────

    /// An empty API key must be rejected immediately, before any network call
    /// is attempted.
    #[tokio::test]
    async fn test_check_virustotal_empty_api_key_rejected() {
        let result = check_virustotal("a".repeat(64).as_str(), "").await;
        assert!(result.is_err(), "Empty API key should be rejected");
        let err = result.unwrap_err();
        assert!(
            err.to_lowercase().contains("api key") || err.to_lowercase().contains("enter"),
            "Error should mention the API key, got: {}",
            err
        );
    }

    /// A whitespace-only API key must also be rejected before any network call.
    #[tokio::test]
    async fn test_check_virustotal_whitespace_api_key_rejected() {
        let result = check_virustotal("a".repeat(64).as_str(), "   ").await;
        assert!(
            result.is_err(),
            "Whitespace-only API key should be rejected"
        );
    }

    // ── VTReport serialisation ────────────────────────────────────────────────

    /// VTReport must be serialisable to JSON (required for the Tauri IPC
    /// bridge to transmit it to the frontend).
    #[test]
    fn test_vt_report_serialises_to_json() {
        let report = VTReport {
            hash: "a".repeat(64),
            verdict: "clean".to_string(),
            malicious: 0,
            suspicious: 0,
            undetected: 72,
            total: 72,
            permalink: format!("https://www.virustotal.com/gui/file/{}", "a".repeat(64)),
        };
        let json = serde_json::to_string(&report);
        assert!(
            json.is_ok(),
            "VTReport should serialise to JSON without error"
        );
        let json_str = json.unwrap();
        assert!(
            json_str.contains("\"verdict\""),
            "JSON must include the verdict field"
        );
        assert!(
            json_str.contains("\"hash\""),
            "JSON must include the hash field"
        );
        assert!(
            json_str.contains("\"permalink\""),
            "JSON must include the permalink field"
        );
    }

    /// All four verdict strings must round-trip through the VTReport struct.
    #[test]
    fn test_vt_report_all_verdicts() {
        for verdict in &["clean", "malicious", "suspicious", "unknown"] {
            let report = VTReport {
                hash: "b".repeat(64),
                verdict: verdict.to_string(),
                malicious: 0,
                suspicious: 0,
                undetected: 0,
                total: 0,
                permalink: String::new(),
            };
            assert_eq!(&report.verdict, verdict);
        }
    }
}
