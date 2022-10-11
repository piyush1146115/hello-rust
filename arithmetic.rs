fn main(){
  print!("{} + {} = {}\n", 34, 80, 80 + 34);
  print!("{}\n", (23 - 6) % 5 + 20 * 30 / (3 + 4));

//   By putting a dot after a literal number, it is transformed into a literal floating-point
//   number. Some programming languages require a digit after the dot, but not Rust
  print!("{}\n", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
  print!("{} {}\n", -12 % 10, -1.2 % 1.);
// This will print -2 -0.19999999999999996.
}
