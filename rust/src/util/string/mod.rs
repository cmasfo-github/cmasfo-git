
// "to upper"
// -> "TO UPPER"
pub fn to_upper(str: &str) -> String {
  str.to_uppercase()
}

// "TO LOWER"
// -> "to lower"
pub fn to_lower(str: &str) -> String {
  str.to_lowercase()
}

// "word head upper"
// -> "Word Head Upper"
pub fn word_head_upper(str: &str) -> String {
  word_head_conversion(str, Conversion::Upper)
}

// "WORD HEAD LOWER"
// -> "wORD hEAD lOWER"
pub fn word_head_lower(str: &str) -> String {
  word_head_conversion(str, Conversion::Lower)
}

enum Conversion {
  Upper,
  Lower,
}

fn word_head_conversion(str: &str, conversion: Conversion) -> String {
  let split = str.split_whitespace();
  let mut vec = Vec::new();

  for word in split {
    if word.len() > 0 {
      if word.len() == 1 {
        let word = word.to_uppercase();
        vec.push(word);
      } else {
        let first = &word[0..=0];
        let rest = &word[1..];
        let first = match conversion {
          Conversion::Upper => to_upper(first),
          Conversion::Lower => to_lower(first),
        };
        let result = first + rest;
        vec.push(result);
      }
    }
  }

  vec.join(" ")
}

// "first head upper"
// -> "First head upper"
pub fn first_head_upper(str: &str) -> String {
  first_head_conversion(str, Conversion::Upper)
}

// "First Head Lower"
// -> "first Head Lower"
pub fn first_head_lower(str: &str) -> String {
  first_head_conversion(str, Conversion::Lower)
}

fn first_head_conversion(str: &str, conversion: Conversion) -> String {
  let len = str.len();
  if len > 0 {
    if len == 1 {
      str.to_uppercase()
    } else {
      let first = &str[0..=0];
      let rest = &str[1..];
      let first = match conversion {
        Conversion::Upper => to_upper(first),
        Conversion::Lower => to_lower(first),
      };
      first + rest
    }
  } else {
    String::new()
  }
}
