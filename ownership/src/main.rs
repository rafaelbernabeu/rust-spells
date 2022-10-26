fn main() {

    {
        let a: &str = "a";
        let b: &str = "b";

        let c: String = a.to_owned() + b;
        println!("c={}", c);
    }

    {
        let mut a: String = String::from("Hello");
        a.push_str("World");
    
        println!("a={}", a);
        //print a content

    }

    {
        let a: String = "a".to_owned();
        let b: String = makes_copy(&a);

        println!("{}", takes_ownership(a));
        println!("{}", b);
        println!("{}", b);
    }

}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}

fn makes_copy(some_string: &String) -> String {
    println!("{}", some_string);
    return String::from(some_string);
}
