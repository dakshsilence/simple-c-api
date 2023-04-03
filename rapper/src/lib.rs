mod bindings;
mod tests {
    use crate::bindings::{self, add};

    #[test]
    fn it_works() {
        let result = unsafe { add(2, 2) };
        assert_eq!(result, 4);

        let rand = unsafe { bindings::get_random() };
    }
}
