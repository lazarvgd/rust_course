pub fn add(a :i32, b:i32) -> i32 {
    a + b
}

pub fn merge_strings(s1:&str, s2:&str) -> String {
    let new_string = format!("{} {}", s1, s2);
    new_string
}