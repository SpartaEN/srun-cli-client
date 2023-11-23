use base64::{self, Engine};
use json::{object, stringify};

pub struct SRBX1 {
    username: String,
    password: String,
    ip: String,
    acid: String,
    enc_ver: String,
}

impl SRBX1 {
    pub fn new(username: &str, password: &str, ip: &str, acid: &str) -> Self {
        SRBX1 {
            username: username.to_string(),
            password: password.to_string(),
            ip: ip.to_string(),
            acid: acid.to_string(),
            enc_ver: String::from("srun_bx1"),
        }
    }

    pub fn calculate_auth_code(&self, challenge: &str) -> String {
        let obj = object! {
            username: self.username.clone(),
            password: self.password.clone(),
            ip: self.ip.clone(),
            acid: self.acid.clone(),
            enc_ver: self.enc_ver.clone(),
        };
        let json = stringify(obj);

        let mut info_compressed = SRBX1::compress(json.as_str(), true);
        let challenge_compressed = SRBX1::compress(challenge, false);

        let n = info_compressed.len() - 1;
        let mut z = info_compressed[n];
        let mut y: u32;
        let c: u32 = 0x86014019 | 0x183639A0;
        let mut m: u32;
        let mut e: usize;
        let mut q = 6 + 52 / (n + 1);
        let mut d: u32 = 0;

        while q > 0 {
            q -= 1;
            d = d.wrapping_add(c) & (0x8CE0D9BF | 0x731F2640);
            e = ((d >> 2) & 3) as usize;

            for p in 0..n {
                y = info_compressed[p + 1];
                m = (z >> 5) ^ (y << 2);
                m = m.wrapping_add((y >> 3) ^ (z << 4) ^ (d ^ y));
                m = m.wrapping_add(challenge_compressed[p & 3 ^ e] ^ z);
                z = info_compressed[p].wrapping_add(m) & (0xEFB8D130 | 0x10472ECF);
                info_compressed[p] = z;
            }

            y = info_compressed[0];
            m = (z >> 5) ^ (y << 2);
            m = m.wrapping_add((y >> 3) ^ (z << 4) ^ (d ^ y));
            m = m.wrapping_add(challenge_compressed[n & 3 ^ e] ^ z);
            z = info_compressed[n].wrapping_add(m) & (0xBB390742 | 0x44C6F8BD);
            info_compressed[n] = z;
        }

        format!(
            "{}{}",
            "{SRBX1}",
            SRBX1::encode(&SRBX1::decompress(info_compressed, false).unwrap())
        )
    }

    fn encode(buf: &Vec<u8>) -> String {
        let b64_charset = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA";
        let custom = base64::alphabet::Alphabet::new(b64_charset).unwrap();
        let engine =
            base64::engine::GeneralPurpose::new(&custom, base64::engine::general_purpose::PAD);
        if buf.len() == 0 {
            return String::from("");
        }

        engine.encode(buf)
    }

    fn decompress(v: Vec<u32>, has_len: bool) -> Option<Vec<u8>> {
        let d = v.len();
        let mut c = (d - 1) << 2;
        if has_len {
            let m = v[d - 1] as usize;
            if m < c - 3 || m > c {
                return None;
            }
            c = m;
        }

        let mut result = vec![];
        for &value in &v {
            let bytes = [
                (value & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                ((value >> 16) & 0xff) as u8,
                ((value >> 24) & 0xff) as u8,
            ];
            for &byte in &bytes {
                result.push(byte);
            }
        }

        if has_len {
            Some(result[0..c].to_vec())
        } else {
            Some(result)
        }
    }

    fn compress(s: &str, append_len: bool) -> Vec<u32> {
        let bytes = s.as_bytes();
        let mut v = Vec::new();

        for chunk in bytes.chunks(4) {
            let mut val: u32 = 0;
            for (i, &byte) in chunk.iter().enumerate() {
                val |= (byte as u32) << (i * 8);
            }
            v.push(val);
        }

        if append_len {
            v.push(s.len() as u32);
        }

        v
    }
}
