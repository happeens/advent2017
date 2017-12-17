use super::circular;

pub fn hash_nums(input: &Vec<u16>) -> Vec<u16> {
    let list = (0..256).collect::<Vec<u16>>();
    let (hashed, _, _) = hash_list(&input, &list, 0, 0);

    hashed
}

pub fn hash_str(input: &str) -> String {
    let mut nums = input.chars()
        .map(|it| it as u16)
        .collect::<Vec<u16>>();

    for it in [17, 31, 73, 47, 23].iter() {
        nums.push(*it as u16);
    }

    let mut list = (0..256).collect::<Vec<u16>>();
    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        let (new_list, new_pos, new_skip_size) =
            hash_list(&nums, &list, current_pos, skip_size);

        list = new_list;
        current_pos = new_pos;
        skip_size = new_skip_size;
    }

    let mut result = String::new();
    for chunk in list.chunks(16) {
        let mut chunk_result = chunk[0];
        for i in 1..16 {
            chunk_result = chunk_result ^ chunk[i];
        }
        result.push_str(&format!("{:02x}", chunk_result));
    }

    result
}

fn hash_list(
    nums: &Vec<u16>,
    list: &Vec<u16>,
    current_pos: usize,
    skip_size: usize
) -> (Vec<u16>, usize, usize) {
    let mut list = list.clone();
    let mut current_pos = current_pos;
    let mut skip_size = skip_size;

    for it in nums {
        let it = *it as usize;

        assert!(it <= list.len());

        let mut slice = circular::get_slice(current_pos, it, &list);
        slice.reverse();

        for i in 0..slice.len() {
            circular::set_circular(
                (i + current_pos) as usize,
                &mut list,
                slice[i]
            );
        }

        current_pos += skip_size + it;
        skip_size += 1;
    }

    (list, current_pos, skip_size)
}
