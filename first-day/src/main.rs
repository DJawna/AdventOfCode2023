use std::io::prelude::*;
use std::{ env::args,  io::Error, io::ErrorKind, fs::File, io::BufReader};


fn main() -> Result<(),std::io::Error> {
    let with_words = true;
    let verbose = true;
    let the_path: String = match args().skip(1).next() {
        Some(candidate) => candidate,
        None => return Err(Error::new(ErrorKind::InvalidInput, "no input file")),
    };

    let file = match File::open(the_path.as_str())  {
        Ok(candidate) => candidate,
        Err(error) => return Err(Error::new(error.kind(),format!("Some error happened during openning the file: {} the error: {}", the_path, error))) 
    }; 

    let mut reader = BufReader::new(file);
    let digits = ['0','1','2','3','4','5','6','7','8','9'];
    let digits_size: usize = 10;
    let mut keep_processing;
    let mut sum: u32 = 0;
    
    loop {

        let mut string_buffer = String::new();
        keep_processing = match reader.read_line(&mut string_buffer){
            Ok(size) => size > 0,
            Err(error) => return Err(Error::new(error.kind(), "An error hapened during reading the latest line"))
        };

        if !keep_processing {
            break;
        }

        let mut working_string_buffer: String = String::clone(&string_buffer);

            if with_words{
                working_string_buffer =
                    working_string_buffer.to_lowercase()
                            .replace("zero", "0")
                            .replace("one", "1")
                            .replace("two", "2")
                            .replace("three", "3")
                            .replace("four", "4")
                            .replace("five", "5")
                            .replace("six", "6")
                            .replace("seven", "7")
                            .replace("eight", "8")
                            .replace("nine", "9");
            }

        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;
        for line_char in working_string_buffer.chars(){
            for digit_index in usize::MIN..digits_size {
                if line_char == digits[digit_index]{
                    if first_digit == None {
                        first_digit = Some(digit_index as u32);

                    }
                    last_digit = Some(digit_index as u32);
                }
            }
        }


        let unwrapped_first_digit: u32 = match  first_digit {
            Some(val) => val,
            None => return Err(Error::new(ErrorKind::InvalidData, 
                format!("first digit not found in the line: {}", string_buffer)))
        };

        let unwrapped_second_digit: u32 = match last_digit{
            Some(val) => val,
            None => return Err(Error::new(ErrorKind::InvalidData,
                format!("last digit not found in the line: {}", string_buffer)))
        };
        let sub_total= (unwrapped_first_digit * 10) + unwrapped_second_digit;
        if verbose {

            println!("before substitution: {}", &string_buffer);
            println!("after substitution: {}", &working_string_buffer);
            println!("subtotal: {}", sub_total);
        }
        sum += sub_total;

    }

    print!("The full sum: {}", sum);

    return Ok(());
}
