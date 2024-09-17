// Rust Blockchain Resources
// https://www.youtube.com/playlist?list=PLlXBiVibng3CSYcPC_4tYUBXNjQcjgY32
// https://github.com/GeekLaunch/blockchain-rust
// https://github.com/paradigmxyz/reth
// https://github.com/rust-ethereum/evm
//
//

//from: https://youtu.be/vJdT05zl6jk?si=w-42TmYtvgnUOSSf
// Blocks contain this information:

// . Index: this block's location within the list of blocks
// . Payload: any relevant information or events that have occurred for/in the block
// · Timestamp: gives our blockchain a sense of time
// . Nonce: special number used for mining (for PoW verification)
// . Previous block hash: cryptographic fingerprint of previous block
// . Hash: cryptographic fingerprint of all of the above data concatenated together

// What is Hashing?
// In a nutshell, a hash algorithm consists of a set of irreversible computations that can be
// performed on a datum to generate a (usually) unique byte sequence.

use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Init Block".to_owned());

    println!("EMPTY {:?}", &block);

    let h = block.hash();
    println!("block.hash: {:?}", &h);

    block.hash = h;
    println!("After Hashing: {:?}", &block);
}
