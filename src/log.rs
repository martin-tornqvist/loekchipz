#[cfg(debug_assertions)]
macro_rules! log
{
    () =>
    {
        println!("({},{})", file!(), line!());
    };
    ($($arg:tt)+) =>
    {
        print!("(DEBUG: {}, {} --- ", file!(), line!());
        print!($($arg)+);
        println!(")");
    };
}

#[cfg(not(debug_assertions))]
macro_rules! log
{
    ($($arg:tt)*) =>
    {
    }
}
