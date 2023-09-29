use crate::kernel_types::{AddEntryType, Address, KernelMessage, Message, MessageReceiver, MessageSender, Payload, ProcessId, Request, Response, VfsRequest, VfsResponse};

pub struct Metadata {
    our_node: String,
    path: String,
    receiver: &MessageReceiver,
    sender: &MessageSender,
}

impl Metadata {
    pub async fn len(self) -> u64 {
        let _ = self.sender
            .send(KernelMessage {
                id: rand::random(),
                source: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("key_value".into()),
                },
                target: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("vfs".into()),
                },
                rsvp: None,
                message: Message::Request(Request {
                    inherit: true,
                    expects_response: true,
                    ipc: Some(serde_json::to_string(&VfsRequest::GetEntryLength {
                        identifier: "key_value",
                        full_path: self.path,
                    }).unwrap()),
                    metadata: None,
                }),
                payload: None,
                signed_capabilities: None,
            })
            .await;
        let (_, response) = self.receiver.recv().await.unwrap();
        let KernelMessage { message, .. } = response;
        let Message::Response((Ok(Response { ipc, metadata: _ }), None)) =
            message
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
    receiver: MessageReceiver,
    sender: MessageSender,
}

impl File {
    pub fn metadata(&self) -> std::io::Result<Metadata> {
        Ok(Metadata {
            our_node: self.our_node.clone(),
            path: self.path,
            receiver: &self.receiver,
            sender: &self.sender,
        })
    }
    // fn read_exact(&self) { unimplemented!() }  //  TODO: used in tests
    pub async fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<()> {
        let length = buf.len();
        let _ = self.sender
            .send(KernelMessage {
                id: rand::random(),
                source: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("key_value".into()),
                },
                target: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("vfs".into()),
                },
                rsvp: None,
                message: Message::Request(Request {
                    inherit: true,
                    expects_response: true,
                    ipc: Some(serde_json::to_string(&VfsRequest::GetFileChunk {
                        identifier: "key_value",
                        full_path: self.path,
                        offset,
                        length,
                    }).unwrap()),
                    metadata: None,
                }),
                payload: None,
                signed_capabilities: None,
            })
            .await;
        let (_, response) = self.receiver.recv().await.unwrap();
        let KernelMessage { message, payload, .. } = response;
        // let Message::Response((Ok(Response { ipc, metadata: _ }), None)) =
        //     message
        // else {
        //     panic!("");
        // };
        // let Some(ipc) = ipc else {
        //     panic!("");
        // };
        // let VfsResponse::GetEntry { identifier: _, full_path: _, children: _ } = serde_json::from_str(&ipc).unwrap()
        // else {
        //     panic!("");
        // };
        let Some(Payload { mime: _, bytes }) = payload else {
            panic!("");
        };
        *buf = bytes;
        Ok(())
    }
    pub async fn set_len(&self, size: u64) -> std::io::Result<()> {
        let _ = self.sender
            .send(KernelMessage {
                id: rand::random(),
                source: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("key_value".into()),
                },
                target: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("vfs".into()),
                },
                rsvp: None,
                message: Message::Request(Request {
                    inherit: true,
                    expects_response: true,
                    ipc: Some(serde_json::to_string(&VfsRequest::SetSize {
                        identifier: "key_value",
                        full_path: self.path,
                        size
                    }).unwrap()),
                    metadata: None,
                }),
                payload: None,
                signed_capabilities: None,
            })
            .await;
        let (_, response) = self.receiver.recv().await.unwrap();
        let KernelMessage { message, payload, .. } = response;
        // let Message::Response((Ok(Response { ipc, metadata: _ }), None)) =
        //     message
        // else {
        //     panic!("");
        // };
        // let Some(ipc) = ipc else {
        //     panic!("");
        // };
        // let VfsResponse::GetEntry { identifier: _, full_path: _, children: _ } = serde_json::from_str(&ipc).unwrap()
        // else {
        //     panic!("");
        // };
        Ok(())
    }
    pub fn sync_data(&self) -> std::io::Result<()> { Ok(()) }
    pub async fn write_all_at(&self, buf: &[u8], offset: u64) -> std::io::Result<()> {
        let _ = self.sender
            .send(KernelMessage {
                id: rand::random(),
                source: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("key_value".into()),
                },
                target: Address {
                    node: self.our_node.clone(),
                    process: ProcessId::Name("vfs".into()),
                },
                rsvp: None,
                message: Message::Request(Request {
                    inherit: true,
                    expects_response: true,
                    ipc: Some(serde_json::to_string(&VfsRequest::WriteOffset {
                        identifier: "key_value",
                        full_path: self.path,
                        offset,
                    }).unwrap()),
                    metadata: None,
                }),
                payload: Some(Payload { mime: None, bytes: buf }),
                signed_capabilities: None,
            })
            .await;
        let (_, response) = self.receiver.recv().await.unwrap();
        let KernelMessage { message, payload, .. } = response;
        // let Message::Response((Ok(Response { ipc, metadata: _ }), None)) =
        //     message
        // else {
        //     panic!("");
        // };
        // let Some(ipc) = ipc else {
        //     panic!("");
        // };
        // let VfsResponse::GetEntry { identifier: _, full_path: _, children: _ } = serde_json::from_str(&ipc).unwrap()
        // else {
        //     panic!("");
        // };
        Ok(())
    }
}

