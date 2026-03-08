use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use std::cell::RefCell;

#[derive(Default, NwgUi)]
pub struct FindDialog {
    #[nwg_control(size: (300, 100), position: (400, 400), title: "検索", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [FindDialog::close] )]
    pub window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "検索する文字列(&N):")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 2)]
    pub find_text: nwg::TextBox,

    #[nwg_control(text: "次を検索(&F)")]
    #[nwg_layout_item(layout: layout, col: 0, row: 2)]
    #[nwg_events( OnButtonClick: [FindDialog::find_next] )]
    pub btn_next: nwg::Button,

    #[nwg_control(text: "キャンセル")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    #[nwg_events( OnButtonClick: [FindDialog::close] )]
    pub btn_cancel: nwg::Button,

    pub notice_sender: RefCell<Option<nwg::NoticeSender>>,
}

// The NwgUi macro may generate a FindDialogUi struct. 
// We need to make sure it's accessible to main.rs if we store it there.
pub use find_dialog_ui::FindDialogUi;

impl FindDialog {
    fn find_next(&self) {
        if let Some(sender) = self.notice_sender.borrow().as_ref() {
            sender.notice();
        }
    }

    fn close(&self) {
        self.window.set_visible(false);
    }
}
