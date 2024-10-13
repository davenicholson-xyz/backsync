use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct DataPacket {
    length: u32,
    data: Vec<u8>,
}

#[allow(dead_code)]
impl DataPacket {
    pub fn new(data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        DataPacket { length, data }
    }

    pub fn from_raw(raw_data: Vec<u8>) -> Option<Self> {
        if raw_data.len() < 4 {
            return None;
        }

        let length_bytes: [u8; 4] = raw_data[0..4].try_into().unwrap();
        let length = u32::from_be_bytes(length_bytes);
        let data = raw_data[4..].to_vec();

        Some(DataPacket { length, data })
    }

    pub fn from_str(message: &str) -> Self {
        let data = message.as_bytes().to_vec();
        let length = data.len() as u32;
        DataPacket { length, data }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut raw_data = Vec::with_capacity(4 + self.data.len());
        raw_data.extend_from_slice(&self.length.to_be_bytes());
        raw_data.extend_from_slice(&self.data);
        raw_data
    }

    pub fn as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
}
