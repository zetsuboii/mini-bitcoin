use super::bytes::ToBytesBigEndian;

/// Two rounds of SHA256.
pub fn hash256(data: &[u8]) -> Vec<u8> {
    let first_round = sha256::digest(data).to_bytes_be();
    sha256::digest(first_round.as_slice()).to_bytes_be()
}
