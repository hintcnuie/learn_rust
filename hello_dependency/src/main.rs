use regex::Regex;
fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("日期是否匹配？{}",re.is_match("2024-01-07"));
}
