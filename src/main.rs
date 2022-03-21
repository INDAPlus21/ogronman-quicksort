use std::io;
use std::io::prelude::*;
use std::cmp;
use std::mem;
use std::ptr;
use std::process;


pub const BLOCK_SIZE: usize = 128;


//Use radix sort, perhaps

//Pattern matching



//Har behövt kolla på så mycket C och C++, mina ögon blöder :(
fn main() {

    let mut line = String::with_capacity(500_000); // FIX ME!

    io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut values: Vec<i32> = Vec::with_capacity(line.len());
    values = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    //let mut values: Vec<i32> = vec![-927, 2333, 2323, 1912, -1068, 3208, -1487, -705, -2409, -743, -1206, 3023, -3736, -652, -449, 2099, -3587, 3037, -2326, 817, -1616, -1978, -2155, -1278, -4061, -211, 3154, -4419, 2167, -1526, -1700, -1755, -186, -2027, 1863, 875, -2045, -2405, 2984, 2118, -4585, 1303, 3578, -351, -1451, -718, 3115, -3188, -1036, -1186];
    //let mut values2: Vec<i32> = vec![-927, 2333, 2323, 1912, -1068, 3208, -1487, -705, -2409, -743, -1206, 3023, -3736, -652, -449, 2099, -3587, 3037, -2326, 817, -1616, -1978, -2155, -1278, -4061, -211, 3154, -4419, 2167, -1526, -1700, -1755, -186, -2027, 1863, 875, -2045, -2405, 2984, 2118, -4585, 1303, 3578, -351, -1451, -718, 3115, -3188, -1036, -1186];

    let length = values.len();

    if  length > 1{
        block_quicksort(&mut values, log_2(length-1));
        //insertionsort(&mut values);
    }
    
    //assert_eq!(values,values2, "The thing failed");

    //let mut output = String::with_capacity(line.len());
    line = "".to_string();
    for value in values {
        line.push_str(&value.to_string());
        line.push_str(" ");
    }
    println!("{}", line);
}


pub fn median_of_three_partition(arr:&mut [i32]) -> usize{
    let start = 0;
    let end = arr.len()-1;
    let mid = ( start + end) / 2;

    if arr[start] <= arr[mid]{
        if arr[mid] > arr[end]{
            if arr[start] < arr[end]{
                return end;
            }else{
                return start;
            }
        }else{
            return mid;
        }
    }else{
        if arr[mid] < arr[end]{
            if arr[start] < arr[end]{
                return start;
            }else{
                return end;
            }
        }else{
            return mid;
        }
    }
}

pub fn insertionsort(arr:&mut [i32]){

    for i in 1..arr.len() {
        let cmp_value = arr[i as usize];
        let mut j:isize = i as isize - 1;

        while j >= 0 as isize && arr[j as usize] > cmp_value {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j+1) as usize] = cmp_value;
    }
}


//TODO make a pick_pivot function that picks the pivot in a good way, maybe that will speed things up a bit

pub fn block_quicksort(arr:&mut Vec<i32>, limit: u32){

    if arr.len() > 0{
        block_rec(&mut arr[..], None ,limit);
    }

}


	//Implementation based on Tuned Quicksort (Elmasry, Katajainen, Stenmark)
    // And rust-lang (which copied this paper) version of quicksort
	//available at http://www.diku.dk/~jyrki/Myris/Kat2014S.html

fn block_rec<'a>(mut arr: &'a mut [i32], mut pred: Option<&'a i32>,mut limit: u32){
    const INS_MAX: usize = 32;

    let mut was_balanced = true;
    let mut was_partitioned = true;

    loop {

        let len = arr.len();

        if len <= INS_MAX{
            insertionsort(arr);
            return;
        }

        if limit == 0{
            heapsort(arr);
            return;
        }

        if !was_balanced{
            break_patterns(arr);
            limit -= 1;
        }

        let pivot = median_of_three_partition(arr);


        if let Some(p) = pred {
            if !is_less(p, &arr[pivot]){
                let mid = part_equal(arr, pivot);

                arr = &mut arr[mid..];
                continue;
            }
        }

        let mid = partition(arr, pivot);

        was_balanced = cmp::min(mid, len - mid) >= len / 8;

        let (left, right) = arr.split_at_mut(mid);
        let (pivot, right) = right.split_at_mut(1);
        let pivot = &pivot[0];

        if left.len() < right.len(){
            block_rec(left, pred,limit);
            arr = right;
            pred = Some(pivot);
        } else{
            block_rec(right, Some(pivot), limit);
            arr = left;
        }

    }

}

//Does not decrease time :/
fn break_patterns(arr: &mut [i32]){
    let len = arr.len();

    if len >= 8{

        //Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
        //Not mine
        let mut random = len as u32;
        let mut gen_u32 = || {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            random
        };
        let mut gen_usize = || {
            if 64 <= 32 {
                gen_u32() as usize
            } else {
                (((gen_u32() as u64) << 32) | (gen_u32() as u64)) as usize
            }
        };

        let modo = len.next_power_of_two();

        let pos  = len / 4 * 2;

        for i in 0..3 {
            let mut other = gen_usize() & (modo - 1);

            if other >= len {
                other -= len;
            }

            arr.swap(pos - 1 + i, other);

        }

    }
}


//Tried to follow / implement this paper of block-quicksort https://dl.acm.org/doi/10.1145/3274660 
//Also got "some" inspiration from the rust-lang quicksort (unstable-sort)
fn partition(arr: &mut [i32], pivot:usize) -> usize{
    let mid:usize = {
        arr.swap(0, pivot);
        let (pivot, arr) = arr.split_at_mut(1);
        let pivot = &mut pivot[0];

        //Gör kanske något fancy med pointers??

        let tmp = mem::ManuallyDrop::new(unsafe{ptr::read(pivot)});
        //let _pivot_guard = CopyOnDrop{src:&*tmp, dest: pivot};
        let pivot = &*tmp;

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

        l + part_block(&mut arr[l..r], pivot)

    };

    arr.swap(0, mid);

    mid

}

