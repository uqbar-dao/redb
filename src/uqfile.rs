use crate::kernel_types::{AddEntryType, Address, KernelMessage, Message, MessageReceiver, MessageSender, NetworkError, Payload, ProcessId, Request, Response, VfsRequest, VfsResponse};

pub struct Metadata {
    our_node: String,
    path: String,
    identifier: String,
    send_and_await_response: fn(String, Result<u64, String>, Option<String>, Option<String>, Option<(Option<String>, Vec<u8>)>) -> ((String, Result<u64, String>), Result<(Option<String>, Option<String>), (String, Option<String>)>),
}

impl Metadata {
    pub fn len(self) -> u64 {
        let (_, response) = (self.send_and_await_response)(
            self.our_node.clone(),
            Err("vfs".into()),
            Some(serde_json::to_string(&VfsRequest::GetEntryLength {
                identifier: self.identifier.clone(),
                full_path: self.path.clone(),
            }).unwrap()),
            None,
            None,
        );
        let Ok((ipc, _)) =
            response
        else {
            panic!("");
        };
        let Some::<String>(ipc) = ipc else {
            panic!("");
        };
        let VfsResponse::GetEntryLength { identifier: _, full_path: _, length } = serde_json::from_str(&ipc).unwrap()
        else {
            panic!("");
        };
        length
    }
}

pub struct File {
    our_node: String,
    path: String,
    identifier: String,
    get_payload: fn() -> Option<(Option<String>, Vec<u8>)>,
    send_and_await_response: fn(String, Result<u64, String>, Option<String>, Option<String>, Option<(Option<String>, Vec<u8>)>) -> ((String, Result<u64, String>), Result<(Option<String>, Option<String>), (String, Option<String>)>),
}

impl File {
    pub fn metadata(&self) -> std::io::Result<Metadata> {
        Ok(Metadata {
            our_node: self.our_node.clone(),
            path: self.path.clone(),
            identifier: self.identifier.clone(),
            send_and_await_response: self.send_and_await_response,
        })
    }
    pub fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<()> {
        let length = buf.len();

        let (_, response) = (self.send_and_await_response)(
            self.our_node.clone(),
            Err("vfs".into()),
            Some(serde_json::to_string(&VfsRequest::GetFileChunk {
                identifier: self.identifier.clone(),
                full_path: self.path.clone(),
                offset,
                length: length as u64,
            }).unwrap()),
            None,
            None,
        );
        let Ok(_) =
            response
        else {
            panic!("");
        };
        let payload = (self.get_payload)();
        let Some((_, bytes)) = payload else {
            panic!("");
        };
        buf.copy_from_slice(&bytes[..buf.len()]);
        Ok(())
    }
    pub fn set_len(&self, size: u64) -> std::io::Result<()> {
        let (_, response) = (self.send_and_await_response)(
            self.our_node.clone(),
            Err("vfs".into()),
            Some(serde_json::to_string(&VfsRequest::SetSize {
                identifier: self.identifier.clone(),
                full_path: self.path.clone(),
                size
            }).unwrap()),
            None,
            None,
        );
        let Ok(_) =
            response
        else {
            panic!("");
        };
        Ok(())
    }
    pub fn sync_data(&self) -> std::io::Result<()> { Ok(()) }
    pub fn write_all_at(&self, buf: &[u8], offset: u64) -> std::io::Result<()> {
        let (_, response) = (self.send_and_await_response)(
            self.our_node.clone(),
            Err("vfs".into()),
            Some(serde_json::to_string(&VfsRequest::WriteOffset {
                identifier: self.identifier.clone(),
                full_path: self.path.clone(),
                offset,
            }).unwrap()),
            None,
            None,
        );
        let Ok(_) =
            response
        else {
            panic!("");
        };
        Ok(())
    }
}

pub struct OpenOptions {
    our_node: Option<String>,
    create: bool,
    identifier: Option<String>,
    get_payload: Option<fn() -> Option<(Option<String>, Vec<u8>)>>,
    send_and_await_response: Option<fn(String, Result<u64, String>, Option<String>, Option<String>, Option<(Option<String>, Vec<u8>)>) -> ((String, Result<u64, String>), Result<(Option<String>, Option<String>), (String, Option<String>)>)>,
}

impl OpenOptions {
    pub fn new() -> Self {
        // OpenOptions { our_node: None, create: false, receiver: None, sender: None }
        OpenOptions {
            our_node: None,
            create: false,
            identifier: None,
            get_payload: None,
            send_and_await_response: None,
        }
    }
    pub fn create(mut self, b: bool) -> Self {
        self.create = b;
        self
    }
    pub fn read(self, b: bool) -> Self { self }
    pub fn write(self, b: bool) -> Self { self }
    pub fn identifier(mut self, identifier: String) -> Self {
        self.identifier = Some(identifier);
        self
    }
    pub fn get_payload(mut self, get_payload: fn() -> Option<(Option<String>, Vec<u8>)>) -> Self {
        self.get_payload = Some(get_payload);
        self
    }
    pub fn send_and_await_response(
        mut self,
        send_and_await_response: fn(String, Result<u64, String>, Option<String>, Option<String>, Option<(Option<String>, Vec<u8>)>) -> ((String, Result<u64, String>), Result<(Option<String>, Option<String>), (String, Option<String>)>),
    ) -> Self {
        self.send_and_await_response = Some(send_and_await_response);
        self
    }
    pub fn open(self, path: String) -> std::io::Result<File> {
        let Some(identifier) = self.identifier else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        let Some(get_payload) = self.get_payload else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        let Some(send_and_await_response) = self.send_and_await_response else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        let Some(our_node) = self.our_node else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        //  does file already exist?
        let (_, response) = send_and_await_response(
            our_node.clone(),
            Err("vfs".into()),
            Some(serde_json::to_string(&VfsRequest::GetEntry {
                identifier: identifier.clone(),
                full_path: path.clone(),
            }).unwrap()),
            None,
            None,
        );
        let Ok(_) =
            response
        else {
            panic!("");
        };
        let payload = get_payload();
        let is_file_exists = match payload {
            None => false,
            Some(_) => true,
        };

        if is_file_exists {
            Ok(File { our_node, path, identifier, get_payload, send_and_await_response })
        } else {
            if !self.create {
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
            } else {
                let (_, response) = send_and_await_response(
                    our_node.clone(),
                    Err("vfs".into()),
                    Some(serde_json::to_string(&VfsRequest::Add {
                        identifier: identifier.clone(),
                        full_path: path.clone(),
                        entry_type: AddEntryType::NewFile,
                    }).unwrap()),
                    None,
                    None,
                );
                let Ok(_) =
                    response
                else {
                    panic!("");
                };
                Ok(File { our_node, path, identifier, get_payload, send_and_await_response })
            }
        }
    }
}
