fn main() {
    let list = [4, 6, 8, 10, 12, 14, 16, 18, 20];
    let index = binary_search_2(&list, 4);

    println!("{:?}", index);
}

fn binary_search(list: Vec<u32>, target: u32) -> u32 {
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

fn binary_search_2<T: cmp::PartialOrd>(list: &[T], target: T) -> Option<usize> {
    let middle = ((list.len() / 2) as f32).ceil() as usize;

    println!("{}", list.len());

    match list.get(middle) {
        None => None,
        Some(val) => {
            if *val == target {
                Some(middle)
            } else if *val > target {
                let sublist = &list[..middle];
                binary_search_2(sublist, target)
            } else {
                let sublist = &list[(middle + 1)..];
                binary_search_2(sublist, target).map(|pos| pos + middle + 1)
            }
        }
    }
}
