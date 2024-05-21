fn printer(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, world!");
    printer(&s);
    printer(&s);
}
