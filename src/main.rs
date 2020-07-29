use fltk::{
    app::{App, AppScheme},
    browser::Browser,
    button::{Button, CheckButton},
    input::{IntInput},
    prelude::*,
    window::Window,
};
mod controls;

#[allow(unused_variables)]

//title label:
//greeting label: Welcome to FileCrawler! Search the contents of files for your search term or regular expression..."
//Starting Folder: (file_picker or text input)
//File Types (e.g. txt,doc) Default is txt: input text
//Search For: text input
//Case sensitive: checkbox
//Regular expression (supercedes search term and case): text input
//Max found files limit. Default is 250: numeric input
//Log File (optional): text input
//Search: button
//list of found files

fn main() {
    let app = App::default().with_scheme(AppScheme::Gleam);
    let gap = 10;
    let mut w = Window::default().with_size(800, 600).center_screen().with_label("Rust File Crawler using fltk-rs");
    let mut title = controls::Label::new(5, 5, 750, 20, "Error here", w.color());
    title.set_text_color(Color::Red);
    let greeting = controls::Label::new(5, title.y() + title.height() + gap, 750, 20, 
        "Welcome to FileCrawler! Search the contents of files for your search term or regular expression...",
        w.color());

    let file_button = Button::default().with_pos(5, greeting.y() + greeting.height() + gap).with_size(150, 20)
                        .with_label("Choose Directory");
    let directory = controls::Label::new(175, file_button.y(), 400, 20, "...", w.color());

    let inp_filetypes = controls::TextBox::new(245, file_button.y() + file_button.height() + gap, 400, 20, 
                        "txt", "File Types (e.g. txt,doc) Default is txt:");

    let inp_search = controls::TextBox::new(85, inp_filetypes.y() + inp_filetypes.height() + gap, 400, 20, "", "Search For:");
    let chk_usecase_button = CheckButton::default().with_pos(100, 
                                inp_search.y() + inp_search.height() + gap / 2)
                                .with_size(20, 20).with_align(Align::Left).with_label("Case sensitive");

    let regex_search = controls::TextBox::new(365, chk_usecase_button.y() + chk_usecase_button.height() + gap, 425, 20, "",
                         "Regular expression (supercedes search term and case):");

    let mut inp_max_files = IntInput::default().with_pos(140, regex_search.y() + regex_search.height() + gap)
                            .with_size(50, 20).with_align(Align::Left).with_label("Max found files limit: ");
    inp_max_files.set_maximum_size(4);
    inp_max_files.set_value("250");

    let inp_logfile = controls::TextBox::new(130, inp_max_files.y() + inp_max_files.height() + gap,
                         400, 20, "", "Log File (optional):");   
    let logfile_button = Button::default().with_pos(545, inp_logfile.y()).with_size(125, 20)
                        .with_label("Choose Log File");

    let search_button = Button::default().with_pos(5, inp_logfile.y() + inp_logfile.height() + gap).with_size(100, 20)
                        .with_label("Search");

    let found_file_browser = Browser::default().with_pos(5, search_button.y() + search_button.height() + gap).with_size(775, 300);

    w.make_resizable(true);
    w.end();
    w.show();

    app.run().unwrap();
}
