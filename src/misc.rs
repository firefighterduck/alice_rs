pub fn find_first<T, P>(vec: &[T], pred: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    if vec.is_empty() {
        return None;
    }

    let mut index: usize = 0;
    while !pred(vec.get(index).unwrap()) {
        index += 1;
        if index >= vec.len() {
            return None;
        }
    }
    Some(index)
}

pub fn find_and_remove<T, P>(vec: &mut Vec<T>, pred: P) -> Option<T>
where
    P: Fn(&T) -> bool,
{
    if let Some(index) = find_first(&vec, pred) {
        let elem = vec.swap_remove(index);
        Some(elem)
    } else {
        None
    }
}

#[test]
pub fn test_find_first() {
    let vec = vec![1, 2, 3, 3, 5];
    let pred = |x: &i32| *x == 3;
    let pred2 = |x: &i32| *x == 6;
    let pred3 = |x: &i32| *x == 5;

    assert_eq!(Some(2), find_first(&vec, pred));
    assert_eq!(None, find_first(&vec, pred2));
    assert_eq!(Some(4), find_first(&vec, pred3));
}

#[test]
pub fn test_find_and_remove() {
    let mut vec = vec![1, 2, 3, 3, 5];
    let pred = |x: &i32| *x == 3;

    assert_eq!(Some(3), find_and_remove(&mut vec, pred));
    assert_eq!(vec![1, 2, 5, 3], vec);

    let mut vec2 = vec![1, 2, 3, 3, 5];
    let pred2 = |x: &i32| *x == 6;
    assert_eq!(None, find_and_remove(&mut vec2, pred2));
    assert_eq!(vec![1, 2, 3, 3, 5], vec2);

    let mut vec3 = vec![1, 2, 3, 3, 5];
    let pred3 = |x: &i32| *x == 5;
    assert_eq!(Some(5), find_and_remove(&mut vec3, pred3));
    assert_eq!(vec![1, 2, 3, 3], vec3)
}
