use bdk::{
  bitcoin::{
    base64, consensus,
    secp256k1::Secp256k1,
    util::{bip32, psbt::PartiallySignedTransaction},
    Address, Network,
  },
  database::MemoryDatabase,
  descriptor,
  keys::{bip39, DerivableKey, DescriptorSecretKey, DescriptorSinglePriv, ExtendedKey},
  miniscript::{
    descriptor::{DescriptorXKey, Wildcard},
    DescriptorPublicKey,
  },
  SignOptions, Wallet,
};
use bip39 as bip39r;
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
  xpub: &bip32::ExtendedPubKey,
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

fn get_wallet(
  root: &bip32::ExtendedPrivKey,
  path: &bip32::DerivationPath,
  network: Network,
) -> Result<Wallet<(), MemoryDatabase>, bdk::Error> {
  let external = {
    let send_path = path.child(bip32::ChildNumber::from_normal_idx(0)?);
    let desc_key = root.into_descriptor_key(None, send_path)?;
    descriptor!(wpkh(desc_key))?
  };
  println!("receive: {}", external.0);
  let internal = {
    let change_path = path.child(bip32::ChildNumber::from_normal_idx(1)?);
    let desc_key = root.into_descriptor_key(None, change_path)?;
    descriptor!(wpkh(desc_key))?
  };
  println!("change:  {}", internal.0);

  return Wallet::new_offline(external, Some(internal), network, MemoryDatabase::default());
}

