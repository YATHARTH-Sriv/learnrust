trait summary{
    fn summarize(&self)->String;
}

struct Post{
    author:String,
    content:String
}

impl summary for Post  {
    fn summarize(&self)->String {
        self.author.clone()
    }
}

fn main() {
    println!("Hello, world!");
    let newpost=Post{
        author:String::from("yayy"),
        content:String::from("yayyyy")
    };
    println!("{}",newpost.summarize());

    let str1=String::from("stringone");
    let str2=String::from("String twoww");
    let result=understand_lifetimes(&str1, &str2);
    println!("{}",result);
}

fn understand_lifetimes<'a>(x:&'a str,y:&'a str)->&'a str{
    if x > y{
        x
    } else{
        y
    }

}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

