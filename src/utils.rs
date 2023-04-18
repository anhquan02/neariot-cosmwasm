use base64::{Engine, engine::general_purpose};
use cosmwasm_std::{Addr, BlockInfo};

#[allow(unused_imports)]
pub fn generate_id(id:Addr,block_info:BlockInfo) -> String {
    let account_id = id;
    let mut raw_id = account_id.to_owned().to_string();
    raw_id.push_str("_");
    raw_id.push_str(&(block_info.time.to_string()));
    let u8_id = raw_id.as_bytes();
    let vec_id: Vec<u8> = u8_id.iter().cloned().collect();
    // let enc_str = base64::encode(vec_id);
    let enc_str = general_purpose::STANDARD_NO_PAD.encode(vec_id);
    return enc_str;
}
