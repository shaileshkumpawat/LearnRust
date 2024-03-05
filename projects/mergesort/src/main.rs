pub fn mergesort<T: Copy + Ord>(A: &mut [T]) {
    // Sort A
    let a: usize = 0;
    let b: usize = A.len();
    if b > 1 {
        let c: usize = (a+b+1)/2;
        mergesort(&mut A[a..c]);
        mergesort(&mut A[c..b]);
        let l = &A[a..c].to_vec();
        let r = &A[c..b].to_vec();
        let mut i: usize = l.len();
        let mut j: usize = r.len();
        merge(A, l, r, &mut i, &mut j);
    }
}

pub fn merge<T: Copy + Ord>(A: &mut [T], l: &[T], r: &[T], i: &mut usize, j: &mut usize) {
    // merge sorted L & R into A
    let a: usize = 0;
    let b: usize = A.len();
    if b > 0 {
        if (*j <= 0) || (*i > 0 && l[(*i-1) as usize] > r[*j-1]) {
            A[b-1] = l[*i-1];
            *i = *i-1;
        }
        else {
            A[b-1] = r[*j-1];
            *j = *j-1;
        }
        merge(&mut A[..(b-1)], l, r, i, j);
    }
}

fn main() {
    let mut arr = [8, 3, 5, 9, 2];
    println!("Before sort: {:?}", arr);
    // selection_sort(&mut arr, None);
    mergesort(&mut arr);
    println!("After sort: {:?}", arr);
    // println!("Hello, world!");
}
