// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,              // 退出消息
    Echo,              // 回显消息
    Move,              // 移动消息
    ChangeColor,       // 颜色改变消息
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
