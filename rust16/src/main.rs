use std::thread;
use std::time::Duration;

fn main() {
    let v=vec![1,2,3];
    let g=vec![4,5,6];
    let handle=thread::spawn(move || {
       println!("moved vector {:?}",v);
    });

    let handletwo=thread::spawn(move || {
        println!("moved second vector from second thread {:?}",g)
    });

    // handle.join().unwrap();

    for i in 1..5{
        println!("From the main thread {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    handletwo.join().unwrap();
    handle.join().unwrap();
}
