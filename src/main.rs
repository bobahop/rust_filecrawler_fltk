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

use std::{error::Error, fmt, thread, time};
use std::fs::{File, OpenOptions} ;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use walkdir::WalkDir;
mod controls;

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

#[derive(Debug)]
struct BobError {
    text: String,
}
impl fmt::Display for BobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.text)
    }
}
impl Error for BobError {}

const BAD_FILE_CHARS: &str = r#"[\\/:*?"<>|]"#;

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
    let _greeting = controls::Label::new(hmargin, row2_y, 750, row_height, 
        "Welcome to FileCrawler! Search the contents of files for your search term or regular expression...",
        w.color());

    let mut directory_button = Button::default().with_pos(hmargin, row3_y).with_size(150, row_height)
                        .with_label("Choose Directory");
    let directory_label = controls::Label::new(175, row3_y, 400, row_height, "", w.color());

    let inp_filetypes = controls::TextBox::new(165, row4_y, 400, row_height, 
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

    let mut found_file_browser = Browser::default().with_pos(5, row11_y).with_size(775, 290);

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
                    let (is_valid, text) = validate(&directory_label.value(),
                        &inp_filetypes.value(), &inp_search.value(), 
                        &inp_regex_search.value(), &inp_max_files.value(),
                        &inp_log_file.value());
                    title.set_value(&text);
                    w.redraw();
                    if !is_valid {
                        continue;
                    }

                    let extensions: Vec<Regex>;
                    let result = extensions_factory(&inp_filetypes.value());
                    match result {
                        Ok(v) => extensions = v,
                        Err(e) => {
                            title.set_value(&e.text);
                            w.redraw();
                            continue;
                        }
                    }

                    let search_term: Regex;
                    let result = set_search_term(&inp_search.value(),
                                 chk_usecase_button.is_checked(),
                                 &inp_regex_search.value());
                    match result {
                        Ok(v) => search_term = v,
                        Err(e) => {
                            title.set_value(&e.text);
                            w.redraw();
                            continue;
                        },
                    }

                    found_file_browser.clear();

                    let file_list =
                    search(&search_term, &directory_label.value(), &extensions, 
                            inp_max_files.value().parse::<usize>().unwrap());
                            
                    if inp_log_file.value().trim() == "" {
                        for file in file_list {
                            found_file_browser.add(&file);
                        }
                    }
                    else {
                        log(&inp_log_file.value().trim(), &file_list);
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
                inp_max_files: &str, inp_log_file: &str) -> (bool, String) {
    let response_text : String;
    let retval: bool;
    let file_types_scrubbed = file_types.trim().replace(" ", "");
    let file_types_count: usize = file_types_scrubbed.split(",").count();
    let file_types_rescrubbed = file_types_scrubbed.replace(",", "");
    let inp_log_file = inp_log_file.trim();
    if directory.trim() == "" {
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
    else if inp_search.trim() == "" && inp_regex_search.trim() == "" {
        retval = false;
        response_text = "You need a search term or regular expression.".to_string();
    }
    else if inp_log_file != "" && Regex::new(BAD_FILE_CHARS).unwrap().is_match(inp_log_file) {
            retval = false;
            response_text = format!("{} is a bad file name", inp_log_file);
    }
    else
    {
        let max_valid = match inp_max_files.trim().parse::<i32>() {
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

fn extensions_factory(ext: &str) -> Result<Vec<Regex>, BobError> {
    //do case insensitive match for filename ending. Example: "(?i)\.txt$"
    //maximum of 25 extensions
    let ext_scrubbed = ext.trim().replace(" ", "");
    let raw_extensions: Vec<&str> = ext_scrubbed.split(",").collect();

    let mut regexts: Vec<Regex> = Vec::with_capacity(raw_extensions.len());
    for raw_extension in raw_extensions.iter() {
        let mut raw_extension = raw_extension.to_string();
        if raw_extension.starts_with(".") {
            raw_extension = "(?i)\\".to_string() + &raw_extension + "$";
        } else {
            raw_extension = "(?i)\\.".to_string() + &raw_extension + "$";
        }
        let reg_result = Regex::new(&raw_extension);
        match reg_result {
            Err(_) => {
                return Err(BobError {
                    text: format!("Failed to accept extension {}", &raw_extension),
                })
            }
            Ok(v) => regexts.push(v),
        }
    }

    Ok(regexts.clone())
}

fn set_search_term(
    term: &str, case: bool, regexp: &str
) -> Result<Regex, BobError> {
    let result = set_regex(regexp, case, term);
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(BobError {text: get_regerror(&e, regexp, case, term)})
        },
    }
}

fn set_regex(regexp: &str, case: bool, term: &str) -> Result<Regex, regex::Error> {
    if regexp != "" {
        Regex::new(regexp)
    } else {
        let caseterm = if case {
            ""
        } else {
            "(?i)"
        };
        let mut searchterm = String::from(caseterm);
        searchterm.push_str(term);
        Regex::new(&searchterm)
    }
}

fn get_regerror(
    error: &regex::Error,
    regexp: &str,
    case: bool,
    term: &str,
)  -> String {
    if regexp != "" {
        format!("Problem regexp {} into regex {:?}", regexp, error)
    } else {
        format!(
            "Problem parsing term \"{}\" and case \"{}\" into regex {:?}",
            term, case, error
        )
    }
}

fn is_valid_file(file_name: &str, extensions: &Vec<Regex>) -> bool {
    for extension in extensions {
        if extension.is_match(file_name) {
            return true;
        }
    }
    false
}

fn file_has_match(entry: &walkdir::DirEntry, search_reg: &Regex) -> bool {
    let file: File;
    let result = File::open(entry.path());
    match result {
        Ok(v) => file = v,
        Err(_) => return false,
    }
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let result = buf_reader.read_to_string(&mut contents);
    match result {
        Ok(_) => search_reg.is_match(&contents),
        Err(_) => false,
    }
}

fn search(search_reg: &Regex, root: &str, extensions: &Vec<Regex>, 
    max_files: usize,) -> Vec<String>{
    let mut found_count: usize = 0;
    let mut file_list: Vec<String> = Vec::with_capacity(max_files);
    for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if !is_valid_file(entry.file_name().to_str().unwrap(), extensions) {
                continue;
            }
            if file_has_match(&entry, &search_reg) {
                found_count += 1;
                if found_count <= max_files {
                    file_list.push(entry.path().to_string_lossy().to_string());
                }
                else {
                    file_list.push(format!("Found files exceeded the limit of {}", max_files).to_string());
                    break;
                }
            }
        }
    }
    file_list
}

fn log(log_name: &str, file_list: &Vec<String>){
    let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(log_name)
                    .unwrap();

    for found_file in file_list {
        file.write_all(found_file.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}





