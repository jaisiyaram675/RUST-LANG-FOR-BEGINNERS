//  means that borrow karna kisi cheej ko kisi se  kuch time ke liye lena,  yahan par maine usko pura pass  na karke uska reference pass kar diya hai
// fn main(){
//     let s1:String = String::from("company");
//     let len:usize = calculate_length(&s1); 
//      println!(" the value of len will be {} and {} " , s1 , len);
//  }
//   fn calculate_length(s:&String)->usize{
//      return s.len();
//   }




// ******************************************************


//  here we do that ki ham reference me change kar sakte hai
// fn main(){
//     let mut s1:String = String::from("I PLAYZ");
//     append_string(&mut s1);  // we are passing s1 as mutable reference because we want to change its value  here we are not passing by value because it is a reference to the String object.
 
//      println!(" the value of len will be {} " , s1);
//  }
// //  if you want to change the value of a String object then you have to pass it as mutable reference
//  fn append_string(s:&mut String){
//     s.push_str(" is a good company");
//  }



// **********************************************

// write function in mutable reference 
//  here i pass all reference of s1 in thi using &mut

fn main(){
    let mut s1:String = String::from("I PLAYZ");
   let w1 =&mut s1;
   w1.push_str(" is a good company");
   println!("  {} " , w1);
   let w2 =&mut s1;
   w2.push_str(" and I love it");
  
   println!("  {} " , w2);

 
}