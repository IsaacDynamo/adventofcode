use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(input: &str) -> Result<Vec<bool>, AppErr> {

    let mut bits = Vec::<bool>::new();
    for c in input.chars() {

        if !c.is_ascii_hexdigit() {
            break;
        }

        let nibble = c.to_digit(16).unwrap();

        bits.push(nibble & 0b1000 != 0);
        bits.push(nibble & 0b0100 != 0);
        bits.push(nibble & 0b0010 != 0);
        bits.push(nibble & 0b0001 != 0);
    }

    Ok(bits)
}

fn main() -> Result<(), AppErr> {


    assert!(part1( &parse("D2FE28")? ) == 6);
    assert!(part1( &parse("8A004A801A8002F478")? ) ==  16);
    assert!(part1( &parse("620080001611562C8802118E34")? ) ==  12);
    assert!(part1( &parse("C0015000016115A2E0802F182340")? ) == 23);
    assert!(part1( &parse("A0016C880162017C3686B18A3D4780")? ) ==  31);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2( &parse("C200B40A82")? ) == 3);
    assert!(part2( &parse("04005AC33890")? ) == 54);
    assert!(part2( &parse("880086C3E88112")? ) == 7);
    assert!(part2( &parse("CE00C43D881120")? ) == 9);
    assert!(part2( &parse("D8005AC2A8F0")? ) == 1);
    assert!(part2( &parse("F600BC2D8F")? ) == 0);
    assert!(part2( &parse("9C005AC2F8F0")? ) == 0);
    assert!(part2( &parse("9C0141080250320F1802104A08")? ) == 1);
 
    println!("{}", part2(&input));

    Ok(())
}

fn take_num(slice: &mut &[bool], len: usize) -> u64 {
    let mut n = 0;
    for i in 0..len {
        n = n << 1;
        if slice[i] {
            n |= 1;
        }
    }
    *slice = &slice[len..];
    n
}

fn take_slice<'a>(slice: &mut &'a[bool], len: usize) -> &'a[bool] {
    let res = &slice[0..len];
    *slice = &slice[len..];
    res
}


fn part1(bits: &[bool]) -> u64 {

    
    let mut stream = bits;


    fn packet(stream: &mut &[bool]) -> u64 {
        let mut sum = 0;

        let version = take_num(stream, 3);
        let id_type = take_num(stream, 3);

        sum += version;

        match id_type {
            4 => {
                loop {
                    let group = take_num(stream, 5);
                    if group & 0b10000 == 0 {
                        break;
                    }
                } 
            },
            _ => {
                let len_type = take_num(stream, 1);         
                match len_type {
                    0 => {
                        let len = take_num(stream, 15);
                        let mut sub = take_slice(stream, len as usize);
                        while sub.len() != 0 {
                            sum += packet(&mut sub);
                        }
                    },
                    1 => {
                        let len = take_num(stream, 11);
                        for _ in 0..len {
                            sum += packet(stream);
                        }
                    },
                    _ => ()
                }
            }
        }

        sum
    }

    packet(&mut stream)
}

fn part2(bits: &[bool]) -> u64 {

    
    let mut stream = bits;

    fn packets(stream: &mut &[bool]) -> Vec<u64> {
        let len_type = take_num(stream, 1);   
        let mut res = Vec::new();
        match len_type {
            0 => {
                let len = take_num(stream, 15);
                let mut sub = take_slice(stream, len as usize);
                while sub.len() != 0 {
                    res.push( packet(&mut sub) );
                }
            },
            1 => {
                let len = take_num(stream, 11);
                for _ in 0..len {
                    res.push( packet(stream) );
                }
            },
            _ => panic!()
        }

        res
    }


    fn packet(stream: &mut &[bool]) -> u64 {
        let mut n;

        let _version = take_num(stream, 3);
        let id_type = take_num(stream, 3);

        match id_type {
            4 => {
                n = 0;
                loop {
                    let group = take_num(stream, 5);
                    n = (n << 4) | (group & 0xF);
                    if group & 0b10000 == 0 {
                        break;
                    }
                }
            },
            0 => {         
                n = packets(stream).iter().sum();
            },
            1 => {         
                n = packets(stream).iter().product();
            },
            2 => {         
                n = *packets(stream).iter().min().unwrap();
            },
            3 => {         
                n = *packets(stream).iter().max().unwrap();
            },
            5 => {         
                let p = packets(stream);
                n = (p[0] > p[1]) as u64; 
            },
            6 => {         
                let p = packets(stream);
                n = (p[0] < p[1]) as u64; 
            },
            7 => {         
                let p = packets(stream);
                n = (p[0] == p[1]) as u64; 
            },
            _ => panic!()
        }

        n
    }

    packet(&mut stream)
}