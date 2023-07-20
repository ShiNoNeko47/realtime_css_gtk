use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk::Screen;
use gtk::prelude::*;
use gtk::Adjustment;
use gtk::Application;
use gtk::Button;
use gtk::CheckButton;
use gtk::CssProvider;
use gtk::Image;
use gtk::Label;
use gtk::RadioButton;
use gtk::ScrolledWindow;
use gtk::StyleContext;
use gtk::TextTagTable;
use gtk::Window;
use sourceview4::prelude::*;
use sourceview4::Buffer;

fn main() {
    let app = gtk::Application::builder().build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::builder().application(app).title("").build();
    let (editor, buffer) = editor();
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let scrolled_window = ScrolledWindow::builder().build();
    scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Never);
    window.set_child(Some(&hbox));
    hbox.add(&editor);
    hbox.add(&scrolled_window);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    scrolled_window.add(&vbox);

    vbox.set_valign(gtk::Align::Start);
    vbox.set_vexpand(true);
    box_fill(&vbox);

    let css_provider = CssProvider::new();
    let last_valid_css: Rc<RefCell<String>> =
        Rc::new(RefCell::new(css_provider.to_str().to_string()));
    StyleContext::add_provider_for_screen(
        &Screen::default().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    buffer.connect_changed(move |buffer| {
        let css = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .unwrap();
        if let Ok(_) = css_provider.load_from_data(css.as_bytes()) {
            last_valid_css.replace(css.to_string());
        } else {
            css_provider
                .load_from_data(last_valid_css.borrow().as_bytes())
                .unwrap();
        }
    });
    window.show_all();
}

fn editor() -> (ScrolledWindow, Buffer) {
    let buffer = Buffer::new(Some(&TextTagTable::new()));
    buffer.set_highlight_syntax(true);
    if let Some(ref language) = sourceview4::LanguageManager::new().language("css") {
        buffer.set_language(Some(language));
    }
    if let Some(ref scheme) = sourceview4::StyleSchemeManager::new().scheme("twilight") {
        buffer.set_style_scheme(Some(scheme));
    }
    let editor = sourceview4::View::with_buffer(&buffer);
    editor.set_monospace(true);
    editor.set_tab_width(4);
    editor.set_auto_indent(true);
    editor.set_hexpand(true);
    editor.set_vexpand(true);
    let editor_container = ScrolledWindow::new(Adjustment::NONE, Adjustment::NONE);
    editor_container.set_child(Some(&editor));

    (editor_container, buffer)
}

fn box_fill(vbox: &gtk::Box) {
    vbox.add(&Button::builder().label("Button").build());
    vbox.add(&RadioButton::builder().label("Radio button").build());
    vbox.add(&CheckButton::builder().label("Checkbox").build());
    vbox.add(&Label::builder().label("Label").build());
    vbox.add(&Image::builder().build());
}
