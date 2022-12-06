fn main() {
    let input: &str = include_str!("../../input/day06/input.txt");

    let r1 = find_start_of_message_marker(input, 4);
    println!("{:?}", r1);

    let r2 = find_start_of_message_marker(input, 14);
    println!("{:?}", r2);
}

fn find_start_of_message_marker(input: &str, len: usize) -> usize {
    for i in 0..(input.len()-len) {
        if is_diff(&input[i..(i+len)]) {
            return i+len;
        }
    }

    0
}

fn is_diff(input: &str) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input.chars().nth(i) == input.chars().nth(j) {
                return false;
            }
        }
    }
    true
}

