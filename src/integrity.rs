extern crate crypto;


use self::crypto::digest::Digest;
use self::crypto::whirlpool::Whirlpool;
use self::crypto::sha3::Sha3;



pub fn sha3_512(context:String) -> String {


    let mut hasher = Sha3::sha3_512();


    hasher.input_str(&context);
    let sha_512 = hasher.result_str();

    sha_512
}


pub fn whirlpool(context:String) -> String {

    let mut whirlpool_hasher = Whirlpool::new();


    whirlpool_hasher.input_str(&context);
    let whirlpool = whirlpool_hasher.result_str();

    whirlpool
}
