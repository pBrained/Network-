use std::io;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./i1_Acronyms.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file); 
    let mut total_lines = 0; 
    let mut most_arguments = 0; 

    for(i, result) in rdr.records().enumerate() {
        let record = result?;
        // println!("Line{}; {} feilds", i + 1, record.len());
        total_lines = i + 1;
        if most_arguments < record.len(){
           most_arguments = record.len(); 
        } else {
        continue;
        }
    } 
    
    println!("Total ammount of lines is {}\n The ammount of arguments is {}", total_lines, most_arguments);



    println!("How many questions do you want?");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    match input.parse::<f64>() {
        Ok(number)=> println!("Ammount of questions: {}", number),
        Err(_) => println!("Invalid input! Please evter a valid number."),
    }
    Ok(())
    

}
