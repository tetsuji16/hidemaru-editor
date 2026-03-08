mod text_engine;
mod find_dialog;

use text_engine::TextEngine;
use find_dialog::FindDialog;
use std::cell::RefCell;
use std::rc::Rc;

use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct HidemaruClone {
    #[nwg_control(size: (1024, 768), position: (100, 100), title: "秀丸エディタ - (無題) [Rust]", accept_files: true)]
    #[nwg_events( OnWindowClose: [HidemaruClone::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    // --- Menus ---
    #[nwg_control(parent: window, text: "ファイル(&F)")]
    menu_file: nwg::Menu,

    #[nwg_control(parent: menu_file, text: "新規作成(&N)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::new_file])]
    menu_item_new: nwg::MenuItem,

    #[nwg_control(parent: menu_file, text: "開く(&O)...")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_file])]
    menu_item_open: nwg::MenuItem,

    #[nwg_control(parent: menu_file, text: "上書き保存(&S)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::save_file])]
    menu_item_save: nwg::MenuItem,

    #[nwg_control(parent: menu_file, text: "名前を付けて保存(&A)...")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::save_as_file])]
    menu_item_save_as: nwg::MenuItem,

    #[nwg_control(parent: menu_file)]
    menu_item_separator1: nwg::MenuSeparator,

    #[nwg_control(parent: menu_file, text: "秀丸エディタの終了(&X)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::exit])]
    menu_item_exit: nwg::MenuItem,

    #[nwg_control(parent: window, text: "編集(&E)")]
    menu_edit: nwg::Menu,

    #[nwg_control(parent: menu_edit, text: "検索(&S)\tCtrl+F")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_find])]
    menu_item_find: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "やり直し(&U)\tCtrl+Z")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::undo])]
    menu_item_undo: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "元に戻したのをやり直し(&R)\tCtrl+Y")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::redo])]
    menu_item_redo: nwg::MenuItem,

    #[nwg_control(parent: menu_edit)]
    menu_item_edit_sep1: nwg::MenuSeparator,

    #[nwg_control(parent: menu_edit, text: "切り取り(&T)\tCtrl+X")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::cut])]
    menu_item_cut: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "コピー(&C)\tCtrl+C")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::copy])]
    menu_item_copy: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "貼り付け(&P)\tCtrl+V")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::paste])]
    menu_item_paste: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "削除(&D)\tDel")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::delete])]
    menu_item_delete: nwg::MenuItem,

    #[nwg_control(parent: menu_edit)]
    menu_item_edit_sep2: nwg::MenuSeparator,

    #[nwg_control(parent: menu_edit, text: "すべて選択(&A)\tCtrl+A")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::select_all])]
    menu_item_select_all: nwg::MenuItem,

    #[nwg_control(parent: window, text: "検索(&S)")]
    menu_search: nwg::Menu,

    #[nwg_control(parent: window, text: "設定(&O)")]
    menu_option: nwg::Menu,

    #[nwg_control(parent: window, text: "ウィンドウ(&W)")]
    menu_window: nwg::Menu,

    #[nwg_control(parent: window, text: "ヘルプ(&H)")]
    menu_help: nwg::Menu,

    // --- Editor Area ---
    #[nwg_control(text: "", flags: "VISIBLE|VSCROLL|HSCROLL")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 1, row_span: 1)]
    text_box: nwg::TextBox,

    // --- Status Bar ---
    #[nwg_control(parent: window)]
    status_bar: nwg::StatusBar,

    // --- Resources ---
    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Text Files (*.txt)|*.txt|All Files (*.*)|*.*")]
    file_dialog: nwg::FileDialog,

    #[nwg_resource(title: "Save File", action: nwg::FileDialogAction::Save, filters: "Text Files (*.txt)|*.txt|All Files (*.*)|*.*")]
    save_dialog: nwg::FileDialog,

    #[nwg_control]
    #[nwg_events( OnNotice: [HidemaruClone::on_find_notice] )]
    find_notice: nwg::Notice,

    // --- State ---
    engine: RefCell<TextEngine>,
    find_dialog_ui: RefCell<Option<find_dialog::FindDialogUi>>,
}

