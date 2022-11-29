macro_rules! echo {
    () => {
        println!("无解");
    };
    // 前缀: 表达式
    ($exp: expr) => {
        println!("{}", stringify!($exp));
    };
    ($($exp: expr), *) => {
        $(
            println!("{}", stringify!($exp));
        )*
    };
}

macro_rules! func {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("my func name is {}", stringify!($fn_name));
        }
    };
}
