use rust11::add;



#[test]
fn it_adds_two() {
    let result = add(2,4);
    assert_eq!(result, 6);
}