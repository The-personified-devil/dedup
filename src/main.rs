#![feature(iter_collect_into)]

use memmap::MmapOptions;
use std::{fs, collections::HashSet, hash::{Hasher, BuildHasher}, ptr::addr_of};

pub struct H (u64);
struct HBuild;

impl BuildHasher for HBuild {
    type Hasher = H;
    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        H(0x55555555)

    }
}

impl Hasher for H {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        let thingys = bytes.len() / 8;
        let mut it = bytes.chunks_exact(4);
        let rem = it.remainder();
        for b in it {
            let val = unsafe { addr_of!(b[0]).cast::<u64>().read() };
            self.0 ^= val;
            self.0 = self.0.rotate_left(5);
        }

        for b in rem {
            self.0 ^= *b as u64;
            self.0 = self.0.rotate_left(5);
        }
        
    }
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }
}

fn main() {
    unsafe {
        // let bv = fs::read("../invalid_tokens.txt").unwrap_unchecked();
        let f = std::fs::File::open("../invalid_tokens.txt").unwrap_unchecked();
        let bv = unsafe { MmapOptions::new().map(&f).unwrap_unchecked() };
        let mut set: HashSet<&[u8], _> = HashSet::with_hasher(HBuild);

        // let cnt = bv[..bv.len()-1].split(|b| *b == b'\n').count();
        // set.reserve(cnt * 2);
        set.reserve(bv.len() / 20);

        let hs = bv[..bv.len()-1].split(|b| *b == b'\n').collect_into(&mut set);
        println!("uniq: {}", hs.len());
    }
}
