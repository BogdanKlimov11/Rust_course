fn timsort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let min_merge = calculate_min_run(n);

    let mut i = 0;
    while i < n {
        let mut j = std::cmp::min(i + min_merge, n);
        insertion_sort(&mut arr[i..j]);
        i += min_merge;
    }

    let mut size = min_merge;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = std::cmp::min(left + size, n);
            let right = std::cmp::min(left + 2 * size, n);
            if mid < right {
                merge(&mut arr[left..mid], &mut arr[mid..right]);
            }
            left += 2 * size;
        }
        size *= 2;
    }
}

fn calculate_min_run(n: usize) -> usize {
    let mut r = 0;
    let mut n = n;
    while n >= 64 {
        r |= n & 1;
        n >>= 1;
    }
    return n + r;
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn merge<T: Ord>(left: &mut [T], right: &mut [T]) {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(std::mem::replace(&mut left[i], left[i]));
            i += 1;
        } else {
            result.push(std::mem::replace(&mut right[j], right[j]));
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    for (i, value) in result.into_iter().enumerate() {
        left[i] = value;
    }
}
