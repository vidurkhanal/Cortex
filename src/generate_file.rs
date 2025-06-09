use base64::{engine::general_purpose, Engine as _};

pub struct GenerateFile {
    base64: Option<String>,
    buffer: Option<Vec<u8>>,
    mime_type: String,
}

impl GenerateFile {
    pub fn with_base64(base64: String, mime_type: String) -> Self {
        Self {
            base64: Some(base64),
            buffer: None,
            mime_type,
        }
    }

    pub fn with_buffer(buffer: Vec<u8>, mime_type: String) -> Self {
        Self {
            base64: None,
            buffer: Some(buffer),
            mime_type,
        }
    }

    pub fn get_base64(&mut self) -> String {
        if let Some(base64) = &self.base64 {
            base64.clone()
        } else {
            self.base64 = Some(general_purpose::STANDARD.encode(self.buffer.as_ref().unwrap()));
        }
        self.base64.clone().unwrap()
    }

    pub fn get_buffer(&mut self) -> Vec<u8> {
        if let Some(buffer) = &self.buffer {
            buffer.clone()
        } else {
            self.buffer = Some(general_purpose::STANDARD.decode(self.base64.as_ref().unwrap()));
        }
        self.buffer.clone().unwrap()
    }
}
