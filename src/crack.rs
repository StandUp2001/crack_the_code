use crate::codes::CodesFile;
use serde::Deserialize;

macro_rules! remove_wrong_codes {
    ($self:ident, $codes:ident, $place:ident, $condition:tt) => {
        if $place > 0 {
            let mut indexes: Vec<usize> = Vec::new();
            for (index, code) in $codes.iter().enumerate() {
                let mut count = 0;
                for (i, number) in code.iter().enumerate() {
                    if $self.input.contains(number) && $self.input[i] $condition *number{
                        count += 1;
                    }
                }
                if count != $place {
                    indexes.push(index);
                }
            }
            for index in indexes.iter().rev() {
                $codes.remove(*index);
            }
        }
    };
}

#[derive(Deserialize)]
pub struct Crack {
    input: Vec<i8>,
    pub correct_place: i8,
    pub wrong_place: i8,
}

impl Crack {
    pub fn create_codes() -> Vec<Crack> {
        let codes = CodesFile::read_codes();
        let length = codes[0].input.len();
        for code in &codes {
            if code.input.len() != length {
                panic!("All codes must have the same length");
            }
        }
        codes
    }

    fn generate_codes(
        possible_numbers: &Vec<i8>,
        current_code: Vec<i8>,
        remaining_depth: usize,
        possible_codes: &mut Vec<Vec<i8>>,
    ) {
        if remaining_depth == 0 {
            possible_codes.push(current_code);
            return;
        }

        for num in possible_numbers {
            let mut new_code = current_code.clone();
            new_code.push(*num);
            Crack::generate_codes(
                possible_numbers,
                new_code,
                remaining_depth - 1,
                possible_codes,
            );
        }
    }

    pub fn create_possible_codes(codes: &Vec<Crack>) -> Vec<Vec<i8>> {
        let mut possible_numbers: Vec<i8> = vec![];
        let mut possible_codes: Vec<Vec<i8>> = vec![];
        for code in codes {
            for number in code.input.iter() {
                if !possible_numbers.contains(&number) {
                    possible_numbers.push(*number);
                }
            }
        }

        Crack::generate_codes(
            &possible_numbers,
            vec![],
            codes[0].input.len(),
            &mut possible_codes,
        );

        possible_codes
    }

    pub fn remove_everything_with_0_0(codes: &Vec<Crack>, possible_codes: &mut Vec<Vec<i8>>) {
        let mut remove_numbers: Vec<i8> = Vec::new();
        for code in codes {
            if code.correct_place == 0 && code.wrong_place == 0 {
                for number in code.input.iter() {
                    remove_numbers.push(*number);
                }
            }
        }

        let mut indexes: Vec<usize> = Vec::new();

        for (index, code) in possible_codes.iter().enumerate() {
            for number in code.iter() {
                if remove_numbers.contains(number) {
                    indexes.push(index);
                    break;
                }
            }
        }

        for index in indexes.iter().rev() {
            possible_codes.remove(*index);
        }
    }

    pub fn remove_wrong_wrong_place(&self, possible_codes: &mut Vec<Vec<i8>>) {
        let wrong = self.wrong_place;
        remove_wrong_codes!(self, possible_codes, wrong, !=);
    }

    pub fn keep_correct_place(&self, possible_codes: &mut Vec<Vec<i8>>) {
        let correct = self.correct_place;
        remove_wrong_codes!(self, possible_codes, correct, ==);
    }
}

impl std::fmt::Debug for Crack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: {:?}, Correct Place: {}, Wrong Place: {}",
            self.input, self.correct_place, self.wrong_place
        )
    }
}
