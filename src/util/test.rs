#[macro_export]
macro_rules! assert_all_elems_eq {
    ($arr:expr, $val:expr) => {
        assert_eq!(
            $arr.iter().all(|elem| *elem == $val),
            true
        );
    };
}

#[macro_export]
macro_rules! flatten {
    ($arr:expr) => {
        $arr.iter().flatten().copied().collect::<Vec<_>>()
    };
}
