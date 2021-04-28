use quick_xml::Reader;
use quick_xml::events::Event;
use std::env;
use std::fs::File;
use std::io::prelude::*;


/* ***************************** */

// #構造体

struct Rider {
    object: Object,
    charactor: Charactor,
}


// MEMO: オブジェクトはpathの格納時に型も一緒に検知できるようにする。画像、3Dデータを格納するようにする
struct Object {
    // オブジェクトが格納されているPATHと利用するプログラムの型を格納する
    path: String,
    type: String,
    status: Status,
}

struct Charctor {
    path: String,
    type: String,
    status: Status,
}

// MEMO: 設定時にマクロを使って新しい型を生成する必要がありそう
struct Status { 

}

    
/* ***************************** */

fn main() {

    /* 使う変数を設定する */
    let mut contents = String::new();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];


    println!("{:?}", filename);

    /* fileをオープンをする */
    let mut f = File::open(filename).expect("file not found");

    println!("{:?}", f);

    /* contentsに文字列を入れる */
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
    
    // xmlの文字列
    let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test</tag2>
                <tag2>
                    Test 2
                </tag2>
            </tag1>"#;

    /* コンテンツのrenderを作る */
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);


    // テキストファイルで使う変数
    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    let test = crates_io();
    println!("{:?}", test);
}
