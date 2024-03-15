fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child + 1 <= end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

fn smooth_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut p = 1;
    let mut m = 1;
    while p < n {
        let lp = p;
        p += m * 3 + 1;
        m *= 3;
    }
    p = (p - 1) / 3;

    while p > 0 {
        let mut i = p;
        while i < n {
            sift_down(arr, i - p, i);
            i += 1;
        }
        p /= 3;
    }

    let mut i = n - 1;
    while i > 0 {
        arr.swap(0, i);
        sift_down(arr, 0, i - 1);
        i -= 1;
    }
}
