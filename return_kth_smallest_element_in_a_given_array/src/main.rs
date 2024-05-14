fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high]; 
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn kth_smallest_element(arr: &mut [i32], k: usize) -> Option<i32> {
    if k >= arr.len() {
        return None; 
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    loop {
        let pivot_index = partition(arr, low, high);

        if pivot_index == k {
            return Some(arr[k]); 
        } else if pivot_index < k {
            low = pivot_index + 1; 
        } else {
            high = pivot_index - 1; 
        }
    }
}

fn main() {
    let mut arr = [12, 3, 5, 7, 19];
    let k = 2;

    if let Some(kth_smallest) = kth_smallest_element(&mut arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Array is too small to find the {}th smallest element.", k);
    }
}
