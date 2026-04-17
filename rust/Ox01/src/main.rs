fn main() {
    let s  = String::from("hello world main"); 
    let g = greet(s) ;
    println!("{}", g ) ; 
}

 fn greet(str:String) -> String 
    {
        return str ; 
    }
