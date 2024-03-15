fn counting_sort(arr: &mut [i32]) {
    let min_val = *arr.iter().min().unwrap();
    let max_val = *arr.iter().max().unwrap();

    let range = (max_val - min_val + 1) as usize;

    let mut count = vec![0; range];

    for &num in arr.iter() {
        let index = (num - min_val) as usize;
        count[index] += 1;
    }

    let mut i = 0;
    for (j, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[i] = (j as i32) + min_val;
            i += 1;
        }
    }
}
