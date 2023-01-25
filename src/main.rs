use std::io;
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use rand::Rng;
use crypto_mac::{Mac, NewMac};
use hmac::{Hmac, Mac as HmacTrait};
use sha2::Sha256;
fn main() {
  let mut rand = ChaChaRng::from_entropy();
  let rand_num = &rand.gen_range(1..666);
  let mut password = String::new();
  io::stdin().read_line(&mut password).expect("Uh oh! Unable to read password");
  let mut salt = password + &rand_num.to_string();
  let encrypted = Hmac::<Sha256>::new(&rand_num).unwrap();
  println!("The rancargo add magic-cryptdom number is {rand_num}");
}