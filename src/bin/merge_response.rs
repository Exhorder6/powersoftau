extern crate powersoftau;
extern crate rand;
extern crate blake2;
extern crate byteorder;

use powersoftau::*;

use std::fs::OpenOptions;
use std::io::{self, Read, BufReader, Write, BufWriter};

/// merge many response file into bigtranscript
fn main(){

    //response有三部分组成，但是transcript只需要其中的2部分，hash值是算出来的：按顺序解析多个response文件，合并在一起，组成大文件，便于参与者验证。
    //current_accumulator
    //pubkey

    // Create `bigtranscript` in this directory
    let writer = OpenOptions::new()
        .read(false)
        .write(true)
        .create_new(true)
        .open("bigtranscript").expect("unable to create `./bigtranscript` in this directory");

    let writer = BufWriter::new(writer);
    let mut writer = HashWriter::new(writer);

    // load many response file from local disk iterator。把所有参与者生成的new_challenge合并在一起
    for i in 1..2 {
        // Try to load `./challenge` from disk.
        println!("read response{0:>03}", i);
        let reader = OpenOptions::new()
            .read(true)
            .open(format!("./response{0:>03}", i)).expect("unable open `response` in this directory");

        let mut response_reader = HashReader::new(reader);

        // read the hash chain
        {
            let mut tmp = [0; 64];
            response_reader.read_exact(&mut tmp).expect("couldn't read hash of challenge file from response file");
        }

        // Load the response's accumulator
        let current_accumulator = Accumulator::deserialize(&mut response_reader, UseCompression::Yes, CheckForCorrectness::Yes)
            .expect("wasn't able to deserialize the response file's accumulator");

        // Load the response's pubkey
        let public_key = PublicKey::deserialize(&mut response_reader)
            .expect("wasn't able to deserialize the response file's public key");

        // 先写current_accumulator
        current_accumulator.serialize(&mut writer, UseCompression::No)
            .expect("unable to write uncompressed accumulator into the `./bigtranscript` file");

        // 再写pubkey
        public_key.serialize(&mut writer).expect("unable to write public key");
    }
}