use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use elements::{Address, bitcoin};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    GenerateAddress {

        #[arg(short, long, default_value = "p2wpkh")]
        address_type: String,


        #[arg(short, long)]
        non_confidential: bool,
    },


    AssetInfo {

        #[arg(short, long)]
        asset_id: String,
    },
}


fn generate_address(address_type: &str, confidential: bool) -> Result<String> {

    let secp = bitcoin::secp256k1::Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());


    let bitcoin_pubkey = bitcoin::PublicKey::new(public_key);


    let blinding_key = if confidential {

        Some(public_key)
    } else {
        None
    };


    let address = match address_type {
        "p2pkh" => Address::p2pkh(
            &bitcoin_pubkey,
            blinding_key,
            &elements::AddressParams::LIQUID_TESTNET
        ),
        "p2sh" => {

            let redeem_script = elements::Script::new_p2pk(&bitcoin_pubkey);
            Address::p2sh(
                &redeem_script,
                blinding_key,
                &elements::AddressParams::LIQUID_TESTNET
            )
        },
        "p2wpkh" => Address::p2wpkh(
            &bitcoin_pubkey,
            blinding_key,
            &elements::AddressParams::LIQUID_TESTNET
        ),
        "p2wsh" => {

            let witness_script = elements::Script::new_p2pk(&bitcoin_pubkey);
            Address::p2wsh(
                &witness_script,
                blinding_key,
                &elements::AddressParams::LIQUID_TESTNET
            )
        },
        _ => return Err(anyhow!("Unsupported address type: {}", address_type)),
    };


    println!("Private key (WIF): {}", bitcoin::PrivateKey::new(secret_key, bitcoin::Network::Testnet).to_wif());


    Ok(address.to_string())
}

#[derive(Deserialize, Debug)]
struct AssetResponse {
    asset_id: String,
    #[serde(default)]
    contract: Option<AssetContract>,
    #[serde(default)]
    chain_stats: Option<AssetStats>,
    #[serde(default)]
    mempool_stats: Option<AssetStats>,
}

#[derive(Deserialize, Debug)]
struct AssetContract {
    entity: Option<AssetEntity>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    precision: Option<u8>,
    #[serde(default)]
    ticker: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AssetEntity {
    domain: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AssetStats {
    #[serde(default)]
    tx_count: Option<u64>,
    #[serde(default)]
    issuance_count: Option<u64>,
    #[serde(default)]
    issued_amount: Option<u64>,
    #[serde(default)]
    burned_amount: Option<u64>,
    #[serde(default)]
    has_blinded_issuances: Option<bool>,
    #[serde(default)]
    peg_in_count: Option<u64>,
    #[serde(default)]
    peg_in_amount: Option<u64>,
    #[serde(default)]
    peg_out_count: Option<u64>,
    #[serde(default)]
    peg_out_amount: Option<u64>,
    #[serde(default)]
    burn_count: Option<u64>,
}


fn get_asset_info(asset_id: &str) -> Result<()> {

    let urls = vec![
        format!("https://blockstream.info/liquid/api/asset/{}", asset_id),
        format!("https://blockstream.info/liquidtestnet/api/asset/{}", asset_id),
    ];

    let client = Client::new();
    let mut last_error = None;

    for url in urls {
        println!("Trying to fetch asset info from: {}", url);
        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {

                    return process_asset_response(response);
                } else {
                    last_error = Some(anyhow!("Failed to get asset info: {}", response.status()));
                }
            },
            Err(e) => {
                last_error = Some(anyhow!("Request error: {}", e));
            }
        }
    }


    println!("\nCould not fetch detailed asset information from API.");
    println!("Basic asset information:");
    println!("Asset ID: {}", asset_id);


    if asset_id == "6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d" {
        println!("This is L-BTC (Liquid Bitcoin), the native asset of the Liquid Network.");
        println!("Ticker: L-BTC");
        println!("Precision: 8");
        println!("Description: Liquid Bitcoin is a bitcoin-pegged asset on the Liquid Network.");
        return Ok(());
    }

    println!("\nNote: For more detailed information, you may need to use a different API or check the Liquid Explorer directly.");
    Ok(())
}


fn process_asset_response(response: reqwest::blocking::Response) -> Result<()> {

    let asset: AssetResponse = response.json()?;


    println!("Asset ID: {}", asset.asset_id);

    if let Some(contract) = asset.contract {
        if let Some(name) = contract.name {
            println!("Name: {}", name);
        }
        if let Some(ticker) = contract.ticker {
            println!("Ticker: {}", ticker);
        }
        if let Some(precision) = contract.precision {
            println!("Precision: {}", precision);
        }
        if let Some(entity) = contract.entity {
            if let Some(domain) = entity.domain {
                println!("Domain: {}", domain);
            }
        }
    }

    if let Some(stats) = asset.chain_stats {
        println!("\nChain Statistics:");
        if let Some(tx_count) = stats.tx_count {
            println!("  Transaction Count: {}", tx_count);
        }
        if let Some(issuance_count) = stats.issuance_count {
            println!("  Issuance Count: {}", issuance_count);
        }
        if let Some(issued_amount) = stats.issued_amount {
            println!("  Issued Amount: {}", issued_amount);
        }
        if let Some(burned_amount) = stats.burned_amount {
            println!("  Burned Amount: {}", burned_amount);
        }
        if let Some(has_blinded_issuances) = stats.has_blinded_issuances {
            println!("  Has Blinded Issuances: {}", has_blinded_issuances);
        }

        if let Some(peg_in_count) = stats.peg_in_count {
            println!("  Peg-in Count: {}", peg_in_count);
        }
        if let Some(peg_in_amount) = stats.peg_in_amount {
            println!("  Peg-in Amount: {}", peg_in_amount);
        }
        if let Some(peg_out_count) = stats.peg_out_count {
            println!("  Peg-out Count: {}", peg_out_count);
        }
        if let Some(peg_out_amount) = stats.peg_out_amount {
            println!("  Peg-out Amount: {}", peg_out_amount);
        }
        if let Some(burn_count) = stats.burn_count {
            println!("  Burn Count: {}", burn_count);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateAddress { address_type, non_confidential } => {
            let address = generate_address(address_type, !*non_confidential)?;
            println!("Generated Liquid address: {}", address);
        },
        Commands::AssetInfo { asset_id } => {
            get_asset_info(asset_id)?;
        },
    }

    Ok(())
}
