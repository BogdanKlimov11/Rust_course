fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let n1 = mid - left + 1;
    let n2 = right - mid;

    let mut L = vec![0; n1];
    let mut R = vec![0; n2];

    for i in 0..n1 {
        L[i] = arr[left + i];
    }
    for j in 0..n2 {
        R[j] = arr[mid + 1 + j];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < n1 && j < n2 {
        if L[i] <= R[j] {
            arr[k] = L[i];
            i += 1;
        } else {
            arr[k] = R[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        arr[k] = L[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k] = R[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut [i32], left: usize, right: usize) {
    if left < right {
        let mid = (left + right) / 2;
        merge_sort(arr, left, mid);
        merge_sort(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}
