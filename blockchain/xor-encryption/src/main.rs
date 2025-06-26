struct XorCipher {
    key: Vec<u8>
}

impl XorCipher {
    pub fn new(key: &str) -> Self {
        let mut key_bytes = Vec::new();
        let input_bytes = key.as_bytes();

        for i in 0..input_bytes.len() {
            key_bytes.push(input_bytes[i]);
        }

        Self {
            key: key_bytes
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted_result = Vec::new();
        for i in 0..data.len() {
            let key_byte = self.key[i % self.key.len()];
            let encrypted_byte = data[i] ^ key_byte;
            encrypted_result.push(encrypted_byte);
        }

        encrypted_result
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut decrypted_result = Vec::new();
        for i in 0..data.len() {
            let key_byte = self.key[i % self.key.len()];
            let decrypted_byte = data[i] ^ key_byte;
            decrypted_result.push(decrypted_byte);
        }

        decrypted_result
    }

    pub fn encrypt_string(&self, text: &str) -> Vec<u8> {
        let text_bytes = text.as_bytes();
        self.encrypt(text_bytes)
    }

    pub fn decrypt_to_string(&self, data: &[u8]) -> Result<String, String> {
        let decrypted_bytes = self.decrypt(data);

        match String::from_utf8(decrypted_bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err("Failed to convert bytes to String".to_string()),
        }
    }
}

// // 100101 -> Input
// // 100110 -> Key
// // 000011 -> Output

// 1 
// 1
// 0 % 4 = 0
// 1 % 4 = 1
// 2 % 4 = 2
// 3 % 4 = 3
// 4 % 4 = 0
// 0

fn main() {
    let xor_cipher = XorCipher::new("Blockchain");
    let message = "Hello, World!";

    let encrypted_message = xor_cipher.encrypt_string(message);

    match String::from_utf8(encrypted_message.clone()) {
        Ok(string) => println!("Encrypted Message String: {:?}", string),
        Err(_) => println!("Failed to convert bytes to String"),
    }
    let decrypted = xor_cipher.decrypt_to_string(&encrypted_message).unwrap();

    println!("Original Message: {}", message);
    println!("Encrypted Message: {:?}", encrypted_message);
    println!("Decrypted Message: {}", decrypted);
}