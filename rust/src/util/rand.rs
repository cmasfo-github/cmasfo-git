
use rand::Rng;
use rand::distributions::uniform::{
  SampleUniform, SampleRange
};

pub fn rand_gen<T, R>(range: R) -> T where
T: SampleUniform,
R: SampleRange<T> {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}
