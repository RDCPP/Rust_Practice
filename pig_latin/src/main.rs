fn main() {
    let mut s = String::from("first");
    if s.starts_with("a") || s.starts_with("e") || s.starts_with("i") || s.starts_with("o") || s.starts_with("u"){
        s.push_str("-hay");
    }
    else{
        let first_char = s.remove(0);
        let mut plus = String::from("-");
        plus.push(first_char);
        plus.push_str("ay");
        s.push_str(&plus);
    }
    println!("{}",s);
}