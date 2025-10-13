use gui_lib::{Screen,Button, Draw};

struct SelectBox{
    input: String,
    height: u32,
    width: u32
}

impl Draw for SelectBox{
    fn draw(&self) {
        // impl draw
    }
}

fn main() {
    let sc = Screen{
        components : vec![
            Box::new(Button{
                height:10,
                width:12,
                label: String::from("hello from button")
            }),
            Box::new(SelectBox {
                height:10,
                width:12,
                input: String::from("")
            })
        ]
    };
    let b = Button {
        height:10,
        width:12,
        label: String::from("hello from button")
    };
    b.draw();
}