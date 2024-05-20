use bitcoin::base58::encode;
use bitcoin::{Network, PrivateKey, PublicKey, Address};
use bitcoin::secp256k1::{Secp256k1, Signing};
use std::str::FromStr;
use std::fs;
// use std::fmt::Debug;
// use hex::encode_upper;
use std::time::{Instant, Duration};
use std::{fmt::Write, num::ParseIntError};
// use rug::{Assign, Integer, Complete};
use rug::{Integer, Complete, integer, Assign};
use getopt::Opt;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn integer_to_hex(integer: Integer) -> String {
    let bytes = integer.to_digits::<u8>(rug::integer::Order::MsfBe);
    let processed_bytes = append_to_32(bytes);
    return encode_hex(&processed_bytes);
}

// fn check_allowed_chars(starting_letters: &str) -> bool {
//     let allowed_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
//     starting_letters.chars().all(|c| allowed_chars.contains(c))
// }

// async fn mine_address(starting_letters: String) {
//     if !check_allowed_chars(&starting_letters) {
//         println!("Allowed chars: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
//         std::process::exit(1);
//     }

//     let secp = Secp256k1::new();

//     // Initialize an Instant to measure elapsed time.
//     let start_time = Instant::now();

//     loop {
//         // Generate random key pair.
//         let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

//         // Convert secp256k1::PublicKey to bitcoin::PublicKey.
//         let bitcoin_public_key =
//             PublicKey::from(SecpPublicKey::from_slice(&public_key.serialize()[..]).unwrap());

//         // Generate pay-to-pubkey-hash address.
//         let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);

//         // Ignore the specified number of characters when checking for a match.
//         let address_tail = &address.to_string()[1..]; // Start from the second character onward

//         // Uncomment the following line for debug: print the current address being attempted.
//         //println!("Trying address: {}", address);

//         // Check if the address (excluding the specified characters) starts with the desired letters.
//         if address_tail.starts_with(&starting_letters) {
//             println!("Found matching address: {}", address);
//             println!("Private Key: {}", encode_upper(secret_key.as_ref()));
//             println!("Public Key: {}", bitcoin_public_key);
//             println!("Elapsed Time: {:.2} minutes", start_time.elapsed().as_secs_f64() / 60.0);
//             std::process::exit(0);
//         }
//     }
// }

fn increment_bytes(b256: &mut [u8], mut amount: u64) -> u64 {
    let mut i = b256.len() - 1;

    while amount > 0 {
        amount += b256[i] as u64;
        b256[i] = amount as u8;
        amount /= 256;

        if i == 0 { break; }
        i -= 1;
    }

    amount
}

fn less_then(left: &mut [u8], right: &mut [u8]) -> bool {
    let length = left.len();
    let mut i = left.len() - 1;
    while i < length {
        if left[i] < right[i] {
            return true;
        }
        i += 1;
    }
    return false;
}

fn append_to_32(mut bytes: Vec<u8>) -> Vec<u8> {
    if bytes.len() < 32 {
        let mut result: Vec<u8> = vec![0; 32 - bytes.len()];
        result.append(&mut bytes);
        return result;
    } else {
        return bytes;
    }
}

fn private_key_to_address<C: Signing>(secp: &Secp256k1<C>, bytes: Vec<u8>) -> bitcoin::Address {
    let private_key = PrivateKey::from_slice(&bytes, Network::Bitcoin).unwrap();
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    return address;
}

fn integer_private_key_to_address<C: Signing>(secp: &Secp256k1<C>, private_key: Integer) -> bitcoin::Address {
    let bytes = private_key.to_digits::<u8>(rug::integer::Order::MsfBe);
    let processed_bytes = append_to_32(bytes);
    return private_key_to_address(&secp, processed_bytes);
}

fn search_private_key_for_address(min_secret_key_bytes: Vec<u8>, max_secret_key_bytes: Vec<u8>, target_address: String) {
    let secp = Secp256k1::new();
    let mut current_secret_key_bytes = min_secret_key_bytes.to_vec();

    let mut i = 0;
    let mut before = Instant::now();
    let mut average_nanos = 0;
    let addresses_per_batch = 100_000;

    while current_secret_key_bytes != max_secret_key_bytes {
        let address = private_key_to_address(&secp, current_secret_key_bytes.to_vec());

        if address.to_string() == target_address {
            println!("target secret key: {}", encode_hex(&current_secret_key_bytes));
            println!("target address: {}", address);
            break;
        }

        if i % addresses_per_batch == 0 {
            let elapsed_nanoseconds = before.elapsed().as_nanos();
            if average_nanos == 0 {
                average_nanos = elapsed_nanoseconds
            } else {
                average_nanos = (average_nanos + elapsed_nanoseconds) / 2;
            }
            println!("average time for {} addresses: {:.2?}", addresses_per_batch, Duration::from_nanos(average_nanos as u64));
            println!("current secret key: {}", encode_hex(&current_secret_key_bytes));
            before = Instant::now();
        }

        i = i + 1;
        increment_bytes(&mut current_secret_key_bytes, 1);
    }
}

