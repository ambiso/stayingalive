use rand::Rng;
use rand_chacha::ChaCha12Rng;

struct MyRng {
    s: [u8; 32],
    wrote_seed: usize,
    r: ChaCha12Rng,
}

impl MyRng {
    #[no_mangle]
    pub extern "C" fn rng_new() -> *mut MyRng {
        let mut s = [0u8; 32];
        rand::thread_rng().fill(&mut s);
        let r: ChaCha12Rng = rand::SeedableRng::from_seed(s);
        Box::into_raw(Box::new(Self {
            s,
            wrote_seed: 0,
            r,
        }))
    }

    #[no_mangle]
    pub extern "C" fn rng_gen(&mut self) -> u32 {
        let mut data = [0u8; 4];
        if self.wrote_seed < self.s.len() / data.len() {
            let l = data.len();
            data.copy_from_slice(&self.s[self.wrote_seed..self.wrote_seed+l]);
            self.wrote_seed += data.len();
        } else {
            self.r.fill(&mut data);
        }
        ((data[0] as u32) << 24)
            | ((data[1] as u32) << 16)
            | ((data[2] as u32) << 8)
            | data[3] as u32
    }

    #[no_mangle]
    pub extern "C" fn rng_free(me: *mut Self) {
        unsafe { Box::from_raw(me) };
    }
}
