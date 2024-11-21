use rand::rngs::OsRng;
use secp256k1::Secp256k1;

fn main() -> () {
    let secp = Secp256k1::new();
    let mut rng = OsRng; // rng is a number between 0 <= k <= 2^256
    let (secret_key, _) = secp.generate_keypair(&mut rng);
    print!("{}\n", secret_key.display_secret().to_string());
}
