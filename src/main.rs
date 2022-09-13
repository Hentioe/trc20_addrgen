use std::env;

use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;

use trc20_adrrgen::KeyPair;

fn main() {
    let n_arg = if let Some(arg) = env::args().nth(1) {
        arg.parse::<u32>().ok()
    } else {
        None
    };

    if let Some(n) = n_arg {
        println!("ADDRESS                           :                                                         PRIVATE");
        for _i in 0..n {
            account_gen().simple_print();
        }
    } else {
        println!("{}", account_gen());
    }
}

fn account_gen() -> KeyPair {
    let mut rng = OsRng;
    let sk = SecretKey::random(&mut rng);
    let pk = PublicKey::from_secret_key(&sk);

    KeyPair::from(sk, pk)
}
