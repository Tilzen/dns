use std::{error, result};

type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;
