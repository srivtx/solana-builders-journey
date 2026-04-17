struct Car { 
  brand : String , 
  speed : u32 ,
}

impl Car {
   fn print_details (&self){
      println!(" car brand name {}  and its speed : {}" , self.brand , self.speed ); 
   }
   fn increase_speed (&mut self ){
     self.speed += 5 ; 
   }
}


fn main ()
{
  let mut car1  = Car {
    brand  : String::from("BMW"), 
    speed : 240, 
  } ; 
  car1.print_details();
  car1.increase_speed(); 
  car1.print_details(); 
}