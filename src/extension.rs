use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read, io::Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct CGIExtensionRequest {
    id: Option<u32>,
    method: String,
    parameters: Vec<String>,
}

type CGIExtensionMethod = fn(params: Vec<String>) -> Result<String, String>;

pub struct CGIExtension {
    name: String,
    alias: String,
    description: String,
    methods: HashMap<String, CGIExtensionMethod>,
}

impl CGIExtension {
    /// Creates a new [`CGIExtension`].
    pub fn new(name: String, alias: String, description: String) -> Self {
        CGIExtension {
            name,
            alias,
            description,
            methods: HashMap::new(),
        }
    }

    pub fn export(&mut self, method: String, method_fn: CGIExtensionMethod) {
        self.methods.insert(method, method_fn);
    }

    pub fn verify(&self) -> Vec<u8> {
        let verify_data = serde_json::json!({
            "alias": self.alias,
            "name": self.name,
            "description": self.description,
            "is_cgi": true,
        });

        serde_json::to_vec(&verify_data).expect("Failed to serialize verify data")
    }

    pub async fn execute(&self) {
        let args: Vec<String> = std::env::args().collect();

        if args.contains(&String::from("--ext_verify")) {
            std::io::stdout()
                .write_all(&self.verify())
                .expect("Failed to write verify data");
        } else {
            let mut input_len = String::new();
            let mut reader = std::io::stdin();
            let _ = reader.read_line(&mut input_len);

            let len: usize = input_len.trim().parse().expect("Invalid buffer length.");

            let mut buffer: Vec<u8> = vec![0; len];
            reader.read_exact(&mut buffer).unwrap();
            let command = String::from_utf8(buffer).unwrap();

            let request: CGIExtensionRequest =
                serde_json::from_str(&command).expect("Failed to parse request data.");

            let response = if let Some(method_fn) = self.methods.get(&request.method) {
                method_fn(request.parameters).unwrap_or_else(|err| err)
            } else {
                String::from("Invalid command.")
            };

            std::io::stdout()
                .lock()
                .write_all(response.as_bytes())
                .expect("Failed to write response");
            std::io::stdout().flush().expect("Failed to flush stdout");
        }
    }
}
