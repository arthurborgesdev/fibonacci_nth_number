use std::io;

fn main() {
    loop {
        println!("Ask for the nth Fibonacci number: ");

        let mut asked_number = String::new();
        
        io::stdin()
            .read_line(&mut asked_number)
            .expect("Failed to read line");
        
        let asked_number: i32 = match asked_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if asked_number <= 0 { continue }

        let mut fibonacci = 0;

        if asked_number == 1 { fibonacci = 0 } else if asked_number == 2 { fibonacci = 1 } else {
            let mut last_two_fibonacci = 0;
            let mut last_one_fibonacci = 1;
            
            let mut start = 0;
            while start < asked_number {
                fibonacci = last_one_fibonacci + last_two_fibonacci;
                last_two_fibonacci = last_one_fibonacci;
                last_one_fibonacci = fibonacci;
                start += 1;
            }
        }

        println!("The {} number of Fibonacci sequence is {}", asked_number, fibonacci);

    }
    
}
