use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut unsorted_vect=vec![1,2,3,5,3];
    println!("{:?}",find_mode_and_median(&mut unsorted_vect));
}

 

fn find_mode_and_median(v:&mut Vec<i32>)-> (f64,i32){
   v.sort();
   println!("sorted vector {:?}",v);
   let length=v.len();
   let mut median=0.0;
   if length % 2 == 0{
            let mid1 = v[length / 2 - 1];
            let mid2 = v[length / 2];
            median = (mid1 + mid2) as f64 / 2.0
   }else{
    median = v[length/2] as f64;
   }
   println!("{}",median);

   let mut map=HashMap::new();
   for i in v{
    let count=map.entry(i).or_insert(0);
    *count=*count+1;

   }
   println!("{:?}",map);
   let mut mode=0;
   let mut mode_key_value=0;
   for (key,&value) in &map{  // pattern matching with derefrencing for value here 
     if value>mode_key_value{
         mode_key_value = value;
         let keyvaalue=**key;
         mode = keyvaalue;
     }
   }
    (median,mode)
}





