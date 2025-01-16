use bitcoin::{Address, Network, PrivateKey, PublicKey};
use clap::{Parser, ValueEnum};
use rand::rngs::OsRng;
use secp256k1::Secp256k1;

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum PubkeyMode {
    Btc,
}

#[derive(Parser)]
struct Args {
    #[clap(long)]
    pvk: Option<String>,

    #[clap(long, value_enum)]
    pbk: Option<PubkeyMode>,
}

fn main() -> () {
    let args: Args = Args::parse();

    let pvk = match args.pvk {
        Some(pvk) => pvk,
        None => {
            let secp = Secp256k1::new();
            let mut rng = OsRng; // rng is a number between 0 <= k <= 2^256
            let (secret_key, _) = secp.generate_keypair(&mut rng);
            secret_key.display_secret().to_string()
        }
    };
    print!("private key: {pvk}\n");

    let pbk = match args.pbk {
        Some(pbk) => match pbk {
            PubkeyMode::Btc => {
                let secp = bitcoin::key::Secp256k1::new();
                let private_key = PrivateKey::from_slice(
                    &hex::decode(&pvk).expect("Failed to decode private key hex"),
                    Network::Bitcoin,
                )
                .expect("Invalid private key format");
                let public_key = PublicKey::from_private_key(&secp, &private_key);
                Address::p2pkh(&public_key, Network::Bitcoin).to_string()
            }
        },
        None => return,
    };
    print!("public key: {pbk}\n");
}
