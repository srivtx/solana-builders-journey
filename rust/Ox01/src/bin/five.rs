struct User {
  name  : String , 
  age : u32  , 
}


fn main (){
   let s1 : User =   User {
    name : String::from ( " srivtx"), 
    age : 22 ,
   }; 
   println!(" Name :  {} , age : {} " ,s1.name  , s1.age  ); 
  }