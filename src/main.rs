use gdk4::traits::DisplayExt;
use gtk4::glib::clone;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::traits::{
    ButtonExt, CheckButtonExt, EditableExt, GridExt, GtkWindowExt, RangeExt, WidgetExt,
};
use gtk4::Label;
use password_generator::core::{calculate_entropy, generate_password};
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

    // buttons
    let password_field = gtk4::Entry::builder().placeholder_text("").build();
    let upper_case = gtk4::CheckButton::with_label("Upper case letters");
    let lower_case = gtk4::CheckButton::with_label("Lower case letters");
    let numbers = gtk4::CheckButton::with_label("Numbers");
    let symbols = gtk4::CheckButton::with_label("Symbols");
    let length_box = gtk4::Entry::builder().placeholder_text("").build();
    let generated_password = generate_password(20, true, true, true, true);
    let range = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 1.0, 32.0, 1.0);
    let generate_button = gtk4::Button::with_label("Generate");
    let copy_button = gtk4::Button::with_label("Copy");
    let quit_button = gtk4::Button::with_label("Exit");
    let title = gtk4::Label::builder()
        .label("Password entropy")
        .halign(gtk4::Align::Start)
        .build();
    title.add_css_class("title-2");

    // attaching to the grid
    grid.attach(&password_field, 0, 0, 1, 1);
    grid.attach(&upper_case, 0, 2, 1, 1);
    grid.attach(&lower_case, 0, 3, 1, 1);
    grid.attach(&numbers, 0, 4, 1, 1);
    grid.attach(&symbols, 0, 5, 1, 1);
    grid.attach(&length_box, 0, 1, 1, 1);
    grid.attach(&copy_button, 2, 0, 1, 1);
    grid.attach(&quit_button, 0, 6, 3, 1);
    grid.attach(&title, 1, 2, 1, 1);
    grid.attach(&generate_button, 1, 0, 1, 1);
    grid.attach(&range, 1, 1, 2, 1);

    // setting default values for checkboxes
    upper_case.activate();
    lower_case.activate();
    numbers.activate();
    symbols.activate();

    length_box.set_text("20");
    password_field.set_text(generated_password.0.as_str());
    range.set_value(20.0);
    update_entropy_label(&title, generated_password.1);

    // actions
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));
    generate_button.connect_clicked(
        clone!(@weak title, @weak length_box, @weak clipboard, @weak upper_case,  @weak lower_case,  @weak numbers,  @weak  symbols, @weak password_field => move |_btn| {
        let length_opt = length_box.text().parse::<i32>();
        let mut length = 10;
        if length_opt.is_ok(){
            length = length_opt.unwrap();
        }
        let generated_password = generate_password(length, upper_case.is_active(), lower_case.is_active(), numbers.is_active(), symbols.is_active());
            password_field.set_text(generated_password.0.as_str());
            update_entropy_label(&title, generated_password.1);
        }),
    );
    range.connect_value_changed(clone!(@weak length_box=>move |range| {
        length_box.set_text( format!("{}",range.value().trunc()).as_str());
    }));
    copy_button.connect_clicked(
        clone!(@weak clipboard, @weak password_field => move |_btn| {
            let text = password_field.text();
            clipboard.set_text(&text);
        }),
    );

    window.show();
}

fn update_entropy_label(label: &Label, entropy: f64) {
    label.set_label(format!("Password Entropy: {}", entropy.trunc()).as_str());
}
