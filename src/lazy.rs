// Inspired by: https://github.com/reem/rust-lazy/

pub enum Thunk<'a, T: 'a> {
    Todo { thunk: &'a FnOnce() -> T },
    Done { value: T },
}

impl<'a, T: 'a> Thunk<'a, T> {
    pub fn new(t: &'a FnOnce() -> T) -> Self {
        Thunk::Todo { thunk: t }
    }

    pub fn eval(&mut self) -> &T {
        let value = match self {
            &mut Thunk::Done { ref value } => value,
            &mut Thunk::Todo { ref thunk } => &thunk()
        };
        value
    }
}

#[macro_export]
macro_rules! lazy {
    ($e:expr) => { Thunk::new(move || { $e }) }
}
