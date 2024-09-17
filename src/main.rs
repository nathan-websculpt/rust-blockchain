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
//
// What is Hashing?
// In a nutshell, a hash algorithm consists of a set of irreversible computations that can be
// performed on a datum to generate a (usually) unique byte sequence.
//
//
//
// https://youtu.be/PZGlYa-6U5U?si=2MzizUJxpuDcT8Er&t=795
// Mining Strategy (Algorithm)
//
// 1. Generate new nonce
// 2. Hash bytes (this is the computationally heavy step)
// 3. Check hash against difficulty
//      a. Insufficient? Go back to step 1
//      b. Sufficient? Continue to step 4
// 4. Add block to chain
// 5. Submit to peers, etc. Since this is out-of-scope for this video and we have no networking
//    capabilities implemented (yet!), we'll just skip this step.
//
//
//

use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Init Block".to_owned(),
        0x000fffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    println!("Before Mining: {:?}", &block);

    block.mine();

    println!("After Mining: {:?}", &block);
}
// NOTE: you can run this once, set your nonce to the (output) mined nonce, comment out the block.mine() line, and run again:
// you will get the same block/hash
//
// this is a way to see how the nonce is arbitrarily changing as the computation occurs/loops (until the difficulty is overcome)