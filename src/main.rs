macro_rules! hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! double {
    ($val:expr) => {
        $val * 2
    };
}

macro_rules! hoge {
    ($name:ident,GET) => {
        hoge!(
            /// Prints out a formatted string containing the function name and method.
            ///
            /// This function generates a formatted string from the function name and method
            /// passed as identifiers, and then prints this string to the console using `println!`.
            ///
            /// # Examples
            ///
            /// ```rust
            /// pub fn my_function() {
            ///     println!("{}!! {}!!!", stringify!(my_function), stringify!(GET));
            /// }
            /// ```
            ///
            /// This will print: "my_function!! GET!!!" to the console.
            $name,
            GET
        );
    };

    ($name:ident,$method:ident) => {
        hoge!(
            #[doc = concat!("Route `", stringify!($method) ,"` requests to the given handler.")]
            $name,
            $method
        );
    };

    (
        $(#[$attr:meta])+
        $name:ident, $method:ident
    ) => {
        $(#[$attr])+
        pub fn $name() {
            println!("{}!! {}!!!", stringify!($name), stringify!($method));
        }
    };
}

fn main() {
    hello!();
    println!("{}", double!(123));
    hoge!(pc, GET);
    pc();
    hoge!(sp, POST);
    sp();
}

#[cfg(test)]
mod tests {
    fn add_two(n: i32) -> i32 {
        n + 2
    }

    #[test]
    fn test_double() {
        // リテラル
        assert_eq!(double!(5), 10);
        assert_eq!(double!(0), 0);
        assert_eq!(double!(100), 200);
        assert_eq!(double!(-5), -10);

        // 変数参照
        let x = 5;
        assert_eq!(double!(x), 10);

        // 算術演算
        assert_eq!(double!(3 + 2), 10);

        // if式
        assert_eq!(double!(if x > 5 { x } else { 2 }), 4);

        // match式
        assert_eq!(double!(match x {
            5 => 2,
            _ => 0,
        }), 4);

        // 関数呼び出し
        assert_eq!(double!(add_two(3)), 10);

        // メソッド呼び出し
        let v = vec![1, 2, 3];
        assert_eq!(double!(v.len()), 6);

        // クロージャ
        let closure = |n: i32| n + 2;
        assert_eq!(double!(closure(3)), 10);
    }
}