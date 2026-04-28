
use std::io ; 
use rand::Rng ; 
use std::cmp::Ordering ; 
use serde::{Serialize , Deserialize} ; 
use std::fs ; 


#[derive(Serialize , Deserialize , Debug)]



struct GameStats { 
  games_played : u32 , 
  games_own : u32 , 
  current_guesses : Vec<u32> , 
}

impl GameStats { 
   fn new ()  -> Self { 
       GameStats {
        games_played : 0 , 
        games_own : 0 , 
        current_guesses : Vec::new(), 
       }
   }
}





fn main() {
    println!(" guess the number"); 
    let mut stats = load_stats(); 

   let  random = rand::thread_rng().gen_range(1..=100); 
    loop { 
     
    let mut guess  =  String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to readline"); 

    let  guess : u32 = match guess.trim().parse(){
        Ok(num) => num , 
        Err(_err) => {
            println!("{}", _err); 
             continue ; 
        },
    }; 

    stats.current_guesses.push(guess); 
    println!(" you guessed {} " , guess); 

    // match 

    match  guess.cmp(&random){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big") , 
        Ordering::Equal => {
             println!("you win") ; 
             stats.games_played += 1 ; 
             stats.games_own += 1 ; 
             save_stats(&stats); 
             println!("stats  {:?} " , stats); 
             break ; 
        }, 
    }

}
}


fn load_stats () -> GameStats{
  match fs::read_to_string("save.json"){
     Ok(contents) => { 
        match::serde_json::from_str(&contents){
          Ok(stats) => stats , 
          Err(_) => GameStats::new(),
        }
     }
     , 
     Err(_) => GameStats::new(), 
  }

}

fn save_stats (stats: &GameStats ) {
  let json = match  serde_json::to_string_pretty(stats){
    Ok(j) => j  , 
    Err(e) => {
      println!(" warning  couldnot serialize stats ,{}", e ); 
      return ; 
    }
  }; 
  if let Err(e) = fs::write("save.json" , json){
  }
}



