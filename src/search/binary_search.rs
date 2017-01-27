#[allow(dead_code)]
pub fn search(target: i32, list: &[i32]) -> i32 {
    let mut start = 0;
    let mut end = list.len() - 1;
    let mut index = 0;

    while start <= end {
        index = index + 1;
        let middle = (start + end) / 2;
        let current = list[middle];
        if target == current {
            return middle as i32
        }
        if current < target {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }
    -1
}

#[cfg(test)]
#[test]
fn it_works() {
    assert_eq!(search(1, &[1,2,3]), 0);
    assert_eq!(search(2, &[1,2,3]), 1);
    assert_eq!(search(3, &[1,2,3]), 2);
    assert_eq!(search(4, &[1,2,3]), -1);
}

