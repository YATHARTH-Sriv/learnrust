pub fn addone(i:u64) -> u64 {
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = addone(2);
        assert_eq!(result, 3);
    }
}
