use gdk4::traits::DisplayExt;
use gtk4::glib::clone;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::traits::{
    ButtonExt, CheckButtonExt, EditableExt, GridExt, GtkWindowExt, RangeExt, WidgetExt,
};
use password_generator::core::generate_password;
pub mod password_generator;

fn main() {
    let application =
        gtk4::Application::new(Some("com.andrei.minipasswordgenerator"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("Mini password generator"));
    window.set_default_size(340, 120);
    let display = gdk4::Display::default().unwrap();
    let clipboard = display.clipboard();
    let grid = gtk4::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    window.set_child(Some(&grid));

    let password_field = gtk4::Entry::builder().placeholder_text("").build();
    password_field.set_text(generate_password(20, true, true, true, true).as_str());
    grid.attach(&password_field, 0, 0, 1, 1);

    let upper_case = gtk4::CheckButton::with_label("Upper case letters");
    upper_case.activate();
    let lower_case = gtk4::CheckButton::with_label("Lower case letters");
    lower_case.activate();
    let numbers = gtk4::CheckButton::with_label("Numbers");
    numbers.activate();
    let symbols = gtk4::CheckButton::with_label("Symbols");
    symbols.activate();
    grid.attach(&upper_case, 0, 2, 1, 1);
    grid.attach(&lower_case, 0, 3, 1, 1);
    grid.attach(&numbers, 0, 4, 1, 1);
    grid.attach(&symbols, 0, 5, 1, 1);

    let length_box = gtk4::Entry::builder().placeholder_text("").build();
    length_box.set_text("20");
    grid.attach(&length_box, 0, 1, 1, 1);

    let range = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 1.0, 32.0, 1.0);
    range.set_value(20.0);    
    grid.attach(&range, 1, 1, 2, 1);
    range.connect_value_changed(clone!(@weak length_box=>move |range| {
        length_box.set_text( format!("{}",range.value().trunc()).as_str());
    }));

    let generate_button = gtk4::Button::with_label("Generate");
    generate_button.connect_clicked(
        clone!(@weak length_box, @weak clipboard, @weak upper_case,  @weak lower_case,  @weak numbers,  @weak  symbols, @weak password_field => move |_btn| {
       let length_opt = length_box.text().parse::<i32>();
let mut length = 10;
       if length_opt.is_ok(){
    length = length_opt.unwrap();
}
       
            password_field.set_text(generate_password(length, upper_case.is_active(), lower_case.is_active(), numbers.is_active(), symbols.is_active()).as_str());
               }),
    );
    grid.attach(&generate_button, 1, 0, 1, 1);

    let copy_button = gtk4::Button::with_label("Copy");
    copy_button.connect_clicked(
        clone!(@weak clipboard, @weak password_field => move |_btn| {
            let text = password_field.text();
            clipboard.set_text(&text);
        }),
    );
    grid.attach(&copy_button, 2, 0, 1, 1);

    let quit_button = gtk4::Button::with_label("Exit");
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    grid.attach(&quit_button, 0, 6, 3, 1);

    window.show();
}
