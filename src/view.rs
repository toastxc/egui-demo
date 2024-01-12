use crate::App;
use egui::Ui;

// calling the trait for eframe
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }
}

// this is where the UI elements are created
// Egui is immediate so it allows you to do weird things like
//
// if value.field.is_true() {
//    ui.button("Hello World!")
// }
//
// The one thing Egui struggles with is a very long scrollable interface
//
fn update_fn(value: &mut App, ui: &mut Ui) {
    value.do_stuff();

    // the field button1_text can be accessed directly because it is not Guarded
    if ui.button(&value.button1_text).clicked() {
        value.request();
    };

    // if true, will display a spinner
    if *value.spin.read().unwrap() {
        ui.spinner();
    };

    // however as mention in main.rs, label1_text is accessed im multiple threads and so needs to be called
    // using read() or write()
    ui.label(&*value.label1_text.read().unwrap());
}
