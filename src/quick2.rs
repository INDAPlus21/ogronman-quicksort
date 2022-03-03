use std::io;
use std::io::prelude::*;
use std::cmp;


//SIMD in rust, for extra snabb skit

//You complete the first 6 tests if you do not sort

//Fram till halva är den relativt sorterad...


//Får runtime error på tredje testet

pub const block_size: usize = 32;

fn main() {

    let mut line = String::with_capacity(500_000); // FIX ME!

    /*io::stdin().lock().read_to_string(&mut line);
    let mut values: Vec<i32> = Vec::with_capacity(line.len());
    values = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();*/

    let mut values: Vec<i32> = vec![-2766, -809, -3479, 526, 1298, 1875, 3023, -635, -488, 1149, 904, -3618, 4813, -2640, 2473, -4012, -4720, -3575, -2940, 2493, -4632, -2595, 2635, 898, 2585, 4059, 4588, 374, 179, -1170, -1058, 4192, -2742, 1751, -3836, 3583, -4132, -4376, 4590, 4423, -3430, 822, -3370, -2477, 4152, 3230, -1974, 2873, 2420, -2837, -3961, -124, 2587, -869, 4160, 2180, 4512, -1054, -3238, -84, 2259, -1827, -2556, 4672, -2313, -3116, 2235, -4442, -4474, 1423, -3985, -3156, -804, -3886, 1123, -1810, 3498, -4029, -1368, 2396, 4100, 2851, -905, -1728, 4346, -2552, -234, -436, 4419, 2910, -2464, 1000, 4925, 4442, -3634, 4368, -2060, 1914, 4170, -3566];
    let mut values2: Vec<i32> = vec![-2766, -809, -3479, 526, 1298, 1875, 3023, -635, -488, 1149, 904, -3618, 4813, -2640, 2473, -4012, -4720, -3575, -2940, 2493, -4632, -2595, 2635, 898, 2585, 4059, 4588, 374, 179, -1170, -1058, 4192, -2742, 1751, -3836, 3583, -4132, -4376, 4590, 4423, -3430, 822, -3370, -2477, 4152, 3230, -1974, 2873, 2420, -2837, -3961, -124, 2587, -869, 4160, 2180, 4512, -1054, -3238, -84, 2259, -1827, -2556, 4672, -2313, -3116, 2235, -4442, -4474, 1423, -3985, -3156, -804, -3886, 1123, -1810, 3498, -4029, -1368, 2396, 4100, 2851, -905, -1728, 4346, -2552, -234, -436, 4419, 2910, -2464, 1000, 4925, 4442, -3634, 4368, -2060, 1914, 4170, -3566];

    let length = values.len(); // O(1) OPERATION

    if  length > 1{
        quicksort(&mut values, 0, length - 1);
        insertionsort(&mut values2, 0, length - 1);
    }
    
    //assert_eq!(values,values2, "The thing failed");

    let mut output = String::with_capacity(line.len());
    for value in values {
        output.push_str(&value.to_string());
        output.push_str(" ");
    }
    println!("{}", output);
}


