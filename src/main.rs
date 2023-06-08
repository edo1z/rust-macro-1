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

fn main() {
    hello!();
    println!("{}", double!(123));
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