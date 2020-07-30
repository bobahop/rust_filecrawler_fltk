use fltk::{input::Input, output::Output, prelude::*};
use std::ops::{Deref, DerefMut};

pub struct TextBox {
    pub text_box: Input,
}

impl TextBox {
    pub fn new(x: i32, y: i32, w: i32, h: i32, default_text: &str, label_text: &str) -> TextBox {
        let tb = TextBox {
            text_box: Input::default()
                .with_pos(x, y)
                .with_size(w, h)
                .with_align(Align::Left)
                .with_label(label_text),
        };
        tb.text_box.set_value(default_text);
        tb
    }
}

impl Deref for TextBox {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        &self.text_box
    }
}

impl DerefMut for TextBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text_box
    }
}

pub struct Label {
    pub label: Output,
}

impl Label {
    pub fn new(x: i32, y: i32, w: i32, h: i32, text: &str, bg_color: Color) -> Label {
        let mut lbl = Label {
            label: Output::default().with_pos(x, y).with_size(w, h),
        };
        lbl.label.set_value(text);
        lbl.label.set_frame(FrameType::NoBox);
        lbl.label.set_color(bg_color);
        //lbl.label.set_readonly(true);
        lbl
    }
}

impl Deref for Label {
    type Target = Output;

    fn deref(&self) -> &Self::Target {
        &self.label
    }
}

impl DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.label
    }
}
