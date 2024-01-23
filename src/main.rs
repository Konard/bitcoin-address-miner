use bitcoin::{Network, PrivateKey, PublicKey, Address};
use bitcoin::secp256k1::{Secp256k1, Signing};
// use std::fmt::Debug;
// use hex::encode_upper;
use std::time::{Instant, Duration};
use std::{fmt::Write, num::ParseIntError};
// use rug::{Assign, Integer, Complete};
use rug::{Integer, Complete};

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

fn append_to_32(bytes: &[u8]) -> Vec<u8> {
    if bytes.len() < 32 {
        let mut result: Vec<u8> = vec![0; 32 - bytes.len()];
        result.extend_from_slice(bytes);
        return result;
    } else {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(bytes);
        return result;
    }
}

fn private_key_to_address<C: Signing>(secp: &Secp256k1<C>, bytes: &[u8]) -> bitcoin::Address {
    let processed_bytes = append_to_32(bytes);
    let private_key = PrivateKey::from_slice(&processed_bytes, Network::Bitcoin).unwrap();
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    // println!("public_key: {}", public_key.to_string());
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    return address;
}


#[tokio::main]
async fn main() {

    // let mut int = Integer::new();
    // int.assign(14);

    let secp = Secp256k1::new();

    // let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

    // println!("{}", secret_key.display_secret());
    // println!("{}", public_key.display_secret());

    // let secp = Secp256k1::new();
    // let secret_key = SecretKey::new(&mut rand::thread_rng());

    // let mut secret_key_bytes = decode_hex("00000000000000000000000000000000000000000000000273a132f43c23acd0").unwrap();

    let target_address = "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so";
    // let target_address = "1LgpDjsqkxF9cTkz3UYSbdTJuvbZ45PKvx";
    // let target_address = "1LeH7eeznEDVeNNmAinoiSjuhNa77izzNo";
    
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000000").unwrap();
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000001cdede8").unwrap();
    // let min_secret_key_bytes = decode_hex("00000000000000000000000000000000000000000000000200000000037d5b38").unwrap();
    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000011127bc0").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000001314b460").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000007cf7f180").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000008f847a80").unwrap();
    // let min_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000002000000012b7a2080").unwrap();
    let min_secret_key_bytes = decode_hex("00000000000000000000000000000000000000000000000200000002451ffbe0").unwrap();
    let max_secret_key_bytes = decode_hex("000000000000000000000000000000000000000000000003ffffffffffffffff").unwrap();

    let int = Integer::parse_radix("00000000000000000000000000000000000000000000000200000002451ffbe0", 16).unwrap();
    println!("{:?}", int);
    let int2 = Integer::parse_radix("000000000000000000000000000000000000000000000003ffffffffffffffff", 16).unwrap();
    // println!("{:?}", int.complete() < int2.complete());
    // println!("{}", int.complete().to_string_radix(16));
    // println!("{}", int2.complete().to_string_radix(16));

    let min_bytes = int.complete().to_digits::<u8>(rug::integer::Order::MsfBe);
    let max_bytes = int2.complete().to_digits::<u8>(rug::integer::Order::MsfBe);

    println!("{:?}", min_bytes);
    println!("{:?}", max_bytes);

    println!("{:?}", min_secret_key_bytes);
    println!("{:?}", max_secret_key_bytes);

    // let mut vec1: Vec<u8> = vec![0; 23];
    // vec1.append(&mut min_bytes);

    let address1 = private_key_to_address(&secp, &min_secret_key_bytes);
    let address2 = private_key_to_address(&secp, &min_bytes);
    println!("address1: {}", address1);
    println!("address2: {}", address2);

    // let min_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000000").unwrap();
    // let max_secret_key_bytes = decode_hex("0000000000000000000000000000000000000000000000020000000000000002").unwrap();

    let mut current_secret_key_bytes = min_secret_key_bytes.to_vec();

    let mut i = 0;
    let mut before = Instant::now();
    let mut average_nanos = 0;
    let addresses_per_batch = 100_000;

    while current_secret_key_bytes != max_secret_key_bytes {
        let address = private_key_to_address(&secp, &current_secret_key_bytes);

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
