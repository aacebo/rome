#[macro_export]
macro_rules! set {
    ($target:expr => $(/$path:tt)* = $value:expr) => {{
        let out = $crate::get_mut!($target => $(/$path)*);
        *out = $crate::value_of!($value);
        out
    }};
}

#[cfg(test)]
mod test {
    use crate::value_of;

    #[test]
    pub fn basic() {
        let mut meta = crate::MetaData::from([
            ("a", value_of!(21)),
            ("b", value_of!(true)),
            ("c", value_of!([3, 2, 1])),
        ]);

        let out = set!(meta => /c/1 = 55);
        assert_eq!(out, &value_of!(55));
    }
}
