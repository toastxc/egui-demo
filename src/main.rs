use eframe::{HardwareAcceleration, Renderer};
use reywen_http::engines::hyper::Hyper;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

pub mod process;

pub mod view;

static MIN_WIDTH: f32 = 300.0;
static DEFAULT_WIDTH: f32 = 480.0;
static MIN_HEIGHT: f32 = 480.0;
static DEFAULT_HEIGHT: f32 = 480.0;



fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_WIDTH, DEFAULT_HEIGHT])
            .with_min_inner_size([MIN_WIDTH, MIN_HEIGHT])
            .with_transparent(true),
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        centered: false,
        ..Default::default()
    };

    eframe::run_native("App", options, Box::new(|_cc| Box::from(App::default()))).unwrap();
}

pub type Guard<T> = Arc<RwLock<T>>;

#[derive(Debug, Clone)]
struct App {
    // this field is only accessed in 'sync' code
    pub button1_text: String,

    // this field is accessed in 'sync' and 'async' code
    // since multiple threads can access the data it is protected
    pub label1_text: Guard<String>,

    // this will let users know if something is happening in the background
    pub spin: Guard<bool>,

    // this is out tokio thread runtime, it is never written to and as such only needs
    // an Arc<T>, instead of Arc<RwLock<T>>
    pub runtime: Arc<Runtime>,

    // this is the http engine used for making requests, I've selected my own but you can choose any!
    pub engine: Arc<Hyper>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            button1_text: String::from("Click to run a HTTP request"),
            label1_text: Arc::new(Default::default()),
            spin: Arc::new(Default::default()),
            runtime: Arc::new(Runtime::new().unwrap()),
            engine: Arc::new(Default::default()),
        }
    }
}
