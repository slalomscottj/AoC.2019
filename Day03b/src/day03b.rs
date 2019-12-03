extern crate itertools;

use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

use std::collections::HashSet;
use std::iter::FromIterator;


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
        .map(|(va,vb)| {
            let a = to_set(&va);
            let b = to_set(&vb);
            let i = a
                .intersection(&b)
                .map(|&xy| {
                    va.iter().position(|&r| r == xy).unwrap() + 1 +
                    vb.iter().position(|&r| r == xy).unwrap() + 1
                })
                .min();
            i
        })
        .for_each(|l| {
            println!("{}",l.unwrap());
        });

    Ok(())
}

fn process_line(line: String) -> Vec::<(i64,i64)> {
    let program = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut visits = Vec::<(i64,i64)>::new();

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

        for _ in 0..distance
        {
            x += ix;
            y += iy;

            visits.push((x,y));
        }
    }

    return visits;
}

fn to_set(vec: &Vec<(i64,i64)>) -> HashSet<(i64,i64)> {
    let temp = vec.clone();
    HashSet::from_iter(temp)
}
