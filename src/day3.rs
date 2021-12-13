extern crate arraymap;
use std::fs;
use std::u16;

fn arr_to_num(arr: [u8; 16], n_bits: usize) -> u16 {
    let mut out: u16 = 0;
    for (idx, val) in arr.iter().enumerate() {
        assert!(*val <= 1);
        out += (*val as u16) << idx;
        if idx >= n_bits - 1 {
            return out;
        }
    }
    out
}

fn most_common_bit_values(nums: &Vec<u16>) -> [u8; 16] {
    let mut counts: [usize; 16] = [0; 16];
    for num in nums.iter() {
        for i in 0..16 {
            counts[i] += ((num >> i) % 2) as usize;
        }
    }
    counts.map(|c| (c >= (nums.len() / 2) as usize) as u8)
}

fn least_common_bit_values(nums: &Vec<u16>) -> [u8; 16] {
    most_common_bit_values(nums).map(|g| 1 - g)
}

fn filter_numbers_most_common(numbers: &Vec<u16>, n_bits: usize, keep: u16) -> u16 {
    assert!(n_bits < 16, "Overflow. n_bits needs to be smaller than 16");

    let mut input = numbers.clone();
    let mut output = Vec::<u16>::new();
    let mut input_ref = &mut input;
    let mut output_ref = &mut output;
    for idx in 0..n_bits + 1 {
        let criteria = arr_to_num(most_common_bit_values(input_ref), n_bits);
        let crit = criteria >> n_bits - idx;

        while input_ref.len() > 0 {
            let num = input_ref.pop().unwrap();
            let bit = num >> n_bits - idx;
            if bit == crit {
                output_ref.push(num);
            }
        }
        // debug
        for num in output_ref.iter() {
            println!("{:0>16b}", num)
        }
        println!("\n");

        if output_ref.len() == 1 {
            return output_ref[0];
        }
        if output_ref.len() == 2 {
            return ((output_ref[0] >> 1) << 1) + keep;
        }
        let tmp_ref = input_ref;
        input_ref = output_ref;
        output_ref = tmp_ref;
    }
    return output_ref[0];
}

fn filter_numbers_least_common(numbers: &Vec<u16>, n_bits: usize, keep: u16) -> u16 {
    assert!(n_bits < 16, "Overflow. n_bits needs to be smaller than 16");

    let mut input = numbers.clone();
    let mut output = Vec::<u16>::new();
    let mut input_ref = &mut input;
    let mut output_ref = &mut output;
    for idx in 0..n_bits + 1 {
        let criteria = arr_to_num(least_common_bit_values(input_ref), n_bits);
        let crit = criteria >> n_bits - idx;

        while input_ref.len() > 0 {
            let num = input_ref.pop().unwrap();
            let bit = num >> n_bits - idx;
            if bit == crit {
                output_ref.push(num);
            }
        }
        // debug
        for num in output_ref.iter() {
            println!("{:0>16b}", num)
        }
        println!("\n");

        if output_ref.len() == 1 {
            return output_ref[0];
        }
        if output_ref.len() == 2 {
            return ((output_ref[0] >> 1) << 1) + keep;
        }
        let tmp_ref = input_ref;
        input_ref = output_ref;
        output_ref = tmp_ref;
    }
    return output_ref[0];
}

fn solve_3_1(filename: &str) -> (u16, u16) {
    // Solves AOC Day 3 Problem 1
    let data = fs::read_to_string(filename).expect("Something went wrong while reading input.");

    // Number of bits in the given binary numbers
    let n_bits = data.split("\n").next().unwrap().len();

    // Accumulate counts
    let mut counts: [u32; 16] = [0; 16];

    let mut n_lines = 0;
    for line in data.lines() {
        for (idx, digit) in line.chars().rev().enumerate() {
            counts[idx] += digit.to_digit(2).unwrap();
        }
        n_lines += 1;
    }

    // get most common bit value
    let gamma_arr: [u8; 16] = counts.map(|c| (c >= (n_lines / 2) as u32) as u8);
    // Least common is equivalent to not most common
    let epsilon_arr = gamma_arr.map(|g| 1 - g);

    // Make uints from boolean arrays

    let gamma = arr_to_num(gamma_arr, n_bits);
    let epsilon = arr_to_num(epsilon_arr, n_bits);

    return (gamma, epsilon);
}

fn solve_3_2(filename: &str) -> (u16, u16) {
    // Solves AOC Day 3 Problem 1
    let data = fs::read_to_string(filename).expect("Something went wrong while reading input.");

    // Number of bits in the given binary numbers
    let n_bits = data.split("\n").next().unwrap().len();

    // Accumulate counts
    let mut counts: [u32; 16] = [0; 16];

    let mut n_lines = 0;
    let mut numbers: Vec<u16> = vec![];
    for line in data.lines() {
        for (idx, c) in line.chars().rev().enumerate() {
            let digit = c.to_digit(2).unwrap();
            counts[idx] += digit;
        }
        numbers.push(u16::from_str_radix(line, 2).expect("Non-binary string."));
        n_lines += 1;
    }

    let oxygen = filter_numbers_most_common(&numbers, n_bits, 1);
    let co2 = filter_numbers_least_common(&numbers, n_bits, 0);
    return (oxygen, co2);
}
