use std::ops::Deref;

struct MyBox<T>(T);

impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test{

    use super::*;
    #[test]
   fn test(){
     let x=5;
    let y=MyBox(x);

    
    assert_eq!(5,*y)
}
}