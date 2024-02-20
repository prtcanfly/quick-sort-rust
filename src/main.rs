use std::cmp::Ordering::{Equal, Greater, Less};

// takes an array slice as input, returns a vector
fn quick_sort(arr: &[i32]) -> Vec<i32> {
    // if the array is 0 or 1 characters, it is already sorted
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    // declare the pivot point and divide the array into multiple vectors
    // the arrays are sorted as lesser than the pivot, equal to the pivot,
    // and greater than the pivot
    let pivot = arr[arr.len() / 2];
    let mut lesser = vec![];
    let mut equal = vec![];
    let mut greater = vec![];

    // recursively sort the values in each array
    for num in arr {
        match num.cmp(&pivot) {
            Less => lesser.push(*num),
            Equal => equal.push(*num),
            Greater => greater.push(*num),
        }
    }

    // concatenate the results
    let mut result = quick_sort(&lesser);
    result.append(&mut equal);
    result.append(&mut quick_sort(&greater));

    // return the final vector
    result
}

fn main() {
    // define the array
    let arr = vec![5, 3, 8, 4, 9, 1, 6, 2, 7];

    // print the sorted array
    println!("{:?}", quick_sort(&arr));
}