fn search_private_key_for_address_in_integer_range(min_secret_key: Integer, max_secret_key: Integer, target_address: String, reverse: bool) {
    let secp = Secp256k1::new();
    
    let mut i = 0;
    let mut before = Instant::now();
    let mut average_nanos = 0;
    let addresses_per_batch = 100_000;

    let mut current_secret = if reverse { max_secret_key.clone() } else { min_secret_key.clone() };
    let step = if reverse { -1 } else { 1 };
    let limit = if reverse { min_secret_key } else { max_secret_key };

    while current_secret != limit {
        let address = integer_private_key_to_address(&secp, current_secret.clone());

        if address.to_string() == target_address {
            println!("target secret key: {}", integer_to_hex(current_secret.clone()));
            println!("target address: {}", address);
            break;
        }

        if i % addresses_per_batch == 0 {
            let elapsed_nanoseconds = before.elapsed().as_nanos();
            if average_nanos == 0 {
                average_nanos = elapsed_nanoseconds
            } else {
                average_nanos = (average_nanos + elapsed_nanoseconds) / 2;
            }
            println!("average time for {} addresses: {:.2?}", addresses_per_batch, Duration::from_nanos(average_nanos as u64));
            println!("current secret key: {}", integer_to_hex(current_secret.clone()));
            before = Instant::now();
        }

        i = i + 1;
        current_secret.assign(current_secret.clone() + step);
    }
}

