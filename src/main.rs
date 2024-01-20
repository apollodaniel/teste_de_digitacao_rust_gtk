use std::{ops::Add, cell::RefCell};

use rand::{prelude::*, rngs::OsRng};
use gtk::{prelude::*, ListBoxRow, glib::bitflags::iter::Iter};

const data: &'static str = include_str!("br_utf8.txt");

#[derive(Debug)]
enum Word{
    Correct(String),
    Incorrect(String)
}

fn get_words()->Vec<&'static str>{
    let mut words: Vec<&str> = data.split("\n").map(|f|f.trim()).collect();
    words.shuffle(&mut OsRng);
    words
}

fn build_ui<'a>(f: &gtk::Application){
    
    let words = get_words();
    let mut current_index:RefCell<usize> = RefCell::new(0);


    let window = gtk::ApplicationWindow::new(f);
    window.set_default_width(640);
    window.set_default_height(480);

    window.set_resizable(false);

    let window_box = gtk::Box::new(gtk::Orientation::Vertical, 32);

    
    let mut typed_words: Vec<Word> = Vec::new();
    let typed_words_ref = RefCell::new(typed_words);

    let mut entry_buffer = gtk::EntryBuffer::new(Some(""));
    let entry = gtk::Entry::builder()
        .buffer(&mut entry_buffer)
        .placeholder_text("Type here").build();
    
    let preview_text = words[0..5].join(" ");
    let mut  preview_buffer = gtk::TextBuffer::builder().text(preview_text).build();

    let preview = gtk::TextView::builder()
        .buffer(&mut preview_buffer)
        .editable(false)
        .build();
    ..=&entry.connect_changed(move|_|{
        let entry_text = entry_buffer.text();
        if entry_text.ends_with(" "){
            let entry_text = entry_text.trim().to_string();
            let current_word =  words.get(*current_index.borrow()).unwrap();
            if entry_text.eq(current_word){
                typed_words_ref.borrow_mut().push(Word::Correct(entry_text));
            }else{
                typed_words_ref.borrow_mut().push(Word::Incorrect(entry_text));
            }
            entry_buffer.set_text("");
            *current_index.borrow_mut()+=1;
            preview_buffer.set_text(words[*current_index.borrow()..*current_index.borrow()+5].join(" ").as_str());
        }
    });
    
    window.connect_key_press_event(|appwindow,f|{
        // exit on esc
        if f.keycode() == Some(9){
            appwindow.close();
        }
        gtk::glib::Propagation::Proceed
    });
    
    preview.set_hexpand(true);
    preview.set_vexpand(true);
    entry.set_hexpand(true);
    entry.set_vexpand(true);
    entry.grab_focus();

    entry.set_margin_bottom(64);
    entry.set_margin_top(64);

    window_box.add(&preview);
    window_box.add(&entry);

    window.add(&window_box);
    window.show_all();
}

fn main() {
    

    let app = gtk::Application::builder().application_id("com.apollo.typing_test").build();

    app.connect_activate(move |f| build_ui(f));
    app.run();

    
}
