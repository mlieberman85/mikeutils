use std::io;
use std::io::prelude::*;

const MIN_MERGE: usize = 32;
const MIN_GALLOP: usize = 7;
const INITIAL_TMP_STORAGE_LENGTH: usize = 256;

fn tim_sort<T: Ord + Clone>(xs: &mut [T]) {
    let mut stack_size = 0;

}

fn insertion_sort<T: Ord>(xs: &mut[T]) {
    let length = xs.len();
    // Unclear to me right now whether or not I can just use the array slice
    // as an iterator.
    for i in 1..length {
        let mut j = i;
        while j > 0 && xs[j-1] > xs[j] {
            xs.swap(j, j-1);
            j -= 1;
        }
    }
}

fn get_ascending_run<T: Ord>(xs: &mut[T]) {

}

fn get_min_run(length: usize) -> usize {
    assert!(length >= 0);
    let mut run = 0;
    let mut n = 0;
    while n >= MIN_MERGE {
        run |= length & 1;
        n >>= 1;
    }
    return run + n;
}

fn merge_collapse() {
}

fn merge_at() {
}

fn gallop_left<T: Ord> (xs: &mut[T]) {

}

fn gallop_right<T: Ord> (xs: &mut[T]) {
}

fn merge_high<T: Ord> (xs: &mut[T]) {
}

fn merge_low<T: Ord> (xs: &mut[T]) {
}

/*fn get_min_run(length: u32) -> u32 {

}*/

fn main() {}
