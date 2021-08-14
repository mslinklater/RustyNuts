fn main() {

  let a = 10;
  if a == 10
  {
    println!("a equals 10!");
  }

  let mut letters = vec![            // <1>
      "a", "b", "c"
  ];

  for letter in letters {
      println!("{}", letter);
      letters.push(letter.clone());  // <2>
  }
}
