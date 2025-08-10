# RustTuring

A fast Rust implementation of Qualcomm's Turing stream cipher.

## Introduction

Turing (named after Alan Turing) is a stream cipher designed to simultaneously
be:

* Extremely fast in software on commodity PCs,
* Usable in very little RAM on embedded processors, and
* Able to exploit parallelism to enable fast hardware implementation.

For more refer to: [Turing: A Fast Stream Cipher](https://bing.com/search?q=Turing%3A+A+Fast+Stream+Cipher) .

## Usage

Install
Run the following Cargo command in your project directory:

```bash
cargo add turing-cipher
```

Or add the following line to your Cargo.toml:

```toml
[dependencies]
turing-cipher = "0.1"
```

use it in your application:

````rust
use turing_cipher::Turing;

pub fn main() {
    let key = b"test key 128bits";
    let iv = b"\0\0\0\0";

    // create a cipher instance.
    let mut cipher = Turing::new();

    // setup key and iv
    cipher.turing_key(key, key.len()).unwrap();
    cipher.turing_iv(iv, iv.len()).unwrap();

    let mut output = [0u8; 340];
    // generate stream 
    let n = cipher.turing_gen(&mut output).unwrap();

    assert_eq!(n, 340);

    println!("output: {:?}", output);
}
````

The key size must be a multiple of 4 bytes and must be between 8 and 32 bytes. The IV is optional and may be omitted by
specifying a 0 bytes value. If the IV is present, the size must be a multiple of 4 bytes. The combined size of the key
and IV must not exceed 48 bytes. These restrictions are part of the algorithm specs.

