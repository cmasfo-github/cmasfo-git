
pub use crate::*;

mod bg;
pub use bg::*;

struct Silly {
  last: Option<String>,
}

pub(super) fn mode_silly() {
  
  println!("Hi! I'm silly. How can I help you?");
  
  let mut silly = Silly { last: None };
  
  loop {
    
    let mut prompt = msg_line!("Prompt: ");
    
    if prompt.is_empty() {
      if let Some(last) = &silly.last {
        prompt = last.clone();
      } else {
        continue;
      }
    } else {
      silly.last = Some(prompt.clone());
    }
    
    let parsed = parse_prompt(&prompt);
    
    if process_prompt(&parsed) {
      continue;
    } else {
      break;
    }
    
  }
  
}

fn parse_prompt(prompt: &str) -> Vec<String> {
  let split = prompt.split_whitespace();
  split.map(|s| s.to_lowercase()).collect()
}

fn process_prompt(parsed: &Vec<String>) -> bool {
  
  let first = &parsed[0];
  
  match first.as_str() {
    "exit" | "quit" | "bye" | "goodbye" => {
      return false;
    }
    "hi" | "hello" => {
      println!("Hello, there! How can I help you?");
    }
    "silly" => {
      println!("Yes, I'm silly. How can I help you?");
    }
    "siri" => {
      println!("No, I'm not siri. My name is silly.");
    }
    "fun" | "fact" | "funfact" => {
      fun_fact();
    }
    _ => {
      let v = vec!(
        "sorry... what?",
        "i don't understand...",
      );
      random_print(&v);
    }
  };
  
  true
  
}

fn fun_fact() {
  let fun_facts = vec!(
    "You know what? Fac-Deez Nuts!",
    "Did you know that Big Freeze will eventually finish the universe?",
    "Did you know that Big Bang happened 13.8 billion years ago?",
    "The shortest war in history was between Britain and Zanzibar in 1896. It lasted just 38 minutes.",
    "The world's largest snowflake on record measured 15 inches wide and 8 inches thick. It fell in Fort Keogh, Montana in 1887.",
    "The longest wedding veil was longer than 63 football fields. It was worn by a woman in Cyprus in 2018.",
    "The world's oldest piece of chewing gum is over 9,000 years old.",
    "The shortest full-length novel in the world is called 'The Dinosaur' and consists of only seven words: 'The dinosaur was there. So was I.'",
    "There is a coffee shop in Japan where you can play with hedgehogs while you drink your coffee.",
    "Cows have best friends and can get stressed when they are separated.",
    "In the 16th century, some Europeans believed that tomatoes were poisonous because they belong to the nightshade family.",
    "The first oranges weren't actually orange - they were green.",
    "The longest word in the English language has 189,819 letters and takes over three hours to pronounce.",
  );
  random_print(&fun_facts);
}
