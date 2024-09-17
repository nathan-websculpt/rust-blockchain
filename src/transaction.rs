use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

// bytes serializes the Output (struct) instance into a vector of bytes, which can be used for hashing. 
// it concatenates the bytes of to_addr and value into a single byte vector.
impl Hashable for Output {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

// input_value: The total value of all inputs in the transaction.
// output_value: The total value of all outputs in the transaction.
// input_hashes: A set of hashes for all inputs in the transaction.
// output_hashes: A set of hashes for all outputs in the transaction.
impl Transaction {
    pub fn input_value (&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    pub fn output_value (&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    pub fn input_hashes (&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn output_hashes (&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    //only check for now
    pub fn is_coinbase (&self) -> bool {
        self.inputs.len() == 0
    }
}

// bytes method returns a vector of bytes representing the transaction (by concatenating the bytes of all inputs and outputs)
// basically serializes the transaction into a byte array that can be hashed.
impl Hashable for Transaction {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes()) //flat_map() flattens/appends vectors all together
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}