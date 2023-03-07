
// return a random number within the given range
pub(crate) fn rand_gen<T, R>(range: R) -> T
where
T: rand::distributions::uniform::SampleUniform,
R: rand::distributions::uniform::SampleRange<T> {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}

// get true or false, based on the chance
pub(crate) fn try_chance(chance: f32) -> bool {
  if chance <= 0.0 {
    false // 0%
  } else if chance >= 1.0 {
    true // 100%
  } else {
    let rand = rand_gen(0.0..1.0);
    rand < chance
  }
}
