/*

Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

*/

use std::collections::HashMap;

fn main() {
    let integers = vec![9, 5, 2, 8, 1, 7, 4, 2, 6, 2, 3, 2];
    println!("The mean is {}", mean(&integers));
    println!("The median is {}", median(&integers));
    println!("The most common value is {}", mode(&integers));
}

fn mean(integers: &Vec<i32>) -> i32 {
    // Loop all the values and divide by the length
    let mut sum = 0;
    for item in integers {
        sum += item;
    }
    let result = sum / integers.len() as i32;
    result
}

// The sorted middle element's value
fn median(integers: &Vec<i32>) -> i32 {
    let mid = integers.len() / 2;
    let sorted = merge_sort(&integers);
    println!("The sorted vector is {:?}", sorted);
    sorted[mid]
}

// The value that occurs most often
fn mode(integers: &Vec<i32>) -> u32 {
    // Create a hashmap of ints and empty values {1: 0, 2: 0 , ...} where the value is the repeated counter
    let mut map = HashMap::new();
    let mut most_repeated_value: u32 = 0;
    let mut most_repeated_times: u32 = 0;
    for integer in integers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
        if count > &mut most_repeated_times {
            most_repeated_value = *integer as u32;
            most_repeated_times = *count;
        }
    }
    most_repeated_value
}

// Merge sort is O(n log n) which is more efficient than the classic O(n") from the normal sort
fn merge_sort(integers: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if integers.len() <= 1 {
        return integers.to_vec();
    }
    let mid = integers.len() / 2;
    let mut left = merge_sort(&integers[..mid].to_vec());
    let mut right = merge_sort(&integers[mid..].to_vec());

    if left.len() == 0 {
        result.append(&mut right);
    } else if right.len() == 0 {
        result.append(&mut left);
    }

    if left.len() != 0 && right.len() != 0 {
        let mut left_index = 0;
        let mut right_index = 0;

        while left_index < left.len() && right_index < right.len() {
            if left[left_index] < right[right_index] {
                result.push(left[left_index]);
                left_index += 1;
            } else {
                result.push(right[right_index]);
                right_index += 1;
            }
        }

        while left_index < left.len() {
            result.push(left[left_index]);
            left_index += 1;
        }

        while right_index < right.len() {
            result.push(right[right_index]);
            right_index += 1;
        }
    }
    result
}
