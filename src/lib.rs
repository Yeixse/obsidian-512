use std::fs;
use std::io;
use std::fmt::Write;

pub struct Obsidian512;

impl Obsidian512 {
    pub fn hash_file(path: &str) -> io::Result<String> {
        let bytes = fs::read(path)?;
        Ok(Self::calculate_hash(&bytes))
    }

    pub fn hash_str(input: &str) -> io::Result<String> {
        Ok(Self::calculate_hash(input.as_bytes()))
    }

    pub fn hash_number<T: Into<u128>>(num: T) -> io::Result<String> {
        let val: u128 = num.into();
        Ok(Self::calculate_hash(&val.to_be_bytes()))
    }

    pub fn hash_bytes(bytes: &[u8]) -> io::Result<String> {
        Ok(Self::calculate_hash(bytes))
    }

    pub fn calculate_hash(bytes: &[u8]) -> String {
        const K: [u32; 8] = [
            0xB7E15162, 0x243F6A88, 0x9E3779B9, 0x61C88647,
            0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6,
        ];

        let mut padded_bytes = bytes.to_vec();
        let inp_len = bytes.len() as u64;
        padded_bytes.push(0x80);
        while (padded_bytes.len() + 8) % 64 != 0 {
            padded_bytes.push(0);
        }
        padded_bytes.extend_from_slice(&inp_len.to_be_bytes());

        let mut state_data: Vec<u32> = padded_bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        let mut h: [u32; 8] = [
            0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
            0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
        ];

        for _i in 0..128 {
            let i_u32 = _i as u32;
            for j in 0..state_data.len() {
                let mut s1 = state_data[j];
                s1 = s1.wrapping_add(h[j % 8]).rotate_left(7);
                s1 ^= (h[(j + 1) % 8] ^ i_u32).wrapping_mul(0x85ebca6b);            
                let ch = (h[0] & h[1]) ^ (!h[0] & h[2]);
                let maj = (h[3] & h[4]) ^ (h[3] & h[5]) ^ (h[4] & h[5]);          
                s1 = s1.wrapping_add(ch).wrapping_add(maj).wrapping_add(K[j % 8]);
                let rot_amt = (s1.count_ones() & 31) as u32;
                s1 = s1.rotate_right(rot_amt) ^ h[_i % 8];
                let temp = h[7].wrapping_add(s1);
                h[7] = h[6]; h[6] = h[5]; h[5] = h[4].wrapping_add(temp);
                h[4] = h[3]; h[3] = h[2]; h[2] = h[1].rotate_left(13);
                h[1] = h[0]; h[0] = temp.wrapping_add(maj).rotate_right(3);
                state_data[j] = s1;
            }
        }

        let mut final_state: [u32; 16] = [0; 16];
        for i in 0..8 {
            final_state[i] = h[i];
            final_state[i + 8] = h[i].wrapping_mul(0x1b873593).rotate_left(15) ^ 0xdeadbeef;
        }

        for (i, &val) in state_data.iter().enumerate() {
            let idx = i % 16;
            final_state[idx] = (final_state[idx] ^ val)
                .wrapping_add(0x9e3779b9)
                .rotate_left((val.count_ones() & 31) as u32);
        }

        let mut hex = String::with_capacity(128);
        for num in final_state {
            write!(hex, "{:08x}", num).unwrap();
        }
        hex
    }
}