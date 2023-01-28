use gdk4::traits::DisplayExt;
use gtk4::glib::clone;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::traits::{ButtonExt, CheckButtonExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
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
    password_field.set_text(generate_password(10, true, true, true, true).as_str());
    grid.attach(&password_field, 0, 0, 1, 1);

    let upper_case = gtk4::CheckButton::with_label("Upper case letters");
    upper_case.activate();
    let lower_case = gtk4::CheckButton::with_label("Lower case letters");
    lower_case.activate();
    let numbers = gtk4::CheckButton::with_label("Numbers");
    numbers.activate();
    let symbols = gtk4::CheckButton::with_label("Symbols");
    symbols.activate();
    grid.attach(&upper_case, 0, 1, 1, 1);
    grid.attach(&lower_case, 0, 2, 1, 1);
    grid.attach(&numbers, 0, 3, 1, 1);
    grid.attach(&symbols, 0, 4, 1, 1);

    let generate_button = gtk4::Button::with_label("Generate");
    generate_button.connect_clicked(
        clone!(@weak clipboard, @weak upper_case,  @weak lower_case,  @weak numbers,  @weak  symbols, @weak password_field => move |_btn| {
        password_field.set_text(generate_password(10, upper_case.is_active(), lower_case.is_active(), numbers.is_active(), symbols.is_active()).as_str());
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

    grid.attach(&quit_button, 0, 5, 3, 1);

    window.show();
}
