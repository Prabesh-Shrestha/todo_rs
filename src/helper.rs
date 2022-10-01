use std::io;
use std::io::Write;
pub fn prompt(message: &str) -> String {
    let mut buf = String::new();
    print!("[{}]=>", message);
    io::stdout().flush().expect("error while flushing stdout");
    io::stdin()
        .read_line(&mut buf)
        .expect("Error while prompting user");
    buf.trim().to_string()
}
