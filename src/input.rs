use std::{fmt::Debug, io, str::FromStr};

pub fn input<T: FromStr>() -> Result<T, <T as FromStr>::Err>
where
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::default();
    io::stdin()
        .read_line(&mut buf)
        .expect("Problem reading stdin");

    let choice = buf.trim().parse::<T>();
    choice
}
