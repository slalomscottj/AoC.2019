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
    let mut numbers: Vec<u64> = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    
    let mut pc: usize = 0;
    while {
        pc = process_vector(pc, &mut numbers);
        pc < usize::max_value()
    }{}
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

            println!("OP:{} A:{}({}) B:{}({}) C:{}={}", numbers[pc], ia, a, ib, b, ic, c);

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

            println!("OP:{} A:{}({}) B:{}({}) C:{}={}", numbers[pc], ia, a, ib, b, ic, c);

            numbers[ic] = c;
            return pc + 4;
        },
        99 => {
            println!("OP:{}", numbers[pc]);
        },
        _ => {
            println!("OP:{} FAIL", numbers[pc]);
        },
    }

    return usize::max_value();
}