extern crate rand;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

fn sum(mut number: u32) -> u32 {
    let mut sum_all_nums = 0;
    while number > 0 {
        sum_all_nums += number % 10;
        number /= 10;
    }
    return sum_all_nums;
}

fn retail() -> String {
    let mut rng = rand::thread_rng();
    let first_part_die = Uniform::from(0..998);
    let second_part_die = Uniform::from(0..9999999);
    let invalid_nums: [u32; 6] = [333, 444, 555, 666, 777, 888];
    let mut first_part: u32 = first_part_die.sample(&mut rng);
    let mut second_part: u32 = second_part_die.sample(&mut rng);

    while invalid_nums.contains(&first_part) {
        first_part = first_part_die.sample(&mut rng);
    }

    while (sum(second_part) % 7) != 0 {
        second_part = second_part_die.sample(&mut rng);
    }

    return format!("{:03}-{:07}", first_part, second_part);
}

fn office97() -> String {
    let mut rng = rand::thread_rng();
    let first_part_die = Uniform::from(1..9991);
    let second_part_die = Uniform::from(0..9999999);
    let mut first_part: u32;
    let mut second_part: u32 = second_part_die.sample(&mut rng);

    loop {
        first_part = first_part_die.sample(&mut rng);
        if first_part % 10 == (((first_part % 100) / 10) + 1) && first_part < 9991 {
            break;
        }
    }

    while (sum(second_part) % 7) != 0 {
        second_part = second_part_die.sample(&mut rng);
    }

    return format!("{:04}-{:07}", first_part, second_part);
}

fn oem() -> String {
    let mut rng = rand::thread_rng();
    let third_part_die = Uniform::from(0..999999);
    let second_part_array: [u32; 9] = [95, 96, 97, 98, 99, 00, 01, 02, 03];
    let first_part: u32 = rng.gen_range(1..366);
    let second_part_indice: usize = rng.gen_range(0..8);
    let second_part: u32 = second_part_array[second_part_indice];
    let mut third_part: u32 = third_part_die.sample(&mut rng);
    let last_part: u32 = rng.gen_range(0..99999);

    while (sum(third_part) % 7) != 0 {
        third_part = third_part_die.sample(&mut rng);
    }

    return format!(
        "{:03}{:02}-{:07}-{:05}",
        first_part, second_part, third_part, last_part
    );
}

fn main() {
    println!("Retail key: {}", retail());
    println!("Office 97 key: {}", office97());
    println!("OEM key: {}", oem());
}
