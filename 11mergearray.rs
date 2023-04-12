fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    if i < a.len() {
        merged.extend_from_slice(&a[i..]);
    }
    if j < b.len() {
        merged.extend_from_slice(&b[j..]);
    }
    return merged;
}

fn main() {
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];
    let merged = merge_sorted_arrays(&a, &b);
    println!("{:?}", merged);
}
