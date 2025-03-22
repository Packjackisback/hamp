pub struct Message {
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
    //I also want to include the IP of the user somehow in the request
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        let length = self.payload.len() as u32;
        encoded.extend_from_slice(&length.to_be_bytes());
        encoded.extend_from_slice(&self.payload);
        encoded
    }

    pub fn decode(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 4 {
            return None;
        }
        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
        if buffer.len() < 4 + length {
            return None;
        }
        Some(Self {
            payload: buffer[4..4 + length].to_vec(),
        })
    }
}