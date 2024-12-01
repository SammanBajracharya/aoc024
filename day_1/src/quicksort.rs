pub fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

pub fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut test_arr_1 = [3, 4, 6, 1, 7, 4];
        let mut test_arr_2 = [44, 53, 76, 47, 24, 35, 70];

        quicksort(&mut test_arr_1);
        quicksort(&mut test_arr_2);

        assert_eq!(test_arr_1, [1, 3, 4, 4, 6, 7]);
        assert_eq!(test_arr_2, [24, 35, 44, 47, 53, 70, 76]);
    }
}

