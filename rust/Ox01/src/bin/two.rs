fn main(){
  let s = String::from("this is demo "); 
  print_str(&s); 
  println!("{}",s); 
}

fn print_str(str : &String ){
   println!("{}" , str); 
}