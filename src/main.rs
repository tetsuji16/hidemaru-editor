#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct HidemaruClone {
    #[nwg_control(size: (800, 600), position: (300, 300), title: "Hidemaru Clone (Rust)", accept_files: true)]
    #[nwg_events( OnWindowClose: [HidemaruClone::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    // --- Menus ---
    #[nwg_control(parent: window, text: "ファイル(&F)")]
    menu_file: nwg::Menu,

    #[nwg_control(parent: menu_file, text: "開く(&O)...")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_file])]
    menu_item_open: nwg::MenuItem,

    #[nwg_control(parent: menu_file, text: "上書き保存(&S)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::save_file])]
    menu_item_save: nwg::MenuItem,

    #[nwg_control(parent: menu_file)]
    menu_item_separator1: nwg::MenuSeparator,

    #[nwg_control(parent: menu_file, text: "秀丸エディタの終了(&X)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::exit])]
    menu_item_exit: nwg::MenuItem,

    #[nwg_control(parent: window, text: "編集(&E)")]
    menu_edit: nwg::Menu,

    // --- Editor Area ---
    #[nwg_control(text: "", flags: "VISIBLE|VSCROLL|HSCROLL")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 1, row_span: 1)]
    text_box: nwg::TextBox,

    // --- Status Bar ---
    #[nwg_control(parent: window)]
    status_bar: nwg::StatusBar,
}

impl HidemaruClone {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn open_file(&self) {
        nwg::modal_info_message(&self.window, "Info", "File Open is not implemented yet.");
    }

    fn save_file(&self) {
        nwg::modal_info_message(&self.window, "Info", "File Save is not implemented yet.");
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("ＭＳ ゴシック").expect("Failed to set default font");

    let _app = HidemaruClone::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
