pub fn read_char() -> char {
    use std::io::{ stdout, stdin, Write };

    let mut ret_str = String::new();

    stdout().flush()
        .expect("*");

    stdin().read_line(&mut ret_str)
        .expect("*");

    match ret_str.trim().chars().next() {
        Some(c) => return c,
        None          => return '*'
    }
}