fn part_equal(arr: &mut [i32], pivot: usize) -> usize{

    arr.swap(0, pivot);
    let (pivot, arr) = arr.split_at_mut(1);
    let pivot = &mut pivot[0];

    let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
    //let _pivot_guard = CopyOnDrop { src: &*tmp, dest: pivot };
    let pivot = &*tmp;

    let mut l = 0;
    let mut r = arr.len();

    loop{
        unsafe{
            while l < r && !is_less(pivot, arr.get_unchecked(l)){
                l += 1;
            }

            while l < r && is_less(pivot, arr.get_unchecked(r-1)){
                r -= 1;
            }

            if l >= r{
                break;
            }

            r -= 1;

            arr.swap(l, r);
            l += 1;
        }
    }

    l + 1

}

fn is_less(l: &i32, r: &i32) -> bool{
    if l < r{
        true
    }else{
        false
    }
}

//Right now literally just the parition from rust-langs unstable_sort

// TODO instead of using fucky pointers, use fucky indexes
// You know that everyelement is an i32
// And so on so get to work you lazy piece

fn part_block(arr: &mut [i32], pivot: &i32) -> usize{
    const BLOCK: usize = 128;

    let mut l = 0;
    let mut block_l = BLOCK;
    let mut start_l = 0;
    let mut end_l = 0;
    let mut offsets_l = [0; BLOCK];

    let mut r = arr.len();
    let mut block_r = BLOCK;
    let mut start_r = 0;
    let mut end_r = 0;
    let mut offsets_r = [0; BLOCK];

    loop {
        let is_done = r-l <= 2 * BLOCK;

        if is_done {
            let mut rem = r - l;
            if start_l < end_l || start_r < end_r {
                rem -= BLOCK;
            }

            if start_l < end_l {
                block_r = rem;
            } else if start_r < end_r {
                block_l = rem;
            } else {
                block_l = rem / 2;
                block_r = rem - block_l;
            }
        }

        if start_l == end_l {
            start_l = 0; //Index of offsets_l
            end_l = start_l; //Index of end of offsets_l
            let mut elem = l;

            for i in 0..block_l {
                    offsets_l[end_l] = i as usize;
                    end_l += !is_less(&arr[elem], pivot) as usize;
                    elem += 1;
            }
        }

        if start_r == end_r {
            start_r = 0; //Index of start of offsets_r
            end_r = start_r;
            let mut elem = r;

            for i in 0..block_r {
                    elem -= 1;
                    offsets_r[end_r] = i as usize;
                    end_r += is_less(&arr[elem], pivot) as usize;
            }
        }

        let count = cmp::min(end_l - start_l, end_r - start_r);

        if count > 0 {
            macro_rules! left {
                () => {
                    l + offsets_l[start_l]
                };
            }
            macro_rules! right {
                () => {
                    r - (offsets_r[start_r]) - 1
                };
            }
                let tmp = arr[left!() as usize];

                //Kopiera ett element från offsets_r[start_r] till offsets_l[]
                //Vet dock inte exakt hur just den här delen fungerar
                //ptr::copy_nonoverlapping(right!(), left!(), 1);

                //Rätt säker att det är den här delen av koden som inte fungerar ://

                arr[left!()] = arr[right!()];

                for _ in 1..count {
                    start_l += 1;
                    //ptr::copy_nonoverlapping(left!(), right!(), 1);
                    arr[right!()] = arr[left!()];
                    start_r += 1;
                    //ptr::copy_nonoverlapping(right!(), left!(), 1);
                    arr[left!()] = arr[right!()];
                }

                //ptr::copy_nonoverlapping(&tmp, right!(), 1);
                //offsets_r.push(tmp);
                arr[right!()] = tmp;
                //mem::forget(tmp);
                start_l += 1;
                start_r += 1;
        }

        if start_l == end_l {
            l += block_l as usize;
        }

        if start_r == end_r {
            r -= block_r as usize;
        }

        if is_done {
            break;
        }
    }


    if start_l < end_l {
        while start_l < end_l {
                end_l -= 1;
                arr.swap(l + (offsets_l[end_l]), r - 1);
                r -= 1;
        }
        //width(v.as_mut_ptr(), r)
        //Borde då vara första r - first index = r - 0 = r
        r
    } else if start_r < end_r {
        while start_r < end_r {
                end_r -= 1;
                arr.swap(l, r - (offsets_r[end_r]) - 1);
                l += 1;
        }
        //width(v.as_mut_ptr(), l)
        //Samma som med r
        l
    } else {
        //width(v.as_mut_ptr(), l)
        l
    }

}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: usize) -> u32 {
    num_bits::<usize>() as u32 - x.leading_zeros() - 1
}

fn heapify(arr: &mut [i32], end:usize){
    let last_parent = end - 2 / 2;
    for i in (0..=last_parent).rev(){
        move_down(arr, i);
    }

}

pub fn heapsort(arr: &mut [i32]){
    let end = arr.len();
    if end <= 1 {
        return;
    }
    heapify(arr, end);

    for i in (1..end).rev(){
        arr.swap(0, i);
        move_down(&mut arr[..i], 0);
    }
}

fn move_down(arr: &mut [i32], mut root:usize){

    let last = arr.len() - 1;

    loop{
        let left = 2*root + 1;
        if left > last{
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left]{
            right
        }else{
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;

    }
}