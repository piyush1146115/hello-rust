fn main(){
      print!("{} + {} = {}\n", 34, 80, 80 + 34);
      print!("{}\n", (23 - 6) % 5 + 20 * 30 / (3 + 4));

      //   By putting a dot after a literal number, it is transformed into a literal floating-point
      //   number. Some programming languages require a digit after the dot, but not Rust
      print!("{}\n", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
      print!("{} {}\n", -12 % 10, -1.2 % 1.);
      // This will print -2 -0.19999999999999996.

      // It will print 87.71428571428571.
      print!("{}\n", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));

      // This is generate a compilation error. in Rust you cannot simply mix integer
      // numbers and floating-point numbers.
      // print!("{}", 2.7 + 1);

      print!("{} + ", 80);
      print!("{} =", 34);
      print!(" {}\n", 80 + 34);


      //Breaking Literal Strings
      println!("{}\n", "These
            are 
            three lines");

      println!("{}\n", "This \
            is \
            just one line");

      println!("{}", "These\n\
            are\n\
            three lines");

}
