fn main() {
    let str = "25a";
     match str.parse::<i32>() {
         Ok(data) => {
             println!("str change into i32 -> {}", data);
         }
         Err(err) => {
             println!("str change into error {}", err);
         }
     }
 }
