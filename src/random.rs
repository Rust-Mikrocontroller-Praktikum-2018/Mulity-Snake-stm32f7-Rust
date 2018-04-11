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
            return random_u32 >> 1;
        }

        // find smallest possible 2^u < (to-from)
        // Todo: Maybe better go backwards and start with ::max_value() and u=31
        let mut u = 1;
        while 2_u32.pow(u) < m {
            u += 1;
        }

        let t = m * (2_u32.pow(u) / m);
        
        let mut random_u32: u32 = u32::max_value();
        while random_u32 >= t {
            // shift left to drop first (32-u) bits then shift back
            random_u32 = (self.rng.poll_and_get().unwrap())>>(32-u);
        }
        random_u32 % m

        // let random_u32 = self.rng.poll_and_get().unwrap();
        // (random_u32 << (32 - u)) >> (32 - u)
    }

    /**
     * Test this Random functions. pls enable semihosting.
     */
    pub fn test_me(mut random_gen: Random) {
        let mut counter = 0;
        let mut from = 0;
        let mut to = u32::max_value();
        let mut r = random_gen.random_range(from, to);
        hprintln!("{} <= {} < {} | counter={}", from, r, to, counter);
        assert!(from <= r);
        assert!(r < to);
        hprintln!("----------------------------");

        counter += 1;
        from = 1;
        to = u32::max_value();
        r = random_gen.random_range(from, to);
        hprintln!("{} <= {} < {} | counter={}", from, r, to, counter);
        assert!(from <= r);
        assert!(r < to);
        hprintln!("----------------------------");

        counter += 1;
        from = 10;
        to = u32::max_value() - 1;
        r = random_gen.random_range(from, to);
        hprintln!("{} <= {} < {} | counter={}", from, r, to, counter);
        assert!(from <= r);
        assert!(r < to);
        hprintln!("----------------------------");

        counter += 1;
        from = 1;
        to = 3;
        r = random_gen.random_range(from, to);
        hprintln!("{} <= {} < {} | counter={}", from, r, to, counter);
        assert!(from <= r);
        assert!(r < to);
        hprintln!("----------------------------");

        from = 1;
        to = 3;
        loop {
            while from < to {
                counter += 1;
                r = random_gen.random_range(from, to);
                hprintln!("{} <= {} < {} | counter={}", from, r, to, counter);
                assert!(from <= r);
                assert!(r < to);
                hprintln!("----------------------------");

                // from = random_gen.rng.poll_and_get().unwrap();
                // to = random_gen.rng.poll_and_get().unwrap();
            }
            from = random_gen.rng.poll_and_get().unwrap();
            to = random_gen.rng.poll_and_get().unwrap();
        }
    }
}
