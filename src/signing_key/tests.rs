use zeroize::Zeroize;

use super::SigningKey;
use crate::SpendAuth;

#[test]
fn zeroize_erases_secret_scalar() {
    let mut bytes = [0u8; 32];
    bytes[0] = 7;
    let mut key = SigningKey::<SpendAuth>::from_bytes(&bytes).unwrap();
    assert_eq!(key.to_bytes(), bytes);

    key.zeroize();
    assert_eq!(key.to_bytes(), [0; 32]);
}
