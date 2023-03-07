
#[allow(unused_macros)]
macro_rules! warn {
  ($($tokens: tt)*) => {
    println!("cargo:warning={}", format!($($tokens)*))
  }
}

fn main() {
}
