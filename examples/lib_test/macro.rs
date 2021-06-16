extern crate rider;

use rider::libs::state::state::Status;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Hello<'a> {
    msg: &'a str,
}

pub trait Summary {
    fn summarize_author(&self) -> &str;

    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Hello<'_> {
    fn summarize_author(&self) -> &str {
        return self.msg;
    }
}

macro_rules! mix_struct {
    () => {};
}

// macro_rules! write_html {
//     ($w:expr, ) => (());

//     ($w:expr, $e:tt) => (write!($w, "{}", $e));

//     ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
//         write!($w, "<{}>", stringify!($tag));
//         write_html!($w, $($inner)*);
//         write!($w, "</{}>", stringify!($tag));
//         write_html!($w, $($rest)*);
//     }};
// }

fn main() {
    let hello = Hello { msg: "hello world" };
    println!("{:?}", hello.summarize());

    // use std::fmt::Write;
    // let mut out = String::new();

    // write_html!(&mut out,
    // html[
    //     head[title["Macros guide"]]
    //     body[h1["Macros are the best!"]]
    // ]);

    // assert_eq!(
    //     out,
    //     "<html><head><title>Macros guide</title></head>\
    //      <body><h1>Macros are the best!</h1></body></html>"
    // );
}
