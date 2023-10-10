use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VfsRequest {
    pub drive: String,
    pub action: VfsAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VfsAction {
    New,
    Add {
        full_path: String,
        entry_type: AddEntryType,
    },
    Rename {
        full_path: String,
        new_full_path: String,
    },
    Delete(String),
    WriteOffset {
        full_path: String,
        offset: u64,
    },
    SetSize {
        full_path: String,
        size: u64,
    },
    GetPath(u128),
    GetHash(String),
    GetEntry(String),
    GetFileChunk {
        full_path: String,
        offset: u64,
        length: u64,
    },
    GetEntryLength(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddEntryType {
    Dir,
    NewFile,                     //  add a new file to fs and add name in vfs
    ExistingFile { hash: u128 }, //  link an existing file in fs to a new name in vfs
    ZipArchive,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetEntryType {
    Dir,
    File,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VfsResponse {
    Ok,
    Err(VfsError),
    GetPath(Option<String>),
    GetHash(Option<u128>),
    GetEntry {
        // file bytes in payload, if entry was a file
        is_file: bool,
        children: Vec<String>,
    },
    GetFileChunk, // chunk in payload
    GetEntryLength(u64),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VfsError {
    BadDriveName,
    BadDescriptor,
    NoCap,
}

#[allow(dead_code)]
impl VfsError {
    pub fn kind(&self) -> &str {
        match *self {
            VfsError::BadDriveName => "BadDriveName",
            VfsError::BadDescriptor => "BadDescriptor",
            VfsError::NoCap => "NoCap",
        }
    }
}
