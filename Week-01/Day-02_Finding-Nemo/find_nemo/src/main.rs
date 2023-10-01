use std::io;

fn find_nemo(words: &str) -> String {
    
    let mut count = 0;
    for word in words.split_whitespace() {
        count += 1;
        if word == "Nemo" {
            return format!("I found Nemo at {}!", count);
        }
    }
    return String::from("I can't find Nemo :(");
}


fn main() {

    loop{
        println!("Please enter the Text to find Nemo");

        let mut mystr=String::new();

            io::stdin()
            .read_line(&mut mystr)
            .expect("Could not read text input");

            println!("{}", find_nemo(&mystr));

    }

}
