fn kth_smallest_element(nums: &[i32], k: usize) -> Option<i32> {
    if k > nums.len() {
        return None;
    }
    let mut nums = nums.to_vec();
    nums.sort();
    Some(nums[k - 1])
}

fn main() {
    let nums = [3, 7, 2, 8, 4, 5];
    let k = 3;
    match kth_smallest_element(&nums, k) {
        Some(x) => println!("The {}th smallest element in {:?} is {}", k, nums, x),
        None => println!("Invalid value of k")
    }
}
