fn main() {
    let list = [4, 6, 8, 10, 12, 14, 16, 18, 20];
    let index = binary_search(&list, 4);
    let a: Option<u32> = Some(40);
    let b = a.map(|c| c + 40);

    println!("{:?}", index);
}

fn binary_search_alt(list: Vec<u32>, target: u32) -> u32 {
    let mut low: i32 = 0;
    let mut high: i32 = (list.len() - 1) as i32;
    let mut answer = 0;

    while low <= high {
        let middle = ((low + high) as f32 / 2_f32).ceil() as isize;
        let chute = list[middle as usize];
        println!("middle: {}, low: {}, high: {}", middle, low, high);

        if chute == target {
            answer = middle as u32;
            break;
        }
        if chute > target as u32 {
            high = (middle - 1) as i32;
        } else {
            low = (middle + 1) as i32;
        }
    }
    answer
}

use std::cmp;

fn binary_search<T: cmp::PartialOrd>(list: &[T], target: T) -> Option<usize> {
    let middle = ((list.len() / 2) as f32).ceil() as usize;

    println!("{}", list.len());

    match list.get(middle) {
        None => None,
        Some(val) => {
            if *val == target {
                Some(middle)
            } else if *val > target {
                let sublist = &list[..middle];
                binary_search(sublist, target)
            } else {
                let sublist = &list[(middle + 1)..];
                binary_search(sublist, target).map(|pos| pos + middle + 1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn finds_number_near_end_of_list() {
        let num_slice = &[
            2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
        ];

        assert_eq!(binary_search(num_slice, 70), Some(16));
    }

    #[test]
    fn finds_number_near_start_of_list() {
        let num_slice = &[
            2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
        ];

        assert_eq!(binary_search(num_slice, 5), Some(2));
    }

    #[test]
    fn returns_none_for_numbers() {
        let num_slice = &[
            2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
        ];

        assert_eq!(binary_search(num_slice, 1), None);
    }

    #[test]
    fn finds_char() {
        let char_slice = &[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        assert_eq!(binary_search(char_slice, 'l'), Some(11));
    }

    #[test]
    fn returns_none_for_chars() {
        let char_slice = &['a', 'b', 'c'];

        assert_eq!(binary_search(char_slice, 'l'), None);
    }
}
