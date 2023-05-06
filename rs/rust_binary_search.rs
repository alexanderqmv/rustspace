#![allow(dead_code)]
#![allow(unused)]

fn bin_search(arr: &mut Vec<usize>, key: usize) -> (usize, bool)
{
    arr.sort();
    let mut left: usize  = 0;
    let mut right: usize = arr.len() - 1;
    while left <= right {
        let mut mid: usize = left + (right - left) / 2;
        if arr[mid] == key {
            return (mid, true);
        }
        if arr[mid] < key {
            left = mid + 1;
        }
        if arr[mid] > key {
            right = mid - 1;
        }
    }
    return (0, false);
}


fn main()
{
    let mut arr= vec![1,5,4,2,0,9,11];
    let get = bin_search(&mut arr, 5);
    println!("Result: {}:{}",get.0, get.1);
}
