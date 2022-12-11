
pub fn run(_num: i32, _min: i32){
  let mut ghoul = _num;

  while ghoul >= 7 {
      ghoul -= _min;
      println!("{}-7",ghoul);
  }
  println!("### DEAD INSIDE 1000-7 ###")
}