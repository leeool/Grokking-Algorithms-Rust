fn swap<T: Copy>(list: &mut [T], index_a: usize, index_b: usize) {
    let tmp = list[index_a];
    list[index_a] = list[index_b];
    list[index_b] = tmp;
}

fn selection_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] > list[j] {
                swap(list, i, j);
            }
        }
    }
}

fn main() {
    let list = &mut [156, 141, 35, 94, 88, 61, 111];
    print!("{:?} => ", list);
    selection_sort(list);
    print!("{:?}\n", list);
}
