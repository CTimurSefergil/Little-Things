use std::fs::read_to_string;

fn main() {
    let satırlar: Vec<String> = read_each_line("input.txt");

    let mut toplam: i32 = 0;

    for i in 0..satırlar.len() {
        let karakterler: Vec<String> = convert_to_digit(&satırlar[i]);
        toplam = toplam + find_and_sum_needed_numbers(karakterler);
    }
    println!("{:?}", toplam);
}

fn read_each_line(elven: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(elven).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn convert_to_digit(line: &String) -> Vec<String> {
    let mut str = Vec::new();
    let c: Vec<char> = line.chars().collect();
    for i in &c {
        str.push(i.to_string());
    }
    str
}

fn find_and_sum_needed_numbers(digits: Vec<String>) -> i32 {
    let mut _sum = 0;

    // println!("{:?}", &digits);

    let mut line_integers: Vec<i32> = Vec::new();

    for i in &digits {
        // let mut line_integers: Vec<i32> = Vec::new();
        // println!("{:?}", line_integers);
        match i.parse::<i32>() {
            Ok(n) => line_integers.push(n),
            Err(_) => (),
        };
        // BU HALİ NEDEN ÇALIŞMADI ÖĞREN !!!
        // if line_integers.len() > 1 {
        // _sum = _sum + (&line_integers[0] * 10) + &line_integers[line_integers.len() - 1];
        // }
        // if line_integers.len() == 1 {
        // _sum = _sum + (&line_integers[0] * 11);
        //  println!("{:?}", &_sum);
        // }
    }
    // println!("{:?}", &line_integers);
    if line_integers.len() > 1 {
        _sum = _sum + (&line_integers[0] * 10) + &line_integers[line_integers.len() - 1];
    }
    if line_integers.len() == 1 {
        _sum = _sum + (&line_integers[0] * 11);
        // println!("{:?}", &_sum);
    }

    _sum
}
