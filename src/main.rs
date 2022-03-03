use std::io;
use std::io::prelude::*;

fn main() {

    let mut line = String::with_capacity(500_000); // FIX ME!

    io::stdin().lock().read_to_string(&mut line);
    let mut values: Vec<i32> = Vec::with_capacity(line.len());
    values = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    //let mut values: Vec<i32> = vec![-977, -964, -948, -923, -893, -886, -788, -785, -768, -762, -754, -713, -681, -664, -661, -596, -594, -573, -572, -558, -543, -520, -515, -505, -499, -492, -491, -485, -450, -418, -411, -356, -346, -345, -343, -324, -321, -286, -235, -207, -201, -166, -156, -151, -131, -102, -81, -77, -49, -26, -24, -3, 5, 9, 15, 23, 29, 37, 72, 100, 106, 113, 157, 261, 271, 279, 280, 307, 336, 366, 414, 436, 528, 531, 557, 584, 609, 626, 659, 665, 684, 730, 737, 740, 761, 774, 813, 826, 852, 859, 860, 885, 894, 923, 940, 959, 961, 964, 966, 989];

    let length = values.len(); // O(1) OPERATION

    if length > 1{
        quicksort(&mut values, 0 , (length - 1) as isize);
    }


    let mut output = String::with_capacity(line.len());
    for value in values {
        output.push_str(&value.to_string());
        output.push_str(" ");
    }
    println!("{}", output);
}


pub fn insertionsort(arr:&mut Vec<i32>, start:isize, end:isize){
    for i in (start+1)..(end+1) {
        let cmp_value = arr[i as usize];
        let mut j:isize = i as isize - 1;

        while j >= start as isize && arr[j as usize] > cmp_value {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j+1) as usize] = cmp_value;
    }
}

pub fn median_of_three_partition(arr:&mut Vec<i32>, start:usize, end:usize) -> i32{
    let mid = ( start + end) / 2;

    if arr[start] <= arr[mid]{
        if arr[mid] > arr[end]{
            if arr[start] < arr[end]{
                return arr[end];
            }else{
                return arr[start];
            }
        }else{
            return arr[mid];
        }
    }else{
        if arr[mid] < arr[end]{
            if arr[start] < arr[end]{
                return arr[start];
            }else{
                return arr[end];
            }
        }else{
            return arr[mid];
        }
    }
}


pub fn quicksort(arr:&mut Vec<i32>, start: isize, end: isize){
        
    if start >= end{
        return
    }

        if end - start <= 30{
            insertionsort(arr, start, end);
            return;
        }

        let pivot = hoares_partition(arr, start, end);

        quicksort(arr, start, pivot);
        quicksort(arr, pivot + 1, end);
}


fn hoares_partition(values: &mut Vec<i32>, start: isize, end: isize) -> isize {
    let pivot = median_of_three_partition(values, start as usize, end as usize);
    let mut i = start - 1;
    let mut j = end + 1;

    loop {
        i += 1;
        while values[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while values[j as usize] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        values.swap(i as usize, j as usize);
    }
}



