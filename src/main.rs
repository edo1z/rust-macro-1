mod my_macros;

fn main() {
    hello!();
    println!("{}", double!(123));
    hoge!(pc, GET);
    pc();
    hoge!(sp, POST);
    sp();
}
