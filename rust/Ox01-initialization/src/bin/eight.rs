struct User {
  name : String , 
  age : u32 , 
}
 
fn main ( ) { 
  let mut user1 = User {
    name : String::from ( "srivx ") , 
    age : 22 ,
  }  ; 

  println!(" user age {}" , user1.age) ; 

  pass_struct_print(&mut user1);

  println!(" user age {}" , user1.age) ; 

}

fn pass_struct_print(str : &mut User){
   str.name.push_str(" dash"); 
   str.age  += 1 ; 
   println!( " user name : {} user age  : {} " , str.name    , str.age); 
}