
mod text_engine;
mod find_dialog;
mod replace_dialog;

use text_engine::TextEngine;
use find_dialog::FindDialog;
use replace_dialog::ReplaceDialog;
use std::cell::RefCell;

use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

// Import winapi constants
use winapi::um::winuser::{
    SendMessageW, GetScrollPos, SB_HORZ,
    WM_CUT, WM_COPY, WM_PASTE, WM_CLEAR,
    EM_SETSEL, EM_GETLINECOUNT, EM_GETFIRSTVISIBLELINE, EM_LINESCROLL,
    EM_LINEFROMCHAR, EM_LINEINDEX
};
use winapi::um::commctrl::{SB_SETPARTS};


#[derive(Default, NwgUi)]
pub struct HidemaruClone {
    #[nwg_control(size: (1024, 768), position: (100, 100), title: "秀丸エディタ - (無題) [Rust]", accept_files: true)]
    #[nwg_events( OnWindowClose: [HidemaruClone::exit], OnFileDrop: [HidemaruClone::file_drop(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 0, margin: [1,1,1,1])]
    layout: nwg::GridLayout,

    // --- Menus ---
    #[nwg_control(parent: window, text: "ファイル(&F)")]
    menu_file: nwg::Menu,

    #[nwg_control(parent: menu_file, text: "新規作成(&N)")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::new_file])]
    menu_item_new: nwg::MenuItem,

    #[nwg_control(parent: menu_file, text: "開く(&O)...")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_file_dialog])]
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
    
    #[nwg_control(parent: menu_edit, text: "検索(&S)\tCtrl+F")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_find])]
    menu_item_find: nwg::MenuItem,

    #[nwg_control(parent: menu_edit, text: "置換(&R)\tCtrl+H")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::open_replace])]
    menu_item_replace: nwg::MenuItem,
    
    #[nwg_control(parent: menu_edit)]
    menu_item_edit_sep3: nwg::MenuSeparator,

    #[nwg_control(parent: menu_edit, text: "すべて選択(&A)\tCtrl+A")]
    #[nwg_events(OnMenuItemSelected: [HidemaruClone::select_all])]
    menu_item_select_all: nwg::MenuItem,

    // --- Ruler (Placeholder) ---
    #[nwg_control(text: "....+....1....+....2....+....3....+....4....+....5....+....6....+....7....+....8", h_align: nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    ruler: nwg::Label,

    // --- Line Numbers ---
    #[nwg_control(text: "", flags: "VISIBLE|VSCROLL", readonly: true)]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    line_numbers: nwg::TextBox,

    // --- Editor Area ---
    #[nwg_control(text: "", flags: "VISIBLE|VSCROLL|HSCROLL")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    #[nwg_events(OnTextInput: [HidemaruClone::on_text_changed], OnKeyRelease: [HidemaruClone::on_cursor_move], OnMousePress: [HidemaruClone::on_cursor_move] )]
    text_box: nwg::TextBox,

    // --- Status Bar ---
    #[nwg_control(parent: window)]
    status_bar: nwg::StatusBar,

    // --- Resources ---
    #[nwg_resource(family: "ＭＳ ゴシック", size: 16)]
    editor_font: nwg::Font,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, multiselect: false)]
    file_dialog: nwg::FileDialog,

    #[nwg_resource(title: "Save File", action: nwg::FileDialogAction::Save)]
    save_dialog: nwg::FileDialog,

    #[nwg_control]
    #[nwg_events( OnNotice: [HidemaruClone::on_find_notice] )]
    find_notice: nwg::Notice,

    #[nwg_control]
    #[nwg_events( OnNotice: [HidemaruClone::on_replace_notice] )]
    replace_notice: nwg::Notice,
    
    // --- State ---
    engine: RefCell<TextEngine>,
    find_dialog_ui: RefCell<Option<find_dialog::FindDialogUi>>,
    replace_dialog_ui: RefCell<Option<replace_dialog::ReplaceDialogUi>>,
    scrolling_programmatically: RefCell<bool>,
}

impl HidemaruClone {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn init_ui(&self) {
        // Set fonts
        self.text_box.set_font(Some(&self.editor_font));
        self.ruler.set_font(Some(&self.editor_font));
        self.line_numbers.set_font(Some(&self.editor_font));

        let parts: [i32; 4] = [400, 550, 700, -1];
        unsafe { SendMessageW(self.status_bar.handle.hwnd().unwrap() as _, SB_SETPARTS as u32, parts.len() as _, parts.as_ptr() as _); }

        self.status_bar.set_text(0, " 準備完了");
        
        self.update_line_numbers();
        self.update_cursor_pos_status();
        self.update_info_status();
    }
    
    fn on_text_changed(&self) {
        // Sync to engine with delta calculation
        let new_text = self.text_box.text();
        let mut engine = self.engine.borrow_mut();
        let old_text = engine.get_text();
        
        if old_text != new_text {
            let change = text_engine::TextEngine::compute_delta(&old_text, &new_text);
            engine.apply_change(change, true);
        }

        self.update_line_numbers();
        self.sync_scroll();
        self.update_cursor_pos_status();
    }

