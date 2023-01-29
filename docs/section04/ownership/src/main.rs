fn main() {
  // let test = (3, "4");

  // let mut s = String::from("hello");

  // let r1 = &s; // no problem
  // let r2 = &s; // no problem
  // println!("{} and {}", r1, r2);
  // // variables r1 and r2 will not be used after this point

  // let r3 = &mut s; // no problem
  // println!("{}", r3);

  let str = String::from("Hello hi");

  let result = first_word(&str);

  println!("{result}");

  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];

}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}