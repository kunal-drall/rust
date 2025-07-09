struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Kunal"),
        last_name: String::from("Drall"),
        age: 21,
    };

    println!("{}", user.first_name);
}
