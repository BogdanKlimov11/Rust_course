fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &num in arr.iter().skip(1) {
        if num > max {
            max = num;
        }
    }
    max
}

fn radix_sort(arr: &mut [i32]) {
    let mut max = find_max(arr);
    let mut exp = 1;

    while max / exp > 0 {
        let mut output = vec![0; arr.len()];
        let mut count = vec![0; 10];

        for &num in arr.iter() {
            count[((num / exp) % 10) as usize] += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for &num in arr.iter().rev() {
            let index = ((num / exp) % 10) as usize;
            output[count[index] - 1] = num;
            count[index] -= 1;
        }

        for i in 0..arr.len() {
            arr[i] = output[i];
        }

        exp *= 10;
    }
}
