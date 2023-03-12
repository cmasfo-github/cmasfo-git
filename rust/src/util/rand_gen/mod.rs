
// return a random number within the given range
pub fn rand_gen<T, R>(range: R) -> T
where
T: rand::distributions::uniform::SampleUniform,
R: rand::distributions::uniform::SampleRange<T> {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}

// get true or false, based on the chance
pub fn try_chance(chance: f32) -> bool {
  if chance <= 0.0 {
    false // 0%
  } else if chance >= 1.0 {
    true // 100%
  } else {
    let rand = rand_gen(0.0..1.0);
    rand < chance
  }
}

pub fn random_print(prints: &Vec<&str>) {

  if prints.len() == 0 { return; }

  let idx = rand_gen(0..prints.len());

  println!("{}", prints.get(idx).unwrap());

}
