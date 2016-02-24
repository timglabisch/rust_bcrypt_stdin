
use rand::{Rng, random};
use rand::os::OsRng;
use std::char;
use std::str::from_utf8;

const BCRYPT_HASH64: &'static [u8] = b"./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn gen_salt_str(chars: usize) -> String {
	let mut rng = OsRng::new().unwrap();
	let bytes = ((chars + 3) / 4) * 3;
	let rv = rng.gen_iter::<u8>().take(bytes).collect::<Vec<_>>();
	let mut sstr = bcrypt_hash64_encode(&rv);
	while sstr.len() > chars {
	    sstr.pop();
	}
	sstr
}

pub fn bcrypt_hash64_encode(bs: &[u8]) -> String {
    b_c_hash64_encode(bs, &BCRYPT_HASH64)
}

fn b_c_hash64_encode(bs: &[u8], hs: &[u8]) -> String {
    let ngroups = (bs.len() + 2) / 3;
    let mut out = String::with_capacity(ngroups * 4);
    for g in 0..ngroups {
    	let mut g_idx = g * 3;
    	let mut enc = 0u32;
    	for _ in 0..3 {
    	    let b = (if g_idx < bs.len() { bs[g_idx] } else { 0 }) as u32;
    	    enc <<= 8;
    	    enc |= b;
    	    g_idx += 1;
    	}
    	for _ in 0..4 {
    	    out.push(char::from_u32(hs[((enc >> 18) & 0x3F) as usize] as u32).unwrap());
    	    enc <<= 6;
    	}
    }
    match bs.len() % 3 {
    	1 => { out.pop(); out.pop(); },
    	2 => { out.pop(); },
    	_ => (),
    }
    out
}
