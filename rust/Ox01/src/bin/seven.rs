struct User {
  name : String , 
  age : u32 , 
}
 
fn main ( ) { 
  let user1 = User {
    name : String::from ( "srivx ") , 
    age : 22 ,
  }  ; 
  pass_struct_print(user1);

}

fn pass_struct_print(str : User){
   println!( " user name : {}" , str.name); 
}