fn iterate_private_key_and_address_in_integer_range(min_secret_key: Integer, max_secret_key: Integer, reverse: bool) {
    let secp = Secp256k1::new();
    
    let mut i = 0;
    // let mut before = Instant::now();
    // let mut average_nanos = 0;
    // let addresses_per_batch = 100_000;

    let mut current_secret = if reverse { max_secret_key.clone() } else { min_secret_key.clone() };
    let step = if reverse { -1 } else { 1 };
    let limit = if reverse { min_secret_key } else { max_secret_key };

    while current_secret != limit {
        let address = integer_private_key_to_address(&secp, current_secret.clone());

        println!("{} ↦ {}", address, integer_to_hex(current_secret.clone()));

        // if address.to_string() == target_address {
        //     break;
        // }

        // if i % addresses_per_batch == 0 {
        //     let elapsed_nanoseconds = before.elapsed().as_nanos();
        //     if average_nanos == 0 {
        //         average_nanos = elapsed_nanoseconds
        //     } else {
        //         average_nanos = (average_nanos + elapsed_nanoseconds) / 2;
        //     }
        //     println!("average time for {} addresses: {:.2?}", addresses_per_batch, Duration::from_nanos(average_nanos as u64));
        //     println!("current secret key: {}", integer_to_hex(current_secret.clone()));
        //     before = Instant::now();
        // }

        i = i + 1;
        current_secret.assign(current_secret.clone() + step);
    }
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "r");

    let mut reverse = false;
    // let mut b_flag = String::new();
    loop {
        match opts.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('r', None) => reverse = true,
                // Opt('b', Some(string)) => b_flag = string.clone(),
                _ => unreachable!(),
            }
        }
    }

    println!("{}", reverse);

    // let args = args.split_off(opts.index());

    // let mut int = Integer::new();
    // int.assign(14);

    let secp = Secp256k1::new();

    // let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

    // println!("{}", secret_key.display_secret());
    // println!("{}", public_key.display_secret());

    // let secp = Secp256k1::new();
    // let secret_key = SecretKey::new(&mut rand::thread_rng());

    // let mut secret_key_bytes = decode_hex("00000000000000000000000000000000000000000000000273a132f43c23acd0").unwrap();

    let target_address = String::from_str("13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so").unwrap();
    // let target_address = "1LgpDjsqkxF9cTkz3UYSbdTJuvbZ45PKvx";
    // let target_address = "1LeH7eeznEDVeNNmAinoiSjuhNa77izzNo";
    // let target_address = String::from_str("1DRXKrQk6gxjfa2E7XwD67Un5kpEsycQoD").unwrap();
    // let target_address = String::from_str("1AQADrSG75JmRAnAtFUyyjBmdeQy5Y5aqf").unwrap();
    // let target_address = String::from_str("1Eo3WvHuWKcjcuA6R8KAPFWkNwMEa2WRvT").unwrap();
    // let target_address = String::from_str("1HeN9bQzsFTs1kXaigXyxE6i2FdDbBkn8n").unwrap();
    // let target_address = String::from_str("19UPUkDkAgZp3qmPpSm1U87xAMEC3aRfDu").unwrap();

    
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000000").unwrap();
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000001cdede8").unwrap();
    // let min_secret_key_bytes = decode_hex("00000000000000000000000000000000000000000000000200000000037d5b38").unwrap();
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000011127bc0").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000001314b460").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000007cf7f180").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000008f847a80").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000012b7a2080").unwrap();
    // let min_secret_key_str = "0000000000000000000000000000000000000000000000020000000257af9220";
    // let max_secret_key_str = "000000000000000000000000000000000000000000000003ffffffffffffffff";

    // let min_secret_key_str = fs::read_to_string("./ranges/20000000000000000-3ffffffffffffffff/from").unwrap();
    // let max_secret_key_str = fs::read_to_string("./ranges/20000000000000000-3ffffffffffffffff/to").unwrap();

    let min_secret_key_str = "0000000000000000000000000000000000000000000000000000000000000001";
    let max_secret_key_str = "000000000000000000000000000000000000000000000000000000000000000f";

    // let min_secret_key_str = fs::read_to_string("./ranges/3ffffffffffffffff-20000000000000000/from").unwrap();
    // let max_secret_key_str = fs::read_to_string("./ranges/3ffffffffffffffff-20000000000000000/to").unwrap();

    println!("{}", min_secret_key_str);
    println!("{}", max_secret_key_str);

    let min_secret_key_bytes = decode_hex(&min_secret_key_str).unwrap();
    let max_secret_key_bytes = decode_hex(&max_secret_key_str).unwrap();

    let min_secret_key = Integer::parse_radix(min_secret_key_str, 16).unwrap().complete();
    println!("{:?}", min_secret_key);
    let max_secret_key = Integer::parse_radix(max_secret_key_str, 16).unwrap().complete();
    // println!("{:?}", int.complete() < int2.complete());
    // println!("{}", int.complete().to_string_radix(16));
    // println!("{}", int2.complete().to_string_radix(16));

    let min_bytes = min_secret_key.to_digits::<u8>(rug::integer::Order::MsfBe);
    let max_bytes = max_secret_key.to_digits::<u8>(rug::integer::Order::MsfBe);

    println!("{:?}", min_bytes);
    println!("{:?}", max_bytes);

    println!("{:?}", min_secret_key_bytes);
    println!("{:?}", max_secret_key_bytes);

    // let mut vec1: Vec<u8> = vec![0; 23];
    // vec1.append(&mut min_bytes);

    let address1 = private_key_to_address(&secp, min_secret_key_bytes.to_vec());
    let address2 = integer_private_key_to_address(&secp, min_secret_key.clone());
    println!("address1: {}", address1);
    println!("address2: {}", address2);

    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000000").unwrap();
    // let max_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000002").unwrap();

    // search_private_key_for_address(min_secret_key_bytes, max_secret_key_bytes, target_address);
    // search_private_key_for_address_in_integer_range(min_secret_key, max_secret_key, target_address, reverse);
    iterate_private_key_and_address_in_integer_range(min_secret_key, max_secret_key, reverse);

    // println!("{:?}", min_secret_key_bytes);
    // println!("{:?}", max_secret_key_bytes);

    return;

    // increment_bytes(&mut secret_key_bytes, 1);

    // let secret_key = SecretKey::from_slice(&secret_key_bytes).unwrap();

    // let public_key = SecpPublicKey::from_secret_key(&secp, &secret_key);

    // let bitcoin_public_key =
    // PublicKey::from(SecpPublicKey::from_slice(&public_key.serialize()[..]).unwrap());

    // let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);

    // println!("secret_key: {}", secret_key.display_secret());
    // // println!("{}", public_key);
    // println!("bitcoin_address: {}", address);

    // return;
    // // Ask the user for the starting letters.
    // println!("[BITCOIN KEY/ADDRESS MINING] \n Enter the starting letters for the address: ");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read input");
    // let starting_letters = input.trim().to_string();
    // println!("\n Starting mining...this may take many minutes (or even days) to complete");

    // // Check if all characters in starting_letters are allowed.
    // if !check_allowed_chars(&starting_letters) {
    //     println!("You entered a wrong char - allowed chars: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
    //     std::process::exit(1);
    // }

    // // Run multiple async threads to mine addresses concurrently.
    // let mut handles = vec![];
    // // 50 parallel threads
    // for _ in 0..50 {
    //     let handle = tokio::spawn(mine_address(starting_letters.clone()));
    //     handles.push(handle);
    // }

    // // Wait for all threads to complete.
    // for handle in handles {
    //     handle.await.expect("Error in thread");
    // }
}