impl HidemaruClone {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn new_file(&self) {
        let mut engine = self.engine.borrow_mut();
        *engine = TextEngine::new();
        self.text_box.set_text("");
        self.window.set_text("秀丸エディタ - (無題) [Rust]");
        self.status_bar.set_text(0, "新規作成しました。");
    }

    fn open_file(&self) {
        if self.file_dialog.run(Some(&self.window)) {
            if let Ok(path) = self.file_dialog.get_selected_item() {
                let path_str = path.into_string().unwrap();
                let mut engine = self.engine.borrow_mut();
                if let Ok(_) = engine.load_from_file(&path_str) {
                    self.text_box.set_text(&engine.get_text());
                    self.window.set_text(&format!("秀丸エディタ - {} [Rust]", path_str));
                    self.status_bar.set_text(0, &format!("読み込み完了: {}", path_str));
                }
            }
        }
    }

    fn save_file(&self) {
        let mut engine = self.engine.borrow_mut();
        engine.set_text(&self.text_box.text());
        
        if let Some(ref path) = engine.file_path.clone() {
            if let Ok(_) = engine.save_to_file(path) {
                self.status_bar.set_text(0, &format!("保存しました: {}", path));
            }
        } else {
            self.save_as_file();
        }
    }

    fn save_as_file(&self) {
        if self.save_dialog.run(Some(&self.window)) {
            if let Ok(path) = self.save_dialog.get_selected_item() {
                let path_str = path.into_string().unwrap();
                let mut engine = self.engine.borrow_mut();
                engine.set_text(&self.text_box.text());
                if let Ok(_) = engine.save_to_file(&path_str) {
                    engine.file_path = Some(path_str.clone());
                    self.window.set_text(&format!("秀丸エディタ - {} [Rust]", path_str));
                    self.status_bar.set_text(0, &format!("名前を付けて保存完了: {}", path_str));
                }
            }
        }
    }

    fn open_find(&self) {
        let mut find_diag_opt = self.find_dialog_ui.borrow_mut();
        if find_diag_opt.is_none() {
            use nwg::NativeUi;
            let data = FindDialog::build_ui(Default::default()).expect("Failed to build Find Dialog");
            // Setup the notice sender
            data.notice_sender.replace(Some(self.find_notice.sender()));
            *find_diag_opt = Some(data);
        }

        if let Some(diag) = find_diag_opt.as_ref() {
            diag.window.set_visible(true);
            diag.window.set_focus();
        }
    }

    fn on_find_notice(&self) {
        let find_diag_ref = self.find_dialog_ui.borrow();
        if let Some(diag) = find_diag_ref.as_ref() {
            let pattern = diag.find_text.text();
            let engine = self.engine.borrow();
            
            // Get current cursor position (start point for search)
            let current_sel = self.text_box.selection();
            let start_char = current_sel.end as usize;

            if let Some(result) = engine.find(&pattern, start_char) {
                let char_start = engine.buffer.byte_to_char(result.start_byte) as u32;
                let char_end = engine.buffer.byte_to_char(result.end_byte) as u32;
                self.text_box.set_selection(char_start..char_end);
                self.text_box.set_focus();
                self.status_bar.set_text(0, &format!("見つかりました: {}", pattern));
            } else {
                nwg::modal_info_message(&diag.window, "検索", &format!("「{}」は見つかりませんでした。", pattern));
            }
        }
    }

    fn undo(&self) { /* To be implemented via Engine */ }
    fn redo(&self) { /* To be implemented via Engine */ }
    
    fn cut(&self) { 
        use winapi::um::winuser::{SendMessageW, WM_CUT};
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_CUT, 0, 0); }
    }
    
    fn copy(&self) { 
        use winapi::um::winuser::{SendMessageW, WM_COPY};
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_COPY, 0, 0); }
    }
    
    fn paste(&self) { 
        use winapi::um::winuser::{SendMessageW, WM_PASTE};
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_PASTE, 0, 0); }
    }
    
    fn delete(&self) { 
        use winapi::um::winuser::{SendMessageW, WM_CLEAR};
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_CLEAR, 0, 0); }
    }
    
    fn select_all(&self) { 
        use winapi::um::winuser::{SendMessageW, EM_SETSEL};
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_SETSEL as u32, 0, -1_isize); }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("ＭＳ ゴシック").expect("Failed to set default font");

    let _app = HidemaruClone::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
