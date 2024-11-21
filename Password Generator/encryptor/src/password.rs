
// encryptor/src/password.rs
use anyhow::{bail, Error, Result};
use base64::encode;
use hash::merhash::mersenne_hash;
/// crypto(length=100), you can exchange/add/delete characters
const CRYPTO: &str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABl
9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:RaKU@}cde56R7=8f/9gIhi,jkzmn";
/// Password generator with hash value, use hash value's high
/// power to get character from CRYPTO
///
/// #Example
/// use encryptor::password::generate_password;
/// let seed = "jdwnp";
/// let length = 16;
/// let passwd = generate_password(seed, length);
/// match passwd {
///Ok(val) => println!("{:#?}", val),
///Err(err) => println!("{:#?}", err),
/// }
pub fn generate_password(seed: &str, length: usize)
-> Result<String, Error>
{
// check password length
if length < 6 {
bail!("length must >= 6"); // return error
}
// calculate mer_hash
let p = match length {
6..=10 => 1,
11..=15 => 2,
16..=20 => 3,
_ => 3,
};
let mut mer_hash = mersenne_hash(seed).pow(p);
// calculate password by mer_hash
let mut passwd = String::new();
let crypto_len = CRYPTO.len();
while mer_hash > 9 {
let loc = mer_hash % crypto_len;
let nthc = CRYPTO.chars()
.nth(loc)
.expect("Error while getting char!");
passwd.push(nthc);
mer_hash /= crypto_len;
}

// combine seed and passwd
let interval = passwd.clone();
for c in seed.chars() {
passwd.push(c);
passwd += &interval;
}
// encode passwd to base64
passwd = encode(passwd);
passwd = passwd.replace("+", "*").replace("/", "*");
// length is not enough, use interval to fill
let interval = passwd.clone();
while passwd.len() < length {
passwd += &interval;
}
// return first length characters as password
Ok(format!("{}: {}", seed, &passwd[..length]))
}
