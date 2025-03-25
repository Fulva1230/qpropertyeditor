#[cfg(test)]
mod test{
    #[test]
    fn test_closure() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        fn f<F>(g: F) where F: FnOnce() -> () {
            g();
        }
        let borrows_mutably  = || list.push(7);
        f(borrows_mutably);

        println!("After calling closure: {list:?}");
    }
}

