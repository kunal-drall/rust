fn main() {
    let name = String::from("Kunal");
    let length = get_str_len(name);
    println!("The length of the string is: {}", length);    // Uncommenting the next line will cause a compile-time error
    // println!("The name is: {}", name); // This line will not compile because `name` has been moved
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}
