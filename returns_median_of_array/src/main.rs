fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 1 {
        
        return nums[len / 2] as f64;
    } else {
        
        let middle_right = len / 2;
        let middle_left = middle_right - 1;
        return (nums[middle_left] as f64 + nums[middle_right] as f64) / 2.0;
    }
}

fn main() {
    let sorted_array1 = vec![1, 2, 3, 4, 5];
    let sorted_array2 = vec![1, 2, 3, 4, 5, 6];

    let median1 = find_median(&sorted_array1);
    let median2 = find_median(&sorted_array2);

    println!("Median of {:?} first array is {}", sorted_array1, median1);
    println!("Median of {:?} is second array is {}", sorted_array2, median2);
}
