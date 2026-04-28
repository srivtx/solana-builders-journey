
use std::io ; 
use rand::Rng ; 
use std::cmp::Ordering ; 
fn main() {
    println!(" guess the number"); 

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

    println!(" you guessed {} " , guess); 

    // match 

    match  guess.cmp(&random){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big") , 
        Ordering::Equal => {
             println!("you win") ; 
             break ; 
        }, 
    }

}


}
