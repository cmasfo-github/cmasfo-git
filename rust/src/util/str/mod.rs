
pub fn to_lower(s: &str) -> String {
  s.to_lowercase()
}

pub fn to_upper(s: &str) -> String {
  s.to_uppercase()
}

pub fn concat(s1: &str, s2: &str) -> String {
  s1.to_string() + s2
}

pub fn strcat(s1: &str, s2: &str) -> String {
  s1.to_string() + s2
}

pub fn append(s1: &str, s2: &str) -> String {
  s1.to_string() + s2
}

pub fn prepend(s1: &str, s2: &str) -> String {
  s2.to_string() + s1
}

// trim all s2 from s1
pub fn trim_all(s1: &str, s2: &str) -> String {
  s1.replace(s2, "")
}

// must not start or end with s2
pub fn trim_middle(s1: &str, s2: &str) -> String {
  let len = s1.len();
  if len > 2 { // at least 3
    let slice = &s1[1..len - 1]; // remove the first and last
    slice.replace(s2, "")
  } else {
    s1.to_string()
  }
}

// must start with s2
pub fn trim_left(s1: &str, s2: &str) -> String {
  s1.trim_start_matches(s2).to_string()
}

// must end with s2
pub fn trim_right(s1: &str, s2: &str) -> String {
  s1.trim_end_matches(s2).to_string()
}
