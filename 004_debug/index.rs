// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main () {
  // Printing with `{:?}` is similar to with `{}`.
  println!("{:?} months in a year.", 12);

  let one = "Slater";
  let two = "Christian";
  let actor = "actor's";
  println!("{1:?} {0:?} is the {actor:?} name.", one, two, actor=actor);

  // `Structure` is printable!
  println!("Now {:?} will print!", Structure(3));

  // The problem with `derive` is there is no control over how
  // the results look. What if I want this to just show a `7`?
  println!("Now {:?} will print!", Deep(Structure(7)));
}
