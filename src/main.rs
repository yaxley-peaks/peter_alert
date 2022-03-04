#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use std::cell::RefCell;

use peter_alert::{PATH, prepare_image, get_ico};
use nwd::NwgUi;
use nwg::NativeUi;


#[derive(NwgUi, Default)]
pub struct PeterAlert {
    #[nwg_control(size: (300, 300), position: (300, 300), title: "Peter Alert", flags: "WINDOW|VISIBLE", icon: Some(&data.icon))]
    #[nwg_events(   OnWindowClose: [PeterAlert::say_goodbye],
                    OnInit: [PeterAlert::decode_and_load_image] )]
    window: nwg::Window,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,
    peter_img: RefCell<Option<nwg::Bitmap>>,
    #[nwg_resource(source_file: Some("./peter.ico"))]
    icon: nwg::Icon,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 0, col: 1)]
    frame: nwg::ImageFrame,

    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    #[nwg_control(text: "")]
    pad: nwg::Label,

    #[nwg_control(text: "Peter Alert")]
    #[nwg_layout_item(layout: grid, row: 1, col:1)]
    peter: nwg::Label,

    #[nwg_layout_item(layout: grid, row: 0, col: 2)]
    #[nwg_control(text: "")]
    pad2: nwg::Label,
}
impl PeterAlert {
    fn decode_and_load_image(&self) {
        prepare_image();
        let image = match self.decoder.from_filename(PATH) {
            Ok(image) => image,
            Err(err) => {
                panic!("failed to load image: {}", err);
            }
        };
        let frame = match image.frame(0) {
            Ok(bmp) => bmp,
            Err(err) => {
                panic!("failed to get frame: {}", err);
            }
        };
        match frame.as_bitmap() {
            Ok(bmp) => {
                let mut img = self.peter_img.borrow_mut();
                img.replace(bmp);
                self.frame.set_bitmap(img.as_ref());
            }
            Err(err) => {
                panic!("failed to get bitmap: {}", err);
            }
        }
    }
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }
}
fn main() {
    // get_ico(); //this doesnt work for some reason
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = PeterAlert::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
