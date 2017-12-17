pub fn access_circular<T : Copy>(i: usize, list: &Vec<T>) -> T {
    let index = i % list.len();
    list[index]
}

pub fn set_circular<T : Copy>(i: usize, list: &mut Vec<T>, v: T) {
    let i = i % list.len();
    list[i] = v;
}

pub fn get_slice<T : Copy>(i: usize, len: usize, list: &Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for at in i..(i + len) {
        result.push(access_circular(at, list));
    }

    result
}
