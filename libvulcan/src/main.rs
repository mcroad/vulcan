use bdk::{
  bitcoin::{
    base64, consensus,
    secp256k1::Secp256k1,
    util::{
      bip32::{self, ExtendedPubKey},
      psbt::PartiallySignedTransaction,
    },
    Address, Network,
  },
  database::MemoryDatabase,
  descriptor,
  keys::{bip39, DerivableKey, ExtendedKey, IntoDescriptorKey},
  SignOptions, Wallet,
};
use std::str::FromStr;
use xyzpub::{convert_version, Version};

#[derive(Debug)]
enum WalletType {
  SingleSig,
  MultiSig,
}
#[derive(Debug)]
enum ScriptType {
  NestedSegwit,
  Segwit,
}

fn get_path(
  network: &Network,
  wallet_type: &WalletType,
  script_type: &ScriptType,
) -> Result<bip32::DerivationPath, bip32::Error> {
  match network {
    Network::Bitcoin => match wallet_type {
      WalletType::SingleSig => match script_type {
        ScriptType::Segwit => bip32::DerivationPath::from_str("m/84h/0h/0h"),
        ScriptType::NestedSegwit => bip32::DerivationPath::from_str("m/49h/0h/0h"),
      },
      WalletType::MultiSig => match script_type {
        ScriptType::Segwit => bip32::DerivationPath::from_str("m/48h/0h/0h/2h"),
        ScriptType::NestedSegwit => bip32::DerivationPath::from_str("m/48h/0h/0h/1h"),
      },
    },
    _ => match wallet_type {
      WalletType::SingleSig => match script_type {
        ScriptType::Segwit => bip32::DerivationPath::from_str("m/84h/1h/0h"),
        ScriptType::NestedSegwit => bip32::DerivationPath::from_str("m/49h/1h/0h"),
      },
      WalletType::MultiSig => match script_type {
        ScriptType::Segwit => bip32::DerivationPath::from_str("m/48h/1h/0h/2h"),
        ScriptType::NestedSegwit => bip32::DerivationPath::from_str("m/48h/1h/0h/1h"),
      },
    },
  }
}

fn _convert_xpub_slip132(
  xpub: &ExtendedPubKey,
  network: &Network,
  wallet_type: &WalletType,
  script_type: &ScriptType,
) -> String {
  let version = match network {
    Network::Bitcoin => match wallet_type {
      WalletType::SingleSig => match script_type {
        ScriptType::Segwit => Version::Zpub,
        ScriptType::NestedSegwit => Version::Ypub,
      },
      WalletType::MultiSig => match script_type {
        ScriptType::Segwit => Version::ZpubMultisig,
        ScriptType::NestedSegwit => Version::YpubMultisig,
      },
    },
    _ => match wallet_type {
      WalletType::SingleSig => match script_type {
        ScriptType::Segwit => Version::Vpub,
        ScriptType::NestedSegwit => Version::Upub,
      },
      WalletType::MultiSig => match script_type {
        ScriptType::Segwit => Version::VpubMultisig,
        ScriptType::NestedSegwit => Version::UpubMultisig,
      },
    },
  };
  return convert_version(xpub.to_string(), &version).unwrap();
}

fn calc_fee(psbt: &PartiallySignedTransaction) -> u64 {
  let mut fee = 0;

  for input in &psbt.inputs {
    if let Some(utxo) = &input.witness_utxo {
      fee = fee + utxo.value;
    } else if let Some(tx) = &input.non_witness_utxo {
      for utxo in &tx.output {
        fee = fee + utxo.value;
      }
    }
  }

  for out in &psbt.global.unsigned_tx.output {
    fee = fee - out.value;
  }

  return fee;
}

fn calc_input(psbt: &PartiallySignedTransaction) -> u64 {
  let mut n = 0;

  for input in &psbt.inputs {
    if let Some(utxo) = &input.witness_utxo {
      n = n + utxo.value;
    } else if let Some(tx) = &input.non_witness_utxo {
      for utxo in &tx.output {
        n = n + utxo.value;
      }
    }
  }

  return n;
}

