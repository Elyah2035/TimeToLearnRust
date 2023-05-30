use std::io;

fn text_to_hex(input: &str) -> String {
    let hex_chars: Vec<String> = input
        .chars()
        .map(|c| format!("{:02X}", c as u32))
        .collect();

    hex_chars.join("")
}

fn main() {
    loop {
        println!("Enter text to convert to hexadecimal:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let text = input.trim();
        let hex = text_to_hex(text);

        println!("Text: {}", text);
        println!("Hex: {}", hex);
    }
}
