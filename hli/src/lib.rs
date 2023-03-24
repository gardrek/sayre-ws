pub use vfc;

pub mod fc;
pub mod file;
pub mod random;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
