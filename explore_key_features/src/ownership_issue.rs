fn printer(s: String) {
  println!("{}", s);
}

// won't compile
fn main() {
    let s = String::from("Hello, world!");
    printer(s);
    printer(s);
}