fn calc_spend_change(
  psbt: &PartiallySignedTransaction,
  network: &Network,
) -> (Vec<(Address, u64)>, Vec<(Address, u64)>) {
  let mut spend: Vec<(Address, u64)> = vec![];
  let mut change: Vec<(Address, u64)> = vec![];

  for (i, txout) in psbt.global.unsigned_tx.output.iter().enumerate() {
    let addr = Address::from_script(&txout.script_pubkey, *network).unwrap();

    // Relying on order and derivations to decide if it's change or spend. likely unreliable
    // TODO: find a better way to check if an output is change
    let out = psbt.outputs.get(i).unwrap();
    if out.bip32_derivation.len() > 0 {
      change.push((addr, txout.value));
    } else {
      spend.push((addr, txout.value));
    }
  }

  return (spend, change);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let secp = Secp256k1::new();

  // testnet
  // let phrase = "shed rebuild nut suffer begin estate vehicle round city sudden fence spoon";
  // signet
  let phrase = "typical bicycle winter insane actor chat upper lazy brother club rib speed bid word caught differ fragile venture merge seed glimpse head exile success";
  let mnemonic = bip39::Mnemonic::parse_in(bip39::Language::English, phrase)?;

  let network = Network::Testnet;
  println!("network: {:?}", network);
  let wallet_type = WalletType::SingleSig;
  println!("wallet_type: {:?}", wallet_type);
  let script_type = ScriptType::Segwit;
  println!("script_type: {:?}", script_type);
  let path = get_path(&network, &wallet_type, &script_type)?;
  println!("Derivation Path: {}", path);

  let xpriv = (mnemonic.into_extended_key()? as ExtendedKey)
    .into_xprv(network)
    .unwrap();

  let xpub = ExtendedPubKey::from_private(&secp, &xpriv);

  let fingerprint = xpub.fingerprint();
  println!("Master Fingerprint: {}", fingerprint);

  let xpriv_at_path = xpriv.derive_priv(&secp, &path)?;
  let xpub_at_path = ExtendedPubKey::from_private(&secp, &xpriv_at_path);
  println!("xpub: {}", xpub_at_path);
  // let slip132_xpub = convert_xpub_slip132(&xpub, &network, &wallet_type, &script_type);

  let specter_xpub = format!(
    "[{}{}]{}",
    fingerprint,
    &path.to_string()[1..],
    xpub_at_path
  )
  .replace("'", "h");
  println!("specter xpub: {}", specter_xpub);

  let descriptor = {
    let send_path = path.child(bip32::ChildNumber::from_normal_idx(0)?);
    let desc_key = (xpriv, send_path).into_descriptor_key()?;
    descriptor!(wpkh(desc_key))?
  };
  let change_descriptor = {
    let change_path = path.child(bip32::ChildNumber::from_normal_idx(1)?);
    let desc_key = (xpriv, change_path).into_descriptor_key().unwrap();
    descriptor!(wpkh(desc_key))?
  };

  let wallet = Wallet::new_offline(
    descriptor,
    Some(change_descriptor),
    network,
    MemoryDatabase::default(),
  )?;

  let serialized_psbt = "cHNidP8BAHECAAAAAVBfV44DutAVcwBhLjMDvKtwbQrmjkH+MFjFXPUqFS9/AQAAAAD9////AkZHmAAAAAAAFgAUZjVsZYVAouW7J4+EBUkSr57hq1cQJwAAAAAAABYAFBAjFi4nxS8Zk94GX+oLLSGwP5sObP4AAAABAHECAAAAAQRANrAsHCjxffzlyW5XgWuP4zvg5uyK69HEtKp36018AAAAAAD9////AhAnAAAAAAAAFgAUe5yb+brLgwDgNAd2FhiCD6ZEJ/vjbpgAAAAAABYAFLXGhIrgQeONcX9YAuONxfsqpR8rSv4AAAEBH+NumAAAAAAAFgAUtcaEiuBB441xf1gC443F+yqlHysiBgPF+tMr9FV6qyy6i32WuHnS+wB30GZgodSYsGaNPeFM7RjiaHvrVAAAgAEAAIAAAACAAQAAAAAAAAAAIgICyh7fjg8uUNT8Kzs5cHSGTKBa4QiUNOGTd7+zkoa0AqwY4mh761QAAIABAACAAAAAgAEAAAABAAAAAAA=";

  println!("\n\nUnsigned PSBT: {:?}\n", serialized_psbt);

  let mut psbt: PartiallySignedTransaction =
    consensus::deserialize(&base64::decode(&serialized_psbt).unwrap())?;

  let fee = calc_fee(&psbt);
  println!("fee: {} sats", fee);

  let input = calc_input(&psbt);
  println!("input: {} sats", input);

  let (spend, change) = calc_spend_change(&psbt, &network);
  let total_spend = spend.iter().fold(0, |prev, (_, value)| prev + value);
  let total_change = change.iter().fold(0, |prev, (_, value)| prev + value);

  println!("spend: {} sats", total_spend);
  println!("change: {} sats", total_change);

  let output = total_spend + total_change;
  println!("input - fee == output: {}", input - fee == output);

  println!("change list: {:#?}", change);
  println!("spend list: {:#?}", spend);
  println!("\n\n");

  if wallet.sign(&mut psbt, SignOptions::default())? {
    println!(
      "Signed PSBT:   {:?}\n",
      base64::encode(&consensus::serialize(&psbt))
    );
  } else {
    println!("Error: could not sign transaction");
  }

  Ok(())
}
