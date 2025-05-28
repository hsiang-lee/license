use clap::{Parser, Subcommand};
use license_rs::{crypto::KeyPair, License, LicenseError};
use chrono::{Utc, TimeDelta};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "license-rs")]
#[command(version = "0.1.0")]
#[command(about = "License generation and validation tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new license key
    Generate {
        /// Customer ID
        #[arg(short, long)]
        customer: String,
        
        /// Product ID
        #[arg(short, long)]
        product: String,
        
        /// Days until expiration
        #[arg(short, long, default_value = "365")]
        days: i64,
        
        /// Features (comma separated)
        #[arg(short, long, default_value = "basic")]
        features: String,
        
        /// Bind to current machine
        #[arg(short, long)]
        bind_machine: bool,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        
        /// Private key file path
        #[arg(short, long)]
        private_key: Option<PathBuf>,
    },
    /// Validate a license key
    Validate {
        /// License file path
        #[arg(short, long)]
        license: PathBuf,
        
        /// Public key file path
        #[arg(short, long)]
        public_key: PathBuf,
    },
    /// Generate a new key pair
    Keygen {
        /// Private key output path
        #[arg(short, long)]
        private_key: PathBuf,
        
        /// Public key output path
        #[arg(short, long)]
        public_key: PathBuf,
    },
}

fn main() -> Result<(), LicenseError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            customer,
            product,
            days,
            features,
            bind_machine,
            output,
            private_key,
        } => {
            let key_pair = if let Some(key_path) = private_key {
                let key_bytes = fs::read(key_path)?;
                KeyPair::from_private_key(&key_bytes)?
            } else {
                KeyPair::generate()?
            };

            let expiry = Utc::now() + TimeDelta::try_days(days).unwrap();
            let features = features.split(',').map(|s| s.trim().to_string()).collect();
            let machine_id = if bind_machine {
                Some(license_rs::machine_info::create_provider()?.get_machine_id()?)
            } else {
                None
            };

            let mut license = License::new(customer, product, expiry, features, machine_id);
            license.sign(&key_pair)?;

            fs::write(output, license.to_string()?)?;
            println!("License generated successfully!");
        }
        Commands::Validate { license, public_key } => {
            let license_data = fs::read_to_string(license)?;
            let license = License::from_str(&license_data)?;
            
            let public_key = fs::read(public_key)?;
            license.validate(&public_key)?;
            
            println!("License is valid!");
            println!("Customer: {}", license.customer_id);
            println!("Product: {}", license.product_id);
            println!("Expires: {}", license.expiry_date);
            println!("Features: {}", license.features.join(", "));
        }
        Commands::Keygen {
            private_key,
            public_key,
        } => {
            let key_pair = KeyPair::generate()?;
            fs::write(private_key, key_pair.private_key())?;
            fs::write(public_key, key_pair.public_key())?;
            println!("Key pair generated successfully!");
        }
    }

    Ok(())
}