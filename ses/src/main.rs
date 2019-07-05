fn main() {
  let mut slice = [1, 2];

  println!("Before mutation:");
  for elem in &slice {
    println!("  {}", *elem);
  }

  let x = slice.get_mut(0).unwrap();
  *x = 3;

  println!("After mutation:");
  for elem in &slice {
    println!("  {}", *elem);
  }
}
