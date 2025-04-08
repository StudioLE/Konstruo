pub trait HandleError<T, E> {
    fn handle_error<F: FnOnce(E)>(self, op: F) -> Option<T>;
}

impl<T, E> HandleError<T, E> for Result<T, E> {
    /// Converts from `Result<T, E>` to [`Option<T>`].
    ///
    /// Passes error to [`F`] and converts `self` into an [`Option<T>`], consuming `self`.
    fn handle_error<F: FnOnce(E)>(self, op: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                op(error);
                None
            }
        }
    }
}