pub struct OpenOptions {
    our_node: Option<String>,
    create: bool,
    receiver: Option<MessageReceiver>,
    sender: Option<MessageSender>,
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions { our_node: None, create: false, receiver: None, sender: None }
    }
    pub fn create(self, b: bool) -> Self {
        self.create = b;
        self
    }
    pub fn read(self, b: bool) -> Self { self }
    pub fn receiver(self, receiver: MessageReceiver) -> Self {
        self.receiver = Some(receiver);
        self
    }
    pub fn sender(self, sender: MessageSender) -> Self {
        self.sender = Some(sender);
        self
    }
    pub fn write(self, b: bool) -> Self { self }
    pub async fn open(self, path: String) -> std::io::Result<File> {
        let Some(receiver) = self.receiver else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        let Some(sender) = self.sender else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        let Some(our_node) = self.our_node else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        //  does file already exist?
        let _ = sender
            .send(KernelMessage {
                id: rand::random(),
                source: Address {
                    node: our_node.clone(),
                    process: ProcessId::Name("key_value".into()),
                },
                target: Address {
                    node: our_node.clone(),
                    process: ProcessId::Name("vfs".into()),
                },
                rsvp: None,
                message: Message::Request(Request {
                    inherit: true,
                    expects_response: true,
                    ipc: Some(serde_json::to_string(&VfsRequest::GetEntry {
                        identifier: "key_value",
                        full_path: path,
                    }).unwrap()),
                    metadata: None,
                }),
                payload: None,
                signed_capabilities: None,
            })
            .await;
        let (_, response) = receiver.recv().await.unwrap();
        let KernelMessage { message, payload, .. } = response;
        // let Message::Response((Ok(Response { ipc, metadata: _ }), None)) =
        //     message
        // else {
        //     panic!("");
        // };
        // let Some(ipc) = ipc else {
        //     panic!("");
        // };
        // let VfsResponse::GetEntry { identifier: _, full_path: _, children: _ } = serde_json::from_str(&ipc).unwrap()
        // else {
        //     panic!("");
        // };
        let is_file_exists = match payload {
            None => false,
            Some(_) => true,
        };

        if is_file_exists {
            Ok(File { our_node, path, receiver, sender })
        } else {
            if !self.create {
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
            } else {
                let _ = self.sender
                    .send(KernelMessage {
                        id: rand::random(),
                        source: Address {
                            node: our_node.clone(),
                            process: ProcessId::Name("key_value".into()),
                        },
                        target: Address {
                            node: our_node.clone(),
                            process: ProcessId::Name("vfs".into()),
                        },
                        rsvp: None,
                        message: Message::Request(Request {
                            inherit: true,
                            expects_response: true,
                            ipc: Some(serde_json::to_string(&VfsRequest::Add {
                                identifier: "key_value",
                                full_path: path,
                                entry_type: AddEntryType::NewFile,
                            }).unwrap()),
                            metadata: None,
                        }),
                        payload: Some(Payload { mime: None, bytes: Some(vec![]) }),
                        signed_capabilities: None,
                    })
                    .await;
                let (_, response) = receiver.recv().await.unwrap();
                Ok(File { our_node, path, receiver, sender })
            }
        }
    }
}
