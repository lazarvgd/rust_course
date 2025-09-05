fn main() {
    println!("Hello, world!");
    let a = 5;
    let b = 10;
    let result = week1::add(a, b); 
    println!("The sum of {} and {} is {}", a, b, result);
    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    let merged = week1::merge_strings(&s1, &s2);
    println!("Merged string: {}", merged);
}
