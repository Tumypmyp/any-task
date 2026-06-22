use keyring_core::Entry;
use zeroize::Zeroize;

const SERVICE: &str = "com.myapp.wallet";
const USER: &str = "mnemonic"; // or a wallet account ID

pub fn save_mnemonic(phrase: &str) -> Result<(), keyring_core::Error> {
    let entry = Entry::new(SERVICE, USER)?;
    entry.set_secret(phrase.as_bytes())?; // store as raw bytes
    Ok(())
}

pub fn load_mnemonic() -> Result<String, keyring_core::Error> {
    let entry = Entry::new(SERVICE, USER)?;
    let mut bytes = entry.get_secret()?;
    let phrase = String::from_utf8(bytes.clone())
        .map_err(|e| keyring_core::Error::Invalid("mnemonic".into(), e.to_string()))?;
    bytes.zeroize(); // wipe the raw bytes from memory
    Ok(phrase)
}

pub fn delete_mnemonic() -> Result<(), keyring_core::Error> {
    let entry = Entry::new(SERVICE, USER)?;
    entry.delete_credential()
}
