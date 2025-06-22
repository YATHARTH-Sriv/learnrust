pub fn add(a:i64,b:i64)->i64{
    a + b
}

struct Rectangle{
    height:f64,
    width:f64
}

impl Rectangle{
    fn check(&self,other:&Rectangle)->bool{
        self.height>other.height && self.width>other.width
    }
}

#[cfg(test)]
mod test{
    use crate::{add, Rectangle};


    #[test]
    fn test_add(){
        assert_eq!(add(2, 3),5);
    }

    #[test]
    fn check(){
        let rect1=Rectangle{
            height:4.0,
            width:5.0
        };

        let rect2=Rectangle{
            height:6.0,
            width:7.0
        };

        assert_eq!(rect1.check(&rect2),false);
    }
}