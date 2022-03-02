use std::thread;

fn main() {

    let mut handles = vec![];

    for i in 0..10 {

        let handle = thread::spawn(move || {

            println!("Hello Multithread {}", i);
            
        });

        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

    return

}
