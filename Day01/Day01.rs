use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut total: u64 = 0;

    for wline in reader.lines() {

        let mass = wline?.parse::<u64>().unwrap();
        print!("{} ", mass);
        
        let fuel = calc_fuel(mass);
        print!("{} ", fuel);

        total = total + fuel;
        println!("{} ", total);        

    }

    Ok(())
}

fn calc_fuel(mass: u64) -> u64 {
    let mut fuel: u64 = mass / 3;
    
    if { fuel > 2 }
    { fuel = fuel - 2 }
    else
    { fuel = 0 }
    print!("{} ", fuel);

    if { fuel > 0 }
    { fuel = fuel + calc_fuel(fuel) }
    
    return fuel;
}