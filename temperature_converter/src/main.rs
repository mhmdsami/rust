use std::io;

fn main() {
    loop {
        println!("[1] convert Fahrenheit to Celcius\n[2] convert Celcius to Fahrenheit\n[0] exit");

        let mut opt = String::new();

        io::stdin().read_line(&mut opt).expect("Failed to read");

        let opt: u16 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if opt == 1 {
            println!("Enter the temperature in Fahrenheit");

            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read");
            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let val = (temp - 32.0) * 5.0 / 9.0;
            println!("{temp}°F = {val}°C");
        } else if opt == 2 {
            println!("Enter the temperature in Celcius");

            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read");
            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let val = temp * 9.0 / 5.0 + 32.0;
            println!("{temp}°C = {val}°F");
        } else if opt == 0 {
            println!("Bye!");
            break;
        } else {
            println!("invalid option")
        }
    }
}
