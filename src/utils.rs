///simplified version of dbg (does not show file/line, use display instead of debug, print the stringified block before executing it)
#[macro_export]
macro_rules! result {
    () => {
        println!()
    };
    ($val:expr $(,)?) => {
        {
            print!("{} = ", stringify!($val));
            let tmp = { $val };
            println!("{}", &tmp);
            tmp
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(result!($val)),+,)
    }
}

///like above, but does use the compact debug format
#[macro_export]
macro_rules! dresult {
    () => {
        println!()
    };
    ($val:expr $(,)?) => {
        {
            print!("{} = ", stringify!($val));
            let tmp = { $val };
            println!("{:?}", &tmp);
            tmp
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dresult!($val)),+,)
    }
}

///like result but even more simplified: show the expression (and execute it) without showing the result
#[macro_export]
macro_rules! show {
    ($val:expr $(,)?) => {
        {
            println!("{}", stringify!($val));
            $val
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(show!($val)),+,)
    };
}

pub fn ex(number: usize, title: &str) {
    let l = title.chars().count() as i32;
    let ll = ((63 - l) / 2).max(0);
    let lr = (63 - l - ll).max(0);
    println!("\n{} exercise {number:02}: '{title:.63}' {}", "*".repeat(ll as usize), "*".repeat(lr as usize));
}