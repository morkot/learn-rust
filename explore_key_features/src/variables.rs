fn main() {
  // let x: i32 = 1; -> won't compile as x is immutable
  let mut x: i32 = 1;
  x += 2;

  assert_eq!(x, 3);
  println!("Succcess!");
}
