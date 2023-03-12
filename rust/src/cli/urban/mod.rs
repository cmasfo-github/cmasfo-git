use scraper::{Html, Selector};
use std::error::Error;
use tokio::runtime::Builder;

pub fn mode_urban() -> Result<(), Box<dyn Error>> {
  println!("To quit, press enter with empty line.");
  let rt = Builder::new_current_thread()
  .enable_all()
  .build()?;
  loop {
    print!("Search: ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let search_word = input.trim();
    
    if search_word.is_empty() {
      break Ok(());
    }
    
    let url = format!("https://www.urbandictionary.com/define.php?term={}", search_word);
    
    let response = rt.block_on(async {
      reqwest::get(&url).await?.text().await
    })?;
    
    let document = Html::parse_document(&response);
    
    let definition_selector
    = Selector::parse(".meaning").unwrap();
    let first_definition 
    = match document.select(&definition_selector).next() {
      Some(first_definition) => first_definition,
      None => {
        println!("No definitions found.");
        continue;
      }
    };
    
    let definition_text = first_definition.text().collect::<String>();
    
    println!("Urban: {}", definition_text);
  }
}
