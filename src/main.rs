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
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    // Create genesis block
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    // Mine genesis block
    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    // Add genesis block to blockchain
    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    // Create new block - with more transactions
    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 36, //change to 360, ensure it fails
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );

    // Mine other block
    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    // Add block to blockchain
    blockchain
        .update_with_block(block)
        .expect("Failed to add block");
}
