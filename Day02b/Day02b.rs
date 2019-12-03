use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


fn main() -> io::Result<()> {
    // Get File Name from Command Line
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open File
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Iterate Lines
    for line in reader.lines() {
        process_line(line?);
        println!("");
    }

    Ok(())
}

fn process_line(line: String) {
    let numbers: Vec<u64> = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for a in 0..99
    {
        for b in 0..99
        {
        
            let mut instance = numbers.to_vec();

            instance[1] = a;
            instance[2] = b;
            print!("{} {} ",a,b);
            let result = run_program(&mut instance);

            if result == 19690720 {
                return;
            }
        }
    }
}

fn run_program(numbers: &mut Vec<u64>) -> u64 {
    let mut pc: usize = 0;
    while {
        pc = process_vector(pc, numbers);
        pc < usize::max_value()
    }{}

    println!("0:{}", numbers[0]);
    return numbers[0];
}

fn process_vector(pc: usize, numbers: &mut Vec<u64>) -> usize {
    match numbers[pc] {
        1 => {
            let ia = numbers[pc + 1] as usize;
            let ib = numbers[pc + 2] as usize;
            let ic = numbers[pc + 3] as usize;

            let a = numbers[ia];
            let b = numbers[ib];
            let c = a + b;

            // We don't care about interim state
            // println!("OP:{} A:{}({}) B:{}({}) C:{}={}", numbers[pc], ia, a, ib, b, ic, c);

            numbers[ic] = c;
            return pc + 4;
        },
        2 => {
            let ia = numbers[pc + 1] as usize;
            let ib = numbers[pc + 2] as usize;
            let ic = numbers[pc + 3] as usize;

            let a = numbers[ia];
            let b = numbers[ib];
            let c = a * b;

            // We don't care about interim state
            // println!("OP:{} A:{}({}) B:{}({}) C:{}={}", numbers[pc], ia, a, ib, b, ic, c);

            numbers[ic] = c;
            return pc + 4;
        },
        99 => {
            // We don't care about interim state
            // println!("OP:{}", numbers[pc]);
        },
        _ => {
            // We don't care about interim state
            // println!("OP:{} FAIL", numbers[pc]);
        },
    }

    return usize::max_value();
}