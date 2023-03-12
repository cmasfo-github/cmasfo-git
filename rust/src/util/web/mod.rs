
pub fn open(url: &str) {

  if open_word(url) { return; }

  match url.parse::<u16>() {
    Ok(_) => {
      let url = "http://127.0.0.1:".to_string() + url;
      open::that(url);
    }
    Err(_) => {
      open::that(url);
    }
  }

}

pub fn open_word(url: &str) -> bool {

  match url {
    // A
    // B
    "bing" => {
      open::that("bing.com");
    }
    // C
    "chat" | "chatgpt" => {
      open::that("chat.openai.com");
    }
    // D
    "dc" | "dcinside" => {
      open::that("dcinside.com");
    }
    // dcinside
    "하드웨어" | "hw" | "hwgall" | "hardware" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=hardware");
    }
    "네트워크" | "net" | "netgall" | "network" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=net");
    }
    "운영체제" | "os" | "osgall" | "sw" | "swgall"
    | "software" | "opersys" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=software");
    }
    "프로그램" | "program" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=program");
    }
    "그래픽스" | "graphics" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=graphics");
    }
    "웹사이트" | "web" | "webgall" | "website" => {
      open::that("https://gall.dcinside.com/mini/board/lists/?id=website");
    }
    // E
    // F
    // G
    "github" => {
      open::that("github.com");
    }
    "google" => {
      open::that("google.com");
    }
    // H
    // I
    // J
    // K
    // L
    // M
    // N
    // O
    // P
    "profile" => {
      open::that("github.com/cmasfo-github");
    }
    // Q
    // R
    "reddit" => {
      open::that("reddit.com");
    }
    "repo" | "repository" | "git" => {
      open::that("github.com/cmasfo-github/cmasfo-git");
    }
    // S
    // T
    // U
    // V
    // W
    // X
    // Y
    // Z
    _ => {
      match url.find(".") {
        Some(_) => {
          return false;
        }
        None => {
          let url = format!("{}.com", url);
          open::that(url);
        }
      }
    }
  }

  true

}
