pub mod solution;
use solution::*;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(
        add_two_numbers(
            generate_list_node(vec![2, 4, 3]),
            generate_list_node(vec![5, 6, 4])
        ),
        generate_list_node(vec![7, 0, 8])
    );
    assert_eq!(
        add_two_numbers(generate_list_node(vec![0]), generate_list_node(vec![0])),
        generate_list_node(vec![0])
    );
    assert_eq!(
        add_two_numbers(
            generate_list_node(vec![9, 9, 9, 9, 9, 9, 9]),
            generate_list_node(vec![9, 9, 9, 9])
        ),
        generate_list_node(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}

#[test]
fn test_reverse_int() {
    assert_eq!(reverse_int(123), 321);
    assert_eq!(reverse_int(-123), -321);
    assert_eq!(reverse_int(120), 21);
    assert_eq!(reverse_int(100), 1);
    assert_eq!(reverse_int(2147483647), 0);
    assert_eq!(reverse_int(-2147483648), 0);
}

#[test]
fn test_trailing_zeroes() {
    assert_eq!(trailing_zeroes(3), 0);
    assert_eq!(trailing_zeroes(5), 1);
    assert_eq!(trailing_zeroes(0), 0);
}

#[test]
fn test_count_numbers_with_unique_digits() {
    assert_eq!(count_numbers_with_unique_digits(2), 91);
    assert_eq!(count_numbers_with_unique_digits(0), 1);
}

#[test]
fn test_find_diagonal_order() {
    assert_eq!(
        find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
    assert_eq!(
        find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_convert_to_base7() {
    assert_eq!(convert_to_base7(100), "202");
    assert_eq!(convert_to_base7(-7), "-10");
}

#[test]
fn test_complex_number_multiply() {
    assert_eq!(
        complex_number_multiply(String::from("1+1i"), String::from("1+1i")),
        String::from("0+2i")
    );
    assert_eq!(
        complex_number_multiply(String::from("1+-1i"), String::from("1+-1i")),
        String::from("0+-2i")
    );
}

#[test]
fn test_cal_points() {
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+")
        ]),
        30
    );
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("-2"),
            String::from("4"),
            String::from("C"),
            String::from("D"),
            String::from("9"),
            String::from("+"),
            String::from("+")
        ]),
        27
    );
    assert_eq!(cal_points(vec![String::from("1")]), 1);
}

#[test]
fn test_has_alternating_bits() {
    assert_eq!(has_alternating_bits(5), true);
    assert_eq!(has_alternating_bits(7), false);
    assert_eq!(has_alternating_bits(11), false);
}

#[test]
fn test_self_dividing_numbers() {
    assert_eq!(
        self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
    assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
}

#[test]
fn test_next_greatest_letter() {
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
}

#[test]
fn test_unique_morse_representations() {
    assert_eq!(
        unique_morse_representations(vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg"),
        ]),
        2
    );
    assert_eq!(unique_morse_representations(vec![String::from("a"),]), 1);
}

#[test]
fn test_number_of_lines() {
    assert_eq!(
        number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("abcdefghijklmnopqrstuvwxyz")
        ),
        vec![3, 60]
    );
    assert_eq!(
        number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("bbbcccdddaaa")
        ),
        vec![2, 4]
    );
}

#[test]
fn test_pivot_index() {
    assert_eq!(pivot_index(vec! {2, 3, -1, 8, 4}), 3);
    assert_eq!(pivot_index(vec! {1,-1,4}), 2);
    assert_eq!(pivot_index(vec! {2,5}), -1);
}
