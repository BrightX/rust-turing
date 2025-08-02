use turing_cipher::Turing;

pub fn main() {
    let key = b"test key 128bits";
    let iv = b"";
    let mut cipher = Turing::new();
    let mut output = [0u8; 340];

    cipher.turing_key(key, key.len()).unwrap();
    cipher.turing_iv(iv, iv.len()).unwrap();
    let n = cipher.turing_gen(&mut output).unwrap();

    assert_eq!(n, 340);

    println!("output: {:?}", output);
}