/// [0] Version
/// [1] Script type
/// [2] Word count
/// [3:3 + word_count * 4] mnemonic seed indeces as 4-digit numbers
/// [3 + word_count * 4:] the rest is interpreted as 3-digits per unicode char. makes up the path after the "m/"
fn parse_extended_seedqr(qr: &str) -> Result<(Vec<&str>, bip32::DerivationPath, ScriptType), ()> {
  if qr.len() > 127 {
    // seedqr must be less <= 127 digits to fit in a 29x29 QRCode
    return Err(());
  }

  let version = qr[0..1].parse::<u8>().unwrap();
  if version != 0 {
    return Err(());
  }

  let script = match qr[1..2].parse::<u8>().unwrap() {
    0 => ScriptType::NestedSegwit,
    1 => ScriptType::Segwit,
    // 2 => ScriptType::Taproot,
    // error
    _ => return Err(()),
  };
  let seed_word_count = match qr[2..3].parse::<u8>().unwrap() {
    0 => bip39::WordCount::Words12,
    1 => bip39::WordCount::Words15,
    2 => bip39::WordCount::Words18,
    3 => bip39::WordCount::Words21,
    4 => bip39::WordCount::Words24,
    // error
    _ => return Err(()),
  };

  let count = match seed_word_count {
    bip39::WordCount::Words12 => 12,
    bip39::WordCount::Words15 => 15,
    bip39::WordCount::Words18 => 18,
    bip39::WordCount::Words21 => 21,
    bip39::WordCount::Words24 => 24,
  };

  let qr = &qr[3..];

  // parse mnemonic phrase
  let end = count * 4;
  let indeces = &qr[0..end];
  let mut words: Vec<&str> = vec![];
  let english_words = bip39r::Language::English.word_list();
  for i in 0..count {
    let start = i * 4;
    let end = start + 4;
    let word_index = indeces[start..end].parse::<usize>().unwrap();
    words.push(english_words[word_index]);
  }

  // parse path as each char as 3-digit unicode
  let path_chars = &qr[end..];
  let count = path_chars.len() / 3;
  let mut path: Vec<char> = vec![];
  for i in 0..count {
    let start = i * 3;
    let end = start + 3;
    path.push(path_chars[start..end].parse::<u8>().unwrap() as char);
  }
  let path_str = &["m/", path.iter().collect::<String>().as_str()].join("");
  let path = bip32::DerivationPath::from_str(path_str).unwrap();
  return Ok((words, path, script));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let secp = Secp256k1::new();

  let network = Network::Testnet;
  println!("network: {:?}", network);
  let wallet_type = WalletType::SingleSig;
  println!("wallet_type: {:?}", wallet_type);
  let script_type = ScriptType::Segwit;
  println!("script_type: {:?}", script_type);
  let path = get_path(&network, &wallet_type, &script_type)?;
  println!("Derivation Path: {}", path);

  println!("");

  // testnet
  // let phrase = "shed rebuild nut suffer begin estate vehicle round city sudden fence spoon";
  // signet
  let phrase = "typical bicycle winter insane actor chat upper lazy brother club rib speed bid word caught differ fragile venture merge seed glimpse head exile success";
  let mnemonic = bip39::Mnemonic::parse_in(bip39::Language::English, phrase)?;
  let xkey: ExtendedKey = mnemonic.into_extended_key()?;
  let root = xkey.into_xprv(network).unwrap();
  println!("root xpriv: {}", root);

  let xpub = bip32::ExtendedPubKey::from_private(&secp, &root);
  println!("root xpub: {}", xpub);

  println!("");

  let fingerprint = xpub.fingerprint();
  println!("Master Fingerprint: {}", fingerprint);

  println!("wif: {}", root.private_key.to_wif());
  let wif_desc = DescriptorSecretKey::SinglePriv(DescriptorSinglePriv {
    key: root.private_key,
    origin: Some((fingerprint, path.clone())),
  });
  println!("wif descriptor: {}", wif_desc);

  println!("");

  let xpriv_at_path = root.derive_priv(&secp, &path)?;
  let xpub_at_path = bip32::ExtendedPubKey::from_private(&secp, &xpriv_at_path);
  println!("xpub:                             {}", xpub_at_path);
  // let slip132_xpub = convert_xpub_slip132(&xpub, &network, &wallet_type, &script_type);

  let specter_xpub = DescriptorPublicKey::XPub(DescriptorXKey {
    derivation_path: bip32::DerivationPath::master(),
    origin: Some((fingerprint, path.clone())),
    xkey: xpub_at_path,
    wildcard: Wildcard::None,
  });
  println!("specter xpub: {}", specter_xpub);

  let wallet = get_wallet(&root, &path, network)?;

  println!("");

  let serialized_psbt = "cHNidP8BAHECAAAAAVBfV44DutAVcwBhLjMDvKtwbQrmjkH+MFjFXPUqFS9/AQAAAAD9////AkZHmAAAAAAAFgAUZjVsZYVAouW7J4+EBUkSr57hq1cQJwAAAAAAABYAFBAjFi4nxS8Zk94GX+oLLSGwP5sObP4AAAABAHECAAAAAQRANrAsHCjxffzlyW5XgWuP4zvg5uyK69HEtKp36018AAAAAAD9////AhAnAAAAAAAAFgAUe5yb+brLgwDgNAd2FhiCD6ZEJ/vjbpgAAAAAABYAFLXGhIrgQeONcX9YAuONxfsqpR8rSv4AAAEBH+NumAAAAAAAFgAUtcaEiuBB441xf1gC443F+yqlHysiBgPF+tMr9FV6qyy6i32WuHnS+wB30GZgodSYsGaNPeFM7RjiaHvrVAAAgAEAAIAAAACAAQAAAAAAAAAAIgICyh7fjg8uUNT8Kzs5cHSGTKBa4QiUNOGTd7+zkoa0AqwY4mh761QAAIABAACAAAAAgAEAAAABAAAAAAA=";

  println!("Unsigned PSBT: {:?}\n", serialized_psbt);

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

  println!("change list: {:?}", change);
  println!("spend list: {:?}", spend);
  println!("");

  if wallet.sign(&mut psbt, SignOptions::default())? {
    println!(
      "Signed PSBT: {:?}",
      base64::encode(&consensus::serialize(&psbt))
    );
  } else {
    println!("Error: could not sign transaction");
  }

  let seed= "136400980811079503490561095703230934105802751813017212440282184807481683015201310078178605500063";
  // fits in a 29x29 qr code
  // https://twitter.com/KeithMukai/status/1420906150036484101
  let serialized_seedqr = ["014", seed, "056052104047049104047048104"].join("");
  println!("serialized:   {}", serialized_seedqr);
  println!(
    "unserialized: {:?}",
    parse_extended_seedqr(serialized_seedqr.as_str()).unwrap()
  );

  Ok(())
}
