// [[file:../gosh-core.note::a722eec4][a722eec4]]
use gut::prelude::*;

pub use rand::prelude::*;

/// Simulation with a fixed seed to get reproducible results.
pub fn rng_with_seed(seed: impl Into<Option<u64>>) -> StdRng {
    let seed = seed.into().unwrap_or_else(|| {
        let value = gut::utils::unix_timestamp();
        info!("gosh: the random process can be repeated with seed {value}");
        value
    });
    StdRng::seed_from_u64(seed)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_rng() {
        use crate::random::*;

        let mut rng = rng_with_seed(None);
        rng.gen::<i32>();
        assert_eq!(rng.gen_range(0..1), 0);
    }
}
// a722eec4 ends here
