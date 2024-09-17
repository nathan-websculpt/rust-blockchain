// provides a way to convert any object into a byte representation and then hash that representation using SHA256
pub trait Hashable {
    // returns a vector of bytes representing the object
    fn bytes(&self) -> Vec<u8>;

    // takes the bytes returned by the bytes method, hashes them using the SHA256 algorithm from the crypto_hash crate,
    // and returns the resulting hash as a vector of bytes
    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
