use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;

pub fn sign_data(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).expect("Err PKey");
    // Sign the data
    let mut signer = Signer::new(MessageDigest::sha1(), &keypair).unwrap();
    signer.update(data).unwrap();
    (
        signer.sign_to_vec().expect("Sign Err"),
        keypair.public_key_to_der().expect("Err Public Key"),
    )
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn t1() {
        let data = [1, 2, 3, 4];
        let s = sign_data(&data);
    }
}
