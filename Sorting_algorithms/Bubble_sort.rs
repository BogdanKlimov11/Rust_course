fn bubble_sort(arr: &mut Vec<i32>) {
    let mut n = arr.len();
    loop {
        let mut swapped = false;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        n -= 1;
    }
}
