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

    let mut values: Vec<i32> = vec![-927, 2333, 2323, 1912, -1068, 3208, -1487, -705, -2409, -743, -1206, 3023, -3736, -652, -449, 2099, -3587, 3037, -2326, 817, -1616, -1978, -2155, -1278, -4061, -211, 3154, -4419, 2167, -1526, -1700, -1755, -186, -2027, 1863, 875, -2045, -2405, 2984, 2118, -4585, 1303, 3578, -351, -1451, -718, 3115, -3188, -1036, -1186];
    let mut values2: Vec<i32> = vec![-927, 2333, 2323, 1912, -1068, 3208, -1487, -705, -2409, -743, -1206, 3023, -3736, -652, -449, 2099, -3587, 3037, -2326, 817, -1616, -1978, -2155, -1278, -4061, -211, 3154, -4419, 2167, -1526, -1700, -1755, -186, -2027, 1863, 875, -2045, -2405, 2984, 2118, -4585, 1303, 3578, -351, -1451, -718, 3115, -3188, -1036, -1186];

    let length = values.len();

    if  length > 1{
        //quicksort(&mut values, 0, length - 1);
        block_quicksort(&mut values);
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
    //let pivot = block_partition(arr, start, end);

    //quicksort(arr, start, pivot);
    //quicksort(arr, pivot + 1, end);
}

pub fn block_quicksort(arr:&mut Vec<i32>){

    if arr.len() > 0{
        let limit = usize::BITS - arr.len().leading_zeros();
        block_rec(&mut arr[..], None, 0);
    }

}

fn block_rec<'a>(mut arr: &'a mut [i32], mut pred: Option<&'a i32>, mut limit: u32){
    const ins_max: usize = 20;

    let mut was_balanced = true;
    let mut was_partitioned = true;

    loop {

        println!("main loop arr is {:#?}", arr);

        let len = arr.len();

        if len <= ins_max{
            insertionsort(arr, 0, len - 1);
            return;
        }

        let pivot = arr.len() - 1;
        let mid = partition(arr, pivot);

        let (left, right) = arr.split_at_mut(mid);
        let (pivot, right) = right.split_at_mut(1);
        let pivot = &pivot[0];

        if left.len() < right.len(){
            block_rec(left, pred, 0);
            arr = right;
            pred = Some(pivot);
        } else{
            block_rec(right, Some(pivot), 0);
            arr = left;
        }

    }

}

fn partition(mut arr: &mut [i32], pivot:usize) -> usize{
    let mid:usize = {
        arr.swap(0, pivot);
        let (pivot, arr) = arr.split_at_mut(1);
        let pivot = &mut pivot[0];

        //Gör kanske något fancy med pointers??

        let mut l = 0;
        let mut r = arr.len();

        unsafe{
            while l < r && arr.get_unchecked(l) < pivot{
                l += 1;
            }

            while l < r && arr.get_unchecked(r - 1) >= pivot{
                r -= 1;
            }
        }

        l + block_partition(&mut arr[l..r])

    };

    arr.swap(0, mid);

    mid

}



pub fn block_partition(mut arr:&mut [i32]) -> usize{

    println!("{:#?}", arr);

    let mut len = arr.len();

    if len <= 1{
        return 0;
    }

    let mut start = 0;
    let mut end = len;

    let mut pivot_pos = end - 1;
    let mut pivot = arr[pivot_pos];

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
            let mut j = 0;
            while j < block_size{
                index_left[num_left] = j;
                num_left += (arr[j] >= pivot) as usize;
                j += 1;
            }
        }
        if num_right == 0{
            start_right = 0;
            let mut j = 0;
            while j < block_size{
                index_right[num_right] = j;
                num_right += (pivot >= arr[last-j]) as usize;
                j += 1;
            }
        }
        num = cmp::min(num_left, num_right);
        
        let mut j = 0;
        while j < num{
            arr.swap(start + index_left[start_left + j], last - index_right[start_right + j]);
            j += 1;
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
        let mut j = 0;
        while j < shift_left{
            index_left[num_left] = j;
            num_left += (arr[j] >= pivot) as usize;

            index_right[num_right] = j;
            num_right += (pivot >= arr[last-j]) as usize;
            j += 1;
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
        
        let mut j = 0;
        while j < shift_left{
            index_left[num_left] = j;
            num_left += (arr[j] >= pivot) as usize;
            j += 1;
        }

    }else{
        shift_left = block_size;
        shift_right = (last - start) - block_size + 1;

        start_right = 0;

        let mut j = 0;
        while j < shift_right{
            index_right[num_right] = j;
            num_right += (pivot >= arr[last-j]) as usize;
            j += 1;
        }

    }

    num = cmp::min(num_left, num_right);

    if num > 0{
        num -= 1;
    }

    let mut j = 0;
    while j < num{
        arr.swap(start + index_left[(start_left + j)], last - index_right[(start_right + j)]);
        j += 1;
    }

    num_left -= num;
    num_right -= num;

    start_left += num;
    start_right += num;
    if num_left == 0 {start += shift_left}
    if num_right == 0 {last -= shift_right}

    if num_left != 0{
        //TODO get it to work, change everything so lower_i can be isize
        let mut lower_i = (start_left + num_left - 1);
        let mut upper = (last - start);

        while lower_i > start_left && index_left[lower_i as usize] == upper{
            upper -= 1;
            lower_i -= 1;
        }
        while lower_i > start_left{
            upper -= 1;
            lower_i -= 1;
            arr.swap((start + upper) as usize, start + index_left[lower_i as usize]);
        }

        let swap_in = (start + upper + 1) as usize;
        arr.swap(pivot_pos, swap_in);
        return swap_in;

    }else if num_right != 0{
        let mut lower_i = (start_right + num_right - 1);
        let mut upper = (last - start);
        
        while lower_i > start_right && index_right[lower_i as usize] == upper{
            upper -= 1;
            lower_i -= 1;
        }

        while lower_i > start_right {
            upper -= 1;
            lower_i -= 1;
            //This thing apparently does it so the index goes to 100, but len is 100 so kinda bad..
            arr.swap((last - upper) as usize, last - index_right[lower_i as usize]);
        }
        let swap_in = (last - upper) as usize;
        arr.swap(pivot_pos, swap_in);
        return swap_in;

    } else{
        arr.swap(pivot_pos, start);
        return start;
    }


}

