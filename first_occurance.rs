fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}
fn main() {
    let arr = [1, 2, 3, 3, 4, 5];
    let target = 3;
    let result = first_occurrence(&arr, target);
    println!("The first occurrence of {} in {:?} is at index {:?}", target, arr, result);
}
