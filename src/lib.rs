/* Created on 2026.03.01 */
// Copyright (c) 2026 Youcef Lemsafer
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde_with::{DisplayFromStr, serde_as};

// Note about u64 serialization: it is 2026 and JS can't handle u64,
// have to serialize as a string and JS will parse as BigInt which
// is an f64 so values above 2^53-1 are truncated...

/// Request to verify a token
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenVerificationRequest {
    pub token: String,
}

/// Response to a token verification request
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenVerificationResponse {
    pub is_valid: bool,
    pub user_id: Option<String>,
}

#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StartUploadRequest {
    /// Size of the file to upload
    #[serde_as(as = "DisplayFromStr")]
    pub size_in_bytes: u64,
    /// The desired maximum number of downloads for the file
    #[serde_as(as = "DisplayFromStr")]
    pub desired_max_downloads: u64
}

#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseToStartUploadRequest {
    /// Whether the upload can proceed: no error occurred and enough quota to upload specified number of bytes
    pub success: bool,
    /// Message, "OK" in case of success. In case of error or insufficient quota gives additional details
    pub message: String,
    /// Remaining usable quota in bytes
    #[serde_as(as = "DisplayFromStr")]
    pub remaining_quota_bytes: u64,
    /// Session id to use for the upload (is None in case of error)
    pub session_id: Option<String>,
    /// The maximum number of downloads that can be assigned to the file
    #[serde_as(as = "DisplayFromStr")]
    pub applicable_max_downloads: u64
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ChunkReceipt {
    /// Server-generated chunk id
    pub id: String,
    /// Server-generated receipt
    pub receipt: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UploadResponse {
    pub chunk_receipt: ChunkReceipt
}

/// Received on successful upload
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostUploadInfo {
    /// Upload session id
    pub upload_session_id: String,
    /// Size of the uploaded file
    #[serde_as(as = "DisplayFromStr")]
    pub size_in_bytes: u64,
    /// Expiration time hint in hours
    pub expiration_time_hours: u32,
    /// List of chunks
    pub chunks: Vec<ChunkReceipt>,
    /// Manifest
    pub manifest: ChunkReceipt,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UploadCommitResponse {
    pub success: bool,
    pub message: String,
    pub expiration_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiError {
    // Internal errors
    InternalError,

    // Upload session errors
    SessionNotFound,
    SessionExpired,
    SessionAlreadyCommitted,
    SessionIsInvalid,
    TooManyBytesInSession,

    // Storage error
    StorageError,

    // Upload validation errors
    InvalidChunkReceipt,
    ChunksNotSorted,
    DuplicateChunkId,
    InvalidChunkCount,
    InvalidManifestReceipt,
}