pub fn insertionsort(arr:&mut [i32], start:usize, end:usize){
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


pub fn quicksort(arr:&mut Vec<i32>,start:usize, end:usize){

    if start >= end{
        return
    }
        
    if end - start <= block_size{
        insertionsort(arr, start, end);
        return;
    }
    //median_of_three_partition(arr);
    let pivot = block_partition(arr, start, end);

    quicksort(arr, start, pivot);
    quicksort(arr, pivot + 1, end);
}



pub fn block_partition(arr:&mut Vec<i32>, start: usize, end:usize) -> usize{

    let mut len = arr.len();
    let mut start = start;

    let mut pivot_pos = end - 1;
    let mut pivot = arr[end];

    let mut index_left:[usize;block_size] = [0; block_size];
    let mut index_right:[usize;block_size] = [0; block_size];

    let mut last = end - 1;

    arr.swap(pivot_pos, last);

    pivot_pos = last;
    last -= 1;

    let mut start_left = 0;
    let mut start_right = 0;

    let mut num_left = 0;
    let mut num_right = 0;

    let mut num = 0;

    while last - start + 1 > 2 * block_size{
        if num_left == 0{
            start_left = 0;
            for j in 0..block_size - 1{
                index_left[num_left] = j;
                num_left += (arr[j] >= pivot) as usize;
            }
        }
        if num_right == 0{
            start_right = 0;
            for j in 0..block_size - 1{
                index_right[num_right] = j;
                num_right += (pivot >= arr[last-j]) as usize;
            }
        }
        num = cmp::min(num_left, num_right);
        for j in 0..num{
            arr.swap(start + index_left[start_left + j], last - index_right[start_right + j]);
        }

        num_left -= num;
        num_right -= num;

        start_left += num;
        start_right += num;
        if num_left == 0 {start += block_size}
        if num_right == 0 {last -= block_size}

    }

    let mut shift_right = 0;
    let mut shift_left = 0;

    if num_right == 0 && num_left == 0{
        shift_left = ((last - start ) + 1) / 2;
        shift_right = (last - start) + 1 - shift_left;

        start_left = 0;
        start_right = 0;

        for j in 0..shift_left{
            index_left[num_left] = j;
            num_left += (arr[j] >= pivot) as usize;

            index_right[num_right] = j;
            num_right += (pivot >= arr[last-j]) as usize;
        }

        if shift_left < shift_right{
            index_right[num_right] = (shift_right - 1);
            num_right += (pivot >= arr[last-shift_right+1]) as usize;
        }

    }
    else if num_right != 0{
        shift_left = (last - start) - block_size + 1;
        shift_right = block_size;
        start_left = 0;
        
        for j in 0..shift_left-1{
            index_left[num_left] = j;
            num_left += (arr[j] >= pivot) as usize;
        }

    }else{
        shift_left = block_size;
        shift_right = (last - start) - block_size + 1;

        start_right = 0;

        for j in 0..shift_right-1{
            index_right[num_right] = j;
            num_right += (pivot >= arr[last-j]) as usize;
        }

    }

    num = cmp::min(num_left, num_right);


    for j in 0..num{
        arr.swap(start + index_left[(start_left + j)], last - index_right[(start_right + j)]);
    }

    num_left -= num;
    num_right -= num;

    start_left += num;
    start_right += num;
    if num_left == 0 {start += shift_left}
    if num_right == 0 {last -= shift_right}

    if num_left != 0{
        //TODO get it to work, change everything so lower_i can be isize
        let mut lower_i:isize = (start_left + num_left - 1) as isize;
        let mut upper:isize = (last - start) as isize;

        while lower_i > start_left as isize && index_left[lower_i as usize] as isize == upper{
            upper -= 1;
            lower_i -= 1;
        }
        while lower_i > start_left as isize{
            upper -= 1;
            lower_i -= 1;
            arr.swap((start as isize + upper) as usize, start + index_left[lower_i as usize]);
        }

        let swap_in = (start as isize + upper + 1) as usize;
        arr.swap(pivot_pos, swap_in);
        return swap_in;

    }else if num_right != 0{
        let mut lower_i:isize = (start_right + num_right - 1) as isize;
        let mut upper:isize = (last - start) as isize;
        
        while lower_i > start_right as isize && index_right[lower_i as usize] as isize == upper{
            upper -= 1;
            lower_i -= 1;
        }

        while lower_i > start_right as isize{
            upper -= 1;
            lower_i -= 1;
            //This thing apparently does it so the index goes to 100, but len is 100 so kinda bad..
            arr.swap((last as isize - upper) as usize, last - index_right[lower_i as usize]);
        }
        let swap_in = (last as isize - upper) as usize;
        arr.swap(pivot_pos, swap_in);
        return swap_in;

    } else{
        arr.swap(pivot_pos, start);
        return start;
    }


}

