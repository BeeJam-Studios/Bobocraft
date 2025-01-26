#[derive(Debug, Clone, Copy)]
pub struct Rng {
    seed: u32,
}

impl Rng {
    pub const fn with_seed(seed: u32) -> Rng {
        Rng {
            seed: if seed != 0 { seed } else { 1 },
        }
    }

    pub fn next(&mut self) -> u32 {
        let lcg_m: u32 = 2147483647; //meresenne prime for LCG with c=0
        let lcg_a: u32 = 48271;
        let lcg_q = lcg_m / lcg_a;
        let lcg_r = lcg_m % lcg_a;
        let lcg_g = lcg_a * (self.seed % lcg_q);
        let lcg_h = lcg_r * (self.seed / lcg_q);
        self.seed = if lcg_g > lcg_h {
            lcg_g - lcg_h
        } else {
            lcg_m - (lcg_h - lcg_g)
        };
        self.seed
    }
}
