use openssl::{aes::{AesKey, aes_ige}, sha::Sha256, symm::Mode};

pub fn encrypt_data(data: String, key: &AesKey) -> Vec<u8> {
    let blocks = break_data(data.into_bytes());
    let mut res: Vec<u8> = Vec::new();
    let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";
    for block in blocks {
        let mut tmp_res = [0u8; 16];
        aes_ige(&block, &mut tmp_res, key, &mut iv, Mode::Encrypt);
        for byte in tmp_res {
            res.push(byte);
        }
    }
    res
} 

pub fn decrypt_data(data: Vec<u8>, key: &AesKey) -> Result<String, ()> {
    let blocks = break_data(data);
    let mut res: Vec<u8> = Vec::new();
    let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";
    for block in blocks {
        let mut tmp_res = [0u8; 16];
        aes_ige(&block, &mut tmp_res, key, &mut iv, Mode::Decrypt);
        for byte in tmp_res {
            res.push(byte);
        }
    }
    match String::from_utf8(res) {
        Ok(a) => Ok(a),
        Err(_) => Err(())
    }
}

fn break_data(data: Vec<u8>) -> Vec<[u8; 16]> {
    let mut res = Vec::new();
    let mut count = 0;
    let mut block = [0u8; 16];
    for byte in data {
        block[count] = byte;
        count += 1;
        if count == 8 {
            count = 0;
            res.push(block);
        }
    }
    if count != 0 {
        while count < 8 {
            block[count] = 0;
            count += 1;
        }
        res.push(block);
    }
    res
}

pub fn generate_key(input_password: &str) -> AesKey {
    let mut hash = Sha256::new();
    hash.update(input_password.as_bytes());
    let res = hash.finish();
    let res = &res[0..16];
    dbg!(&res);
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    AesKey::new_encrypt(&res).unwrap()
}
