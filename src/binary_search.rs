
pub fn binary_search<T: Ord>(list: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if key == list[mid] {
            return Some(mid);
        } else if key < list[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

pub fn binary_search_rec<T: Ord>(list: &[T], key: T, low: usize, high: usize) -> Option<usize> {
    let mid = (low + high) / 2;

    if key == list[mid] {
        Some(mid)
    } else if key < list[mid] {
        binary_search_rec(list, key, low, mid - 1)
    } else {
        binary_search_rec(list, key, mid + 1, high)
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn find_element_exists() {
        let arr = [1,2,3,4,5,6,7,8,9];
        assert_eq!(binary_search(&arr, 3), Some(2));
    }

    #[test]
    fn find_element_does_not_exist() {
        let arr = [1,2,3,4,5,6,7,8,9];
        assert_eq!(binary_search(&arr, 12), None);
    }
}
