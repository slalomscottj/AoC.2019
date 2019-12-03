extern crate itertools;

use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

use std::collections::HashSet;


fn main() -> io::Result<()> {
    // Get File Name from Command Line
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open File
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Iterate Lines
    reader.lines()
        .map(|l| process_line(l.unwrap()))
        .batching(|it| {
            match it.next() {
                None => None,
                Some(a) => match it.next() {
                    None => None,
                    Some(b) => Some((a,b))
                }
            }
        })
        .map(|(a,b)| {
            let i = a
                .intersection(&b)
                .map(|(x,y)| {
                    x.abs() + y.abs()
                })
                .min();
            i
        })
        .for_each(|l| {
            println!("{:?}",l);
        });

    Ok(())
}

fn process_line(line: String) -> HashSet::<(i64,i64)> {
    let program = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut visits = HashSet::<(i64,i64)>::new();

    for step in program {
        let dir = step.chars().next().unwrap();
        let distance = step[1..].parse::<i64>().unwrap();

        let mut iy: i64 = 0;
        let mut ix: i64 = 0;

        match dir {
            'U' => iy = 1,
            'D' => iy = -1,
            'L' => ix = -1,
            'R' => ix = 1,
            _ => panic!("Unknown compass direction {}", dir)
        }
        println!("Dist {}", distance);
        for _ in 0..distance
        {
            x += ix;
            y += iy;

            println!("{} {}", x, y);
            visits.insert((x,y));
        }
    }

    println!("");
    return visits;
}


