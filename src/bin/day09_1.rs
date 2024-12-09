const FILE_PATH: &str = "day09.txt";

#[inline]
fn get_first_empty_sector(file_system: &[Option<usize>]) -> usize {
    for (i, section) in file_system.iter().enumerate() {
        if section.is_none() {
            return i;
        }
    }

    panic!("No such empty sector found");
}

#[inline]
fn get_last_full_sector(file_system: &[Option<usize>]) -> usize {
    for (i, section) in file_system.iter().enumerate().rev() {
        if section.is_some() {
            return i;
        }
    }

    panic!("No such full sector found");
}

fn main() {
    let mut file_system: Vec<Option<usize>> = Vec::new();

    let text = std::fs::read_to_string(FILE_PATH).expect("Error reading input file");

    let mut id = 0_usize;
    let mut is_section_used = true;
    for c in text[..text.len()-1].chars() { // skip newline and iterate over chars
        if is_section_used {
            for _ in 0..c.to_digit(10).unwrap() {
                file_system.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                file_system.push(None);
            }
        }
        is_section_used = !is_section_used;
    }

    loop {
        let first_empty_index = get_first_empty_sector(&file_system);
        let last_full_index = get_last_full_sector(&file_system);

        if first_empty_index > last_full_index {
            break;
        }

        file_system[first_empty_index] = file_system[last_full_index];
        file_system[last_full_index] = None;
    }

    let mut checksum = 0_usize;
    for (i, sector) in file_system.into_iter().enumerate() {
        if sector.is_none() {
            break;
        }
        checksum += i * sector.unwrap();
    }

    println!("{}", checksum);
}

