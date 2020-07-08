pub fn loop_loop() {
    let mut i = 0;

    let resul = loop {
        i += 1;
        println!("{}", i);
        if i == 10 {
            break i * 2;
        }
    };

    println!("{:?}", resul)
}

pub fn loop_while() {
    let mut i = 10;

    while i >= 0 {
        println!("{}", i);
        i -= 1;
        if i == 0 {
            println!("Go go go!");
            break;
        }
    };
}

pub fn loop_for() {
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for x in array.iter() {
        println!("{:?}", x);
    }
}
