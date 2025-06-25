#[derive(Debug,PartialEq)]
struct Shoe{
    size:u32,
    style:String
}

fn shoes_size(shoes:Vec<Shoe>,user_size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == user_size).collect()

}

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn filter_by_size(){
       let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let new_shoes_vector=shoes_size(shoes, 10);
        // println!("{:?}",shoes);// Does Not work

        assert_eq!(
            new_shoes_vector,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }
}

