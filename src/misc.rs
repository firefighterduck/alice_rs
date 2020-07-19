pub fn find_first<T, P>(vec: &Vec<T>, pred: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    let mut index: usize = 0;
    while !pred(vec.get(index).unwrap()) {
        index += 1;
        if index >= vec.len() {
            return None;
        }
    }
    Some(index)
}

#[test]
pub fn test_find_first() {
    let vec = vec![1, 2, 3, 3, 5];
    let pred = |x: &i32| *x == 3;
    let pred2 = |x: &i32| *x == 6;
    assert_eq!(Some(2), find_first(&vec, pred));
    assert_eq!(None, find_first(&vec, pred2));
}
