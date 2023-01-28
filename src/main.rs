use gdk4::traits::DisplayExt;
use gtk4::glib::clone;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::traits::{ButtonExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
use gtk4::Entry;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
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
    password_field.set_text(generate_password(10).as_str());
    grid.attach(&password_field, 0, 0, 1, 1);

    // Create the second button and put it into the grid at (1, 0)
    let button_2 = gtk4::Button::with_label("Generate");
    button_2.connect_clicked(
        clone!(@weak clipboard, @weak password_field => move |_btn| {
        password_field.set_text(generate_password(10).as_str());
               }),
    );

    grid.attach(&button_2, 1, 0, 1, 1);
    let button_3 = gtk4::Button::with_label("Copy");
    button_3.connect_clicked(
        clone!(@weak clipboard, @weak password_field => move |_btn| {
            let text = password_field.text();
            clipboard.set_text(&text);
        }),
    );
    grid.attach(&button_3, 2, 0, 1, 1);

    // Create the quit button and put it into the grid at (0, 1)
    let quit_button = gtk4::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    grid.attach(&quit_button, 0, 1, 3, 1);

    window.show();
}

fn generate_password(length: i32) -> String {
    let low_case = "abcdefghijklmnopqrstuvxyz";
    let up_case = "ABCDEFGHIJKLMNOPQRSTUVXYZ";
    let numbers = "0123456789";
    let chars = "\\!\"£$%&/()=?^*°:;.,";
    let all = format!("{}{}{}{}", low_case, up_case, numbers, chars);
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..all.len() - 2);
    let mut password = "".to_owned();
    for n in 1..length {
        let throw = die.sample(&mut rng);
        let mut char = all.chars();
        let char = char.nth(throw).unwrap();
        password = format!("{}{}", password, char);
    }
    return password;
}
