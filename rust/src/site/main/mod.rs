
use super::Page;

pub fn main_page() -> Page {

  let id = String::from("main");
  let url = String::from(""); // none, root
  let title = String::from("GUI - Main");
  let content = format!("
<!DOCTYPE html>
<html>
<head>
  <title>{title}</title>
</head>
<body>
  <p>Hello, World!</p>
</body>
</html>
");

  Page {
    id,
    url,
    title,
    content,
  }

}
