#![windows_subsystem = "windows"]

use fltk::{
    app::{App, AppScheme, channel},
    browser::Browser,
    button::{Button, CheckButton},
    dialog:: {FileChooser, FileChooserType},
    input::{IntInput},
    prelude::*,
    window::DoubleWindow,
};

use std::{thread, time};
//use regex::Regex;
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

#[derive(Debug, Copy, Clone)]
enum Message {
    Search,
    StartDirectory,
    LogFile,
}

fn main() {
    
    let myapp = App::default().with_scheme(AppScheme::Plastic);
    let vmargin = 10;
    let hmargin = 5;
    let row_height = 20;
    let row1_y = vmargin;
    let row2_y = row1_y + row_height + vmargin;
    let row3_y = row2_y + row_height + vmargin;
    let row4_y = row3_y + row_height + vmargin;
    let row5_y = row4_y + row_height + vmargin;
    let row6_y = row5_y + row_height + (vmargin / 2);
    let row7_y = row6_y + row_height + vmargin;
    let row8_y = row7_y + row_height + vmargin;
    let row9_y = row8_y + row_height + vmargin;
    let row10_y = row9_y + row_height + vmargin;
    let row11_y = row10_y + row_height + vmargin;
    let mut w = DoubleWindow::default().with_size(800, 600).center_screen().with_label("Rust File Crawler using fltk-rs");
    let mut title = controls::Label::new(hmargin, row1_y, 750, row_height, "", w.color());
    title.set_text_color(Color::Red);
    let greeting = controls::Label::new(hmargin, row2_y, 750, row_height, 
        "Welcome to FileCrawler! Search the contents of files for your search term or regular expression...",
        w.color());

    let mut directory_button = Button::default().with_pos(hmargin, row3_y).with_size(150, row_height)
                        .with_label("Choose Directory");
    let directory_label = controls::Label::new(175, row3_y, 400, row_height, "", w.color());

    let inp_filetypes = controls::TextBox::new(175, row4_y, 400, row_height, 
                        "txt", "File Types (e.g. txt,doc): ");

    let inp_search = controls::TextBox::new(85, row5_y, 400, row_height, "", "Search For:");
    let chk_usecase_button = CheckButton::default().with_pos(100, row6_y)
                                .with_size(20, row_height).with_align(Align::Left).with_label("Case sensitive");

    let inp_regex_search = controls::TextBox::new(365, row7_y, 425, row_height, "",
                         "Regular expression (supercedes search term and case):");

    let mut inp_max_files = IntInput::default().with_pos(140, row8_y)
                            .with_size(50, row_height).with_align(Align::Left).with_label("Max found files limit: ");
    inp_max_files.set_maximum_size(4);
    inp_max_files.set_value("250");

    let inp_log_file = controls::TextBox::new(130, row9_y,
                         400, row_height, "", "Log File (optional):");   
    let mut log_file_button = Button::default().with_pos(545, row9_y).with_size(125, 20)
                        .with_label("Choose Log File");

    let mut search_button = Button::default().with_pos(5, row10_y).with_size(100, row_height)
                        .with_label("Search");

    let found_file_browser = Browser::default().with_pos(5, row11_y).with_size(775, 290);

    w.make_resizable(true);
    w.end();
    w.show();

    let (s, r) = channel::<Message>();

    search_button.emit(s, Message::Search);
    directory_button.emit(s, Message::StartDirectory);
    log_file_button.emit(s, Message::LogFile);

    while myapp.wait().unwrap() {
        match r.recv() {
            Some(msg) => match msg {
                Message::Search => {
                    let (is_valid, text) = validate(directory_label.value().trim(),
                        inp_filetypes.value().trim(), inp_search.value().trim(), 
                        inp_regex_search.value().trim(), inp_max_files.value().trim());
                    title.set_value(&text);
                    w.redraw();
                    if !is_valid {
                        continue;
                    }
                    
                },
                Message::StartDirectory => {
                    let starting_directory = &directory_label.value();
                    let starting_directory = &(get_start_directory(&myapp, starting_directory));
                    directory_label.set_value(starting_directory);
                    w.redraw();
                }
                Message::LogFile => {
                    let log_file = &inp_log_file.value();
                    let log_file = &(get_log_file(&myapp, &log_file));
                    inp_log_file.set_value(log_file);
                    w.redraw();
                }
            }
            None => {}
        }
        thread::sleep(time::Duration::from_millis(16));
    }
}

fn get_log_file(myapp: &App, log_file: &str) -> String{
    let mut fc = FileChooser::new(log_file, "", FileChooserType::Single, "Choose your log file...");
    fc.show();
    while fc.shown() {
        myapp.wait().unwrap();
    }
    if fc.value(1).is_none(){
        return log_file.to_string();
    }
    else {
        fc.value(1).unwrap()
    }
}

fn get_start_directory(myapp: &App, start_directory: &str) -> String{
    let mut fc = FileChooser::new(start_directory, "", FileChooserType::Directory, "Choose your start directory...");
    fc.show();
    while fc.shown() {
        myapp.wait().unwrap();
    }
    //println!("{} {}", fc.value(1).unwrap(), fc.directory().unwrap());
    if fc.value(1).is_none(){
        start_directory.to_string()
    }
    else {
        fc.value(1).unwrap()
    }
}

fn validate (directory: &str, file_types: &str,
                inp_search: &str, inp_regex_search: &str,
                inp_max_files: &str) -> (bool, String) {
    let response_text : String;
    let retval: bool;
    let file_types_scrubbed = file_types.replace(" ", "");
    let file_types_count: usize = file_types_scrubbed.split(",").count();
    let file_types_rescrubbed = file_types_scrubbed.replace(",", "");
    if directory == "" {
        retval = false;
        response_text = "You need a starting directory.".to_string();
    }
    else if file_types_rescrubbed == "" {
        retval = false;
        response_text = "You need at least 1 file type.".to_string();
    }
    else if file_types_count > 25 {
        retval = false;
        response_text = "Maximum of 25 file types.".to_string();
    }
    else if inp_search == "" && inp_regex_search == "" {
        retval = false;
        response_text = "You need a search term or regular expression.".to_string();
    }
    else
    {
        let max_valid = match inp_max_files.parse::<i32>() {
            Ok (val) => {
                if val <= 0 {
                    false
                }
                else {
                //inp_max_files is set for maximum_size of 4 bytes, i.e. value of 9999
                //so no need to test for too large a number
                    true
                }
            },
            Err(_) => false,
        };
        if !max_valid {
            retval = false;
            response_text = "Max found files must be a number from 1 to 9999.".to_string();
        }
        else {
            retval = true;
            response_text = "".to_string();
        }
    }

    (retval, response_text)
}
