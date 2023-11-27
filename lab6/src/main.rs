mod ecpoint;
mod signer;

use anyhow::*;
use clap::*;
use num::BigInt;
use signer::Signer;

use std::path::PathBuf;

fn default_signer() -> Signer {
    let p = "57896044618658097711785492504343953926634992332820282019728792003956564821041"
        .parse()
        .unwrap();
    let a = BigInt::from(7);
    let b = "43308876546767276905765904595650931995942111794451039583252968842033849580414"
        .parse()
        .unwrap();
    let x = BigInt::from(2);
    let y = "4018974056539037503335449422937059775635739389905545080690979365213431566280"
        .parse()
        .unwrap();
    let q = "57896044618658097711785492504343953927082934583725450622380973592137631069619"
        .parse()
        .unwrap();
    Signer::new(p, a, b, q, x, y)
}

fn get_k() -> BigInt {
    "53854137677348463731403841147996619241504003434302020712960838528893196233395"
        .parse()
        .unwrap()
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    sign: bool,

    key: PathBuf,
    message: PathBuf,
    sign_loc: PathBuf,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate keys
    #[command(name = "keygen")]
    KeyGen { path: PathBuf },
    /// Sign file
    Sign {
        #[arg(help = "the message to sign")]
        message: PathBuf,
        #[arg(help = "the key to use")]
        private_key: PathBuf,
        #[arg(help = "where to write the sign")]
        sign: PathBuf,
    },
    /// Verify file
    Verify {
        #[arg(help = "the message to verify")]
        message: PathBuf,
        #[arg(help = "the key to use")]
        public_key: PathBuf,
        #[arg(help = "the sign to use")]
        sign: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    // let bytes = include_bytes!("main.rs");
    // g_keys(bytes);

    let cli = Cli::parse();

    let signer = default_signer();

    match &cli.command {
        Commands::KeyGen { path } => {
            let mut path = path.to_owned();

            let (private, public) = signer.gen_keys();

            if path.is_dir() {
                path.push("key")
            };

            std::fs::write(&path, private.to_string())?;
            let mut pub_filename = path.file_name().unwrap().to_owned();
            pub_filename.push(".pub");
            path.set_file_name(pub_filename);
            std::fs::write(&path, public.to_string())
                .context(format!("Writing pub key to {path:?}"))?;
        }
        Commands::Sign {
            message,
            sign,
            private_key: key,
        }
        | Commands::Verify {
            message,
            sign,
            public_key: key,
        } => {
            let message_hash = lab5::gost_hash(std::fs::File::open(message)?)?;
            let message_hash: [u8; 32] = unsafe { std::mem::transmute(message_hash) };
            let message_hash = BigInt::from_signed_bytes_le(&message_hash);

            let signing = matches!(&cli.command, Commands::Sign { .. });

            if signing {
                let private_key = std::fs::read_to_string(key)?.parse()?;
                let res = signer.sign(message_hash, private_key, get_k());
                std::fs::write(sign, format!("{} {}", res.0, res.1))?;
            } else {
                let public_key = std::fs::read_to_string(key)?
                    .parse()
                    .map_err(|_| anyhow!("public key is invalid"))?;

                let sign: Vec<_> = std::fs::read_to_string(sign)?
                    .split_ascii_whitespace()
                    .flat_map(|s| s.parse())
                    .collect();

                let sign: [BigInt; 2] = sign.try_into().map_err(|_| anyhow!("sign is invalid"))?;

                if signer.verify(message_hash, sign.into(), public_key) {
                    println!("Sign is verified!");
                } else {
                    println!("Sign isn't not verified!");
                }
            }
        }
    }

    Ok(())
}
