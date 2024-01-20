use std::cell::RefCell;

use rand::{rngs::OsRng, seq::SliceRandom};
use gtk::{CssProvider, gdk::Screen, prelude::{CssProviderExt, ApplicationExt, ApplicationExtManual}};
use gtk::prelude::GtkWindowExt;
const DATA: &'static str = include_str!("br_utf8.txt");

#[derive(Debug)]
enum Word{
    Correct(String),
    Incorrect(String)
}

fn get_words()->Vec<&'static str>{
    let mut words: Vec<&str> = DATA.split("\n").map(|f|f.trim()).collect();
    words.shuffle(&mut OsRng);
    words
}

fn load_css(window: &gtk::ApplicationWindow) {
    let provider = CssProvider::new();
    ..CssProvider::load_from_data(&provider, include_str!("style.css").as_bytes()); 
    let screen: Screen = gtk::ApplicationWindow::screen(window).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION)
}

fn build_ui<'a>(f: &gtk::Application){
    
    let words = get_words();
    let current_index:RefCell<usize> = RefCell::new(0);


    let window = gtk::ApplicationWindow::new(f);
    load_css(&window);
    window.set_default_width(640);
    window.set_default_height(480);
    window.set_title("Teste de digitação");
    window.set_resizable(false);

    let window_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    
    let typed_words_ref: RefCell<Vec<Word>> = RefCell::new(Vec::new());

    let mut entry_buffer = gtk::EntryBuffer::new(Some(""));
    let entry = gtk::Entry::builder()
        .buffer(&mut entry_buffer)
        .placeholder_text("Digite aqui").build();
    
    let preview_text = words[1..10].join(" ");
    let mut  preview_buffer = gtk::TextBuffer::builder().text(preview_text).build();

    let preview = gtk::TextView::builder()
        .buffer(&mut preview_buffer)
        .editable(false)
        .can_focus(false)
        .build();
    let is_correct_preview_text = "";
    let mut  is_correct_preview_buffer = gtk::TextBuffer::builder().text(is_correct_preview_text).build();

    let is_correct_preview = gtk::TextView::builder()
        .buffer(&mut is_correct_preview_buffer)
        .editable(false)
        .can_focus(false)
        .build();

    let mut current_word =  RefCell::new(words.get(*current_index.borrow()).unwrap().to_string());
    
    let first_word_preview_text = current_word.borrow().clone();
    let mut  first_word_preview_buffer = gtk::TextBuffer::builder().text(first_word_preview_text).build();

    let first_word_preview = gtk::TextView::builder()
        .buffer(&mut first_word_preview_buffer)
        .editable(false)
        .can_focus(false)
        .build();
    
    ..=&entry.connect_changed(move |_|{
        let entry_text = entry_buffer.text();
        is_correct_preview_buffer.set_text(if current_word.borrow().starts_with(&entry_text) {"Correct!"}else{"Incorrect"});
        if entry_text.ends_with(" "){
            let entry_text = entry_text.trim().to_string();
            if entry_text.eq(&*current_word.borrow()){
                typed_words_ref.borrow_mut().push(Word::Correct(entry_text));
            }else{
                typed_words_ref.borrow_mut().push(Word::Incorrect(entry_text));
            }
            *current_index.borrow_mut()+=1;
            *current_word.borrow_mut()=words[*current_index.borrow()].to_string();
            entry_buffer.set_text("");
            preview_buffer.set_text(words[*current_index.borrow()+1..*current_index.borrow()+10].join(" ").as_str());
            is_correct_preview_buffer.set_text("");
            first_word_preview_buffer.set_text(&current_word.borrow());
        }
    });
    
    window.connect_key_press_event(|appwindow,f|{
        // exit on esc
        if f.keycode() == Some(9){
            appwindow.close();
        }
        gtk::glib::Propagation::Proceed
    });

    let preview_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let preview_and_first_word_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    
    preview.set_hexpand(true);
    preview.set_vexpand(true);
    entry.set_hexpand(true);
    entry.set_height_request(100);
    entry.grab_focus();

    preview_and_first_word_box.add(&first_word_preview);
    preview_and_first_word_box.add(&preview);

    preview.set_wrap_mode(gtk::WrapMode::WordChar);
    preview.set_widget_name("preview");
    is_correct_preview.set_widget_name("is_correct_preview");
    entry.set_widget_name("entry");
    preview_and_first_word_box.set_widget_name("preview_box");
    

    preview_box.add(&is_correct_preview);
    preview_box.add(&preview_and_first_word_box);

    use gtk::prelude::*;
    window_box.add(&preview_box);
    window_box.add(&entry);

    window.add(&window_box);
    window.show_all();
}

fn main() {
    let app = gtk::Application::builder().application_id("com.apollo.typing_test").build();

    app.connect_activate(move |f| build_ui(f));
    app.run();

    
}
