use secp256k1::Secp256k1;

fn main() -> () {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let (secret_key, _) = secp.generate_keypair(&mut rng);
    print!("{}\n", secret_key.display_secret().to_string());
}
