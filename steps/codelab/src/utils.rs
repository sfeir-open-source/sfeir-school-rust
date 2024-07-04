use rand::Rng;

pub fn generate_ships(numb_ships: i32) -> Result<Vec<String>, String>{
  if numb_ships > 10 {
    Err("to many ships".to_string())
  } else  if numb_ships == 0 {
    Err("need at least one ship".to_string())
  } else {
    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..numb_ships {
      let number = rng.gen_range(0..10);
      let letter = match rng.gen_range(0..10) {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        7 => "H",
        8 => "I",
        _ => "J",
      }.to_string();
      vec.push(letter + &number.to_string());
    }
    Ok(vec)
  }
}
