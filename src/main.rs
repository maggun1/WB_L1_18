fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();
    let chars: Vec<char> = input.chars().collect();
    for i in (0..chars.len()).rev() {
        reversed.push(chars[i]);
    }
    reversed
}

fn main() {
    let original = "главрыба";
    let reversed = reverse_string(&original);
    println!("{} - {}", original, reversed);
}