    fn on_cursor_move(&self) {
        self.update_cursor_pos_status();
    }
    
    fn update_line_numbers(&self) {
        let line_count = unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_GETLINECOUNT as u32, 0, 0) } as usize;
        let old_line_count = self.line_numbers.text().lines().count();

        if line_count != old_line_count {
            let numbers = (1..=line_count).map(|i| i.to_string()).collect::<Vec<_>>().join("\r\n");
            self.line_numbers.set_text(&numbers);
        }
    }

    fn update_cursor_pos_status(&self) {
        let selection_start = self.text_box.selection().start;

        let line_index = unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_LINEFROMCHAR as u32, selection_start as _, 0) } as usize;
        let line_start_index = unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_LINEINDEX as u32, line_index as _, 0) } as u32;
        
        let col_index = selection_start - line_start_index;

        self.status_bar.set_text(3, &format!("Ln: {}, Col: {}", line_index + 1, col_index + 1));
    }

    fn update_info_status(&self) {
        let engine = self.engine.borrow();
        self.status_bar.set_text(1, &format!(" {}", engine.encoding));
        self.status_bar.set_text(2, &format!(" {}", engine.line_ending));
    }

    fn sync_scroll(&self) {
        *self.scrolling_programmatically.borrow_mut() = true;

        // Vertical scroll
        let first_visible_line = unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_GETFIRSTVISIBLELINE as u32, 0, 0) } as isize;
        let current_line_numbers_scroll = unsafe { SendMessageW(self.line_numbers.handle.hwnd().unwrap() as _, EM_GETFIRSTVISIBLELINE as u32, 0, 0) } as isize;

        let line_delta = first_visible_line - current_line_numbers_scroll;
        if line_delta != 0 {
             unsafe { SendMessageW(self.line_numbers.handle.hwnd().unwrap() as _, EM_LINESCROLL as u32, 0, line_delta as isize) };
        }
        
        // Horizontal scroll sync
        let h_scroll = unsafe { GetScrollPos(self.text_box.handle.hwnd().unwrap() as _, SB_HORZ as i32) };
        // Approximate character width from font size (16px MS Gothic is roughly 8px per half-width char)
        let char_offset = (h_scroll / 8) as usize;
        
        let full_ruler = "....+....1....+....2....+....3....+....4....+....5....+....6....+....7....+....8....+....9....+....0....+....1....+....2....+....3....+....4....+....5....+....6....+....7....+....8";
        if char_offset < full_ruler.len() {
            self.ruler.set_text(&full_ruler[char_offset..]);
        } else {
            self.ruler.set_text("");
        }
        
        *self.scrolling_programmatically.borrow_mut() = false;
    }

    fn new_file(&self) {
        let mut engine = self.engine.borrow_mut();
        *engine = TextEngine::new();
        self.text_box.set_text("");
        self.window.set_text("秀丸エディタ - (無題) [Rust]");
        self.status_bar.set_text(0, "新規作成しました。");
        self.update_line_numbers();
        self.update_cursor_pos_status();
    }
    
    fn open_file(&self, path: &str) {
        let mut engine = self.engine.borrow_mut();
        if let Ok(_) = engine.load_from_file(path) {
            self.text_box.set_text(&engine.get_text());
            self.window.set_text(&format!("秀丸エディタ - {} [Rust]", path));
            self.status_bar.set_text(0, &format!("読み込み完了: {}", path));
            self.update_line_numbers();
            self.sync_scroll();
            self.update_cursor_pos_status();
            self.update_info_status();
        } else {
             nwg::modal_error_message(&self.window, "ファイルエラー", &format!("ファイル {} を開けませんでした。", path));
        }
    }
    
    fn open_file_dialog(&self) {
        if self.file_dialog.run(Some(&self.window)) {
            if let Ok(path) = self.file_dialog.get_selected_item() {
                self.open_file(&path.into_string().unwrap());
            }
        }
    }
    
    fn file_drop(&self, data: &nwg::EventData) {
        if let Some(path) = data.on_file_drop().files().get(0) {
            self.open_file(path);
        }
    }

    fn save_file(&self) {
        // First, sync textbox content to engine
        self.engine.borrow_mut().set_text(&self.text_box.text());

        if let Some(ref path) = self.engine.borrow().file_path.clone() {
            if self.engine.borrow().save_to_file(path).is_ok() {
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
                self.engine.borrow_mut().set_text(&self.text_box.text());
                if self.engine.borrow_mut().save_to_file(&path_str).is_ok() {
                    self.engine.borrow_mut().file_path = Some(path_str.clone());
                    self.window.set_text(&format!("秀丸エディタ - {} [Rust]", path_str));
                    self.status_bar.set_text(0, &format!("名前を付けて保存完了: {}", path_str));
                }
            }
        }
    }

    fn open_find(&self) {
        let mut find_diag_opt = self.find_dialog_ui.borrow_mut();
        if find_diag_opt.is_none() {
            let data = FindDialog::build_ui(Default::default()).expect("Failed to build Find Dialog");
            data.notice_sender.replace(Some(self.find_notice.sender()));
            *find_diag_opt = Some(data);
        }

        if let Some(diag) = find_diag_opt.as_ref() {
            diag.window.set_visible(true);
            diag.window.set_focus();
        }
    }

    fn open_replace(&self) {
        let mut replace_diag_opt = self.replace_dialog_ui.borrow_mut();
        if replace_diag_opt.is_none() {
            let data = ReplaceDialog::build_ui(Default::default()).expect("Failed to build Replace Dialog");
            data.notice_sender.replace(Some(self.replace_notice.sender()));
            *replace_diag_opt = Some(data);
        }

        if let Some(diag) = replace_diag_opt.as_ref() {
            diag.window.set_visible(true);
            diag.window.set_focus();
        }
    }

    fn on_find_notice(&self) {
        let find_diag_ref = self.find_dialog_ui.borrow();
        if let Some(diag) = find_diag_ref.as_ref() {
            let pattern = diag.find_text.text();
            let engine = self.engine.borrow();
            
            let current_sel = self.text_box.selection();
            let start_char = current_sel.end as usize;

            if let Some(result) = engine.find(&pattern, start_char) {
                let char_start = engine.buffer.byte_to_char(result.start_byte) as u32;
                let char_end = engine.buffer.byte_to_char(result.end_byte) as u32;
                self.text_box.set_selection(char_start..char_end);
                self.text_box.set_focus();
                self.status_bar.set_text(0, &format!("見つかりました: {}", pattern));
            } else {
                nwg::modal_info_message(&diag.window, "検索", "これ以上は見つかりませんでした。");
            }
        }
    }

    fn on_replace_notice(&self) {
        let replace_diag_ref = self.replace_dialog_ui.borrow();
        if let Some(diag) = replace_diag_ref.as_ref() {
            let pattern = diag.find_text.text();
            let replacement = diag.replace_text.text();
            let action = *diag.action.borrow();
            
            let mut engine = self.engine.borrow_mut();
            let current_sel = self.text_box.selection();
            
            match action {
                replace_dialog::ReplaceAction::FindNext => {
                    let start_char = current_sel.end as usize;
                    if let Some(result) = engine.find(&pattern, start_char) {
                        let char_start = engine.buffer.byte_to_char(result.start_byte) as u32;
                        let char_end = engine.buffer.byte_to_char(result.end_byte) as u32;
                        self.text_box.set_selection(char_start..char_end);
                        self.text_box.set_focus();
                    } else {
                        nwg::modal_info_message(&diag.window, "検索", "これ以上は見つかりませんでした。");
                    }
                },
                replace_dialog::ReplaceAction::Replace => {
                    let start_char = current_sel.start as usize;
                    if let Some(new_end) = engine.replace_once(&pattern, &replacement, start_char) {
                        self.text_box.set_text(&engine.get_text());
                        self.text_box.set_selection(start_char as u32..new_end as u32);
                        self.text_box.set_focus();
                        self.update_line_numbers();
                    }
                },
                replace_dialog::ReplaceAction::ReplaceAll => {
                    let count = engine.replace_all(&pattern, &replacement);
                    self.text_box.set_text(&engine.get_text());
                    self.update_line_numbers();
                    nwg::modal_info_message(&diag.window, "すべて置換", &format!("{} 箇所置換しました。", count));
                },
                _ => {}
            }
        }
    }

    fn undo(&self) { 
        let mut engine = self.engine.borrow_mut();
        if let Some(_) = engine.undo() {
            self.text_box.set_text(&engine.get_text());
            self.update_line_numbers();
            self.update_cursor_pos_status();
        }
    }
    
    fn redo(&self) { 
        let mut engine = self.engine.borrow_mut();
        if let Some(_) = engine.redo() {
            self.text_box.set_text(&engine.get_text());
            self.update_line_numbers();
            self.update_cursor_pos_status();
        }
    }
    
    fn cut(&self) { 
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_CUT as u32, 0, 0); }
    }
    
    fn copy(&self) { 
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_COPY as u32, 0, 0); }
    }
    
    fn paste(&self) { 
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_PASTE as u32, 0, 0); }
    }
    
    fn delete(&self) { 
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, WM_CLEAR as u32, 0, 0); }
    }
    
    fn select_all(&self) { 
        unsafe { SendMessageW(self.text_box.handle.hwnd().unwrap() as _, EM_SETSEL as u32, 0, -1_isize); }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let app = HidemaruClone::build_ui(Default::default()).expect("Failed to build UI");
    app.init_ui();

    nwg::dispatch_thread_events();
}
