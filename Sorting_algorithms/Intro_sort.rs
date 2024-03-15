fn intro_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let max_depth = (2 * (len as f64).log2()) as usize;
    introsort_recursive(arr, 0, len, max_depth);
}

fn introsort_recursive<T: Ord>(arr: &mut [T], start: usize, end: usize, depth_limit: usize) {
    if end - start <= 1 {
        return;
    } else if depth_limit == 0 {
        heap_sort(arr, start, end);
        return;
    } else {
        let pivot_index = partition(arr, start, end);
        introsort_recursive(arr, start, pivot_index, depth_limit - 1);
        introsort_recursive(arr, pivot_index + 1, end, depth_limit - 1);
    }
}

fn partition<T: Ord>(arr: &mut [T], start: usize, end: usize) -> usize {
    let pivot = &arr[end - 1];
    let mut i = start;
    for j in start..end - 1 {
        if &arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, end - 1);
    i
}

fn heap_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    heapify(arr, start, end);
    for i in (start..end).rev() {
        arr.swap(start, i);
        sift_down(arr, start, i);
    }
}

fn heapify<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut i = start + (end - start) / 2;
    while i > start {
        i -= 1;
        sift_down(arr, i, end);
    }
}

fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = 2 * root + 1;
        if child >= end {
            break;
        }
        if child + 1 < end && &arr[child] < &arr[child + 1] {
            child += 1;
        }
        if &arr[root] < &arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
