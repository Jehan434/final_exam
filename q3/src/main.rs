extern crate rand;

use rand::thread_rng;
use rand::Rng;
fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let  l1 = arr.clone();
    let  r1 = arr.clone();
    let l = &l1[left..mid];
    let r = &r1[mid..right];
    /* Merge the temp arrays back into arr[l..r]*/
    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray
    while i < n1 && j < n2 {
        if l[i] < r[j] {
            arr[k] = l[i];
            i = i + 1;
        } else {
            arr[k] = r[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = l[i];
        i = i + 1;
        k = k + 1;
    }
    /* Copy the remaining elements of R[], if there
    are any */
    while j < n2 {
        arr[k] = r[j];
        j = j + 1;
        k = k + 1;
    }
    arr
}
//fn randomarr(size: u32) -> Vec<i32> {
//     let  randarr: Vec<i32> = Vec::with_capacity(size as usize);
//         let mut  randarr = rand::thread_rng();
//         let  randarr: Vec<i32> = vec![ randarr.gen_range(0..999)];
//     return randarr;
// }


fn main() {
  // let mut arr: Vec<i32> = vec![64, 34, 25, 8, 22, 11, 9];
   // let mut arr = rand::thread_rng();
    //let mut arr: Vec<i32> = vec![ arr.gen_range(0..999)];

    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(1, 10000);
    let mut arr = [0i32; 1000];
    for x in &mut arr {
        *x = rng.sample(distr);
    }

    arr = <[i32; 1000]>::try_from(merge_sort(Vec::from(arr.clone()), 0, arr.len())).unwrap();
    println!("Sorted array is {:?}", arr);

}