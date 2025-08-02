mod turing;
mod turing_tables;

pub use turing::Turing;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::Turing;

    // 测试向量
    const TEST_KEY: &[u8] = b"test key 128bits";
    const TEST_FRAME: &[u8] = &[0, 0, 0, 0];
    const TEST_OUT1: &str = "69 66 26 bb dc 6e 09 f6 da 9a ba b5 b5 6c 14 87 82 46 df 18";
    const STREAM_OUT: &str = "5d a8 8c ed 8a a6 55 ba 78 08 ef f8 cf 32 63 c0 75 e4 40 3c";

    // 十六进制工具函数
    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        hex.split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect()
    }

    fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_turing_basic() {
        let mut cipher = Turing::new();
        let mut output = [0u8; 340];

        cipher.turing_key(TEST_KEY, TEST_KEY.len()).unwrap();
        cipher.turing_iv(TEST_FRAME, TEST_FRAME.len()).unwrap();
        let n = cipher.turing_gen(&mut output).unwrap();

        assert_eq!(n, 340);

        let first_20 = &output[..20];
        let hex_output = bytes_to_hex(first_20);
        assert_eq!(hex_output, TEST_OUT1);
        println!("hex_output: {}", hex_output);
    }
}
