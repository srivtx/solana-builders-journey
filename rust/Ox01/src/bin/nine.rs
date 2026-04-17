// struct method 

struct User {
  name : String  , 
  age : u32 
}

impl User{
  fn print_user(&self){
    println!("name : {}" ,self.name  ); 
    println!("age : {}" , self.age ); 
  }
}

fn main (){

  let user1 = User{ 
    name : String::from( " srivtx ")  , 
    age : 22
  }; 
  user1.print_user(); 

}