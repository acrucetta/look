# Feature Roadmap

As a user, I want to:

- [ ] Open the path of the files from the CLI
- [ ] Have an easy to understand score
- [ ] Filter the search by tags or dates
- [ ] Have some NLP in the search to find deeper patterns in the data
- [ ] See a brief highlight of the text within the top results

File path example

```rust
use ansi_term::Colour::{Blue, Green};
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("/path/to/file.txt");
    let shortened_path = shorten_path(path);
    println!("Open file: {}{}{}", Blue.bold().paint("file:///"), Green.bold().paint(shortened_path.display()), Blue.bold().paint(":1"));
}

fn shorten_path(path: &Path) -> PathBuf {
    let mut shortened_path = PathBuf::new();
    if let Ok(path_after_prefix) = path.strip_prefix("/") {
        shortened_path.push("...");
        shortened_path.push(path_after_prefix);
    } else {
        shortened_path.push(path);
    }
    shortened_path
}

Open file: file:///.../to/file.txt:1
```
