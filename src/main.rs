use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_data() -> io::Result<()> {
    // Open the file in read-only mode
    let file = File::open("./src/data.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Iterate through lines in the file
    for line_result in reader.lines() {
        // Unwrap the line or handle potential errors
        let line = line_result?;
        
        // Process each line (here, we're just printing it)
        // println!("{}", line);
        sum = sum + get_number(line.to_string());
    }

    println!("= {sum}");

    Ok(())
}

// Use two pointer to optimize
fn get_number(line: String) -> u32 {
    let mut unit = 0;
    let mut ten = 0;

    // Getting Tens digit, called unit by mistake
    for c in line.chars() {
        // println!("{c}");
        if is_number(c.to_string()){
            unit = c.to_string().parse().expect("Failed");
            break;
        }
    }

    // Getting unit digit, called tens by mistake
    for c in line.chars().rev() {
        // println!("{c}");
        if is_number(c.to_string()){
            ten = c.to_string().parse().expect("Failed");
            break;
        }
    }
    // println!("{}",ten*10 + unit);        Uncomment this line to see all the data

    // Forming the number and returning it
    ten + unit*10
}

fn is_number(input: String) -> bool {
    input.parse::<u32>().is_ok()
}

fn main() {
    get_data().expect("Error reading file");
}

