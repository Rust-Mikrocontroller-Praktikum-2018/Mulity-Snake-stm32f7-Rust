extern crate stm32f7_discovery as stm32f7;

// from stm32f7::random.rs:
// Since for disabling the rng, some rcc clock on the AHB2 Bus must be disabled as well.
// Therefore use .disable(rcc) after you are done.
// ```
// random_gen.disable(rcc);

pub struct Random {
    rng: stm32f7::random::Rng,
}

impl Random {
    pub fn new(
        rng: &'static mut stm32f7::board::rng::Rng,
        rcc: &mut stm32f7::board::rcc::Rcc,
    ) -> Random {
        let mut random_gen = stm32f7::random::Rng::init(rng, rcc).unwrap();
        Random { rng: random_gen }
    }

    /**
     * Random u32 number.
     */
    pub fn random_u32(&mut self) -> Result<u32, stm32f7::random::ErrorType> {
        self.rng.poll_and_get()
    }

    /**
     * Random u32 number in from (including) to (excluding).
     * https://crypto.stackexchange.com/questions/7996/correct-way-to-map-random-number-to-defined-range
     */
    pub fn random_range(&mut self, from: u32, to: u32) -> u32 {
        assert!(from < to);

        let m = to - from; // > 0, see assert in first line
        // if from=0 and to=u32::max_value() return random u32
        if m == u32::max_value() {
            hprintln!("max value case");
            return self.random_u32().unwrap();
        }
        // 2.pow(32) > u32::max_value() => Error
        if m > 2_u32.pow(31) {
            hprintln!("> 2^31 case");
            let random_u32 = self.rng.poll_and_get().unwrap();
            if random_u32 < to {
                return random_u32;
            } else {
                return random_u32 - m;
            }
        }
        if m == 2_u32.pow(31) {
            let random_u32 = self.rng.poll_and_get().unwrap();
            return random_u32>>1;
        }

        let random_u32 = self.rng.poll_and_get().unwrap();
        // find smallest possible 2^u < (to-from)
        let mut u = 1;
        while 2_u32.pow(u) < m {
            u += 1;
        }
        let t = m * (2_u32.pow(u) / m);
        if random_u32 >= t {
            // ToDo: If x≥t then loop to the previous step.
            // Return r = x mod m.
        }

        (random_u32<<(32-u))>>(32-u)
    }
}
