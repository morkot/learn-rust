fn main() {
  let x: i32 = 3;

  {
    let y: i32 = 1;
    println!("X is {} and Y is {}", x, y);
  }

  //println!("X is {} and Y is {}", x, y); -> won't compile
}
