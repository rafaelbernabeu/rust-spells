

fn main() {

    let name: &str = "Rafael";   //lives in stack
    let middle_name: &str = "Abrão";
    let last_name: &str = "Bernabeu";

    let mut full_name: String = name.to_owned() + " " + &middle_name + " " + &last_name;
    let mut welcome_message: String = String::from("hello world");   //lives in heap

    welcome_message.push_str( &full_name);


    let name_len = full_name.len();

    println!("{}, {}", welcome_message, full_name);


    welcome_message.clear();
    let mut welcome_slice = &welcome_message[welcome_message.len()..];


    print!("{}", welcome_slice)





}