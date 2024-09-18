fn main() {
    let input = "snow dog sun";
    let reverted = reverse_word(input);
    println!("{}", reverted) // sun dog snow
}

// Алгоритм:
// разбиваем строку и складывем в вектор слова
// ревертим вектор
// результат джоиним в строку
fn reverse_word(s: &str) -> String {
    let reverted: Vec<&str> = s.split_whitespace()
        .into_iter()
        .rev()
        .collect();

    reverted.join(" ")
}