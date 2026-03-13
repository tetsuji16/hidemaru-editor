use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use std::cell::RefCell;

#[derive(Default, NwgUi)]
pub struct ReplaceDialog {
    #[nwg_control(size: (400, 150), position: (400, 400), title: "置換", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ReplaceDialog::close] )]
    pub window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "検索する文字列(&N):")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    label_find: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, col_span: 2)]
    pub find_text: nwg::TextBox,

    #[nwg_control(text: "置換後の文字列(&P):")]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    label_replace: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1, col_span: 2)]
    pub replace_text: nwg::TextBox,

    #[nwg_control(text: "次を検索(&F)")]
    #[nwg_layout_item(layout: layout, col: 0, row: 2)]
    #[nwg_events( OnButtonClick: [ReplaceDialog::find_next] )]
    pub btn_next: nwg::Button,

    #[nwg_control(text: "置換(&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    #[nwg_events( OnButtonClick: [ReplaceDialog::replace] )]
    pub btn_replace: nwg::Button,

    #[nwg_control(text: "すべて置換(&A)")]
    #[nwg_layout_item(layout: layout, col: 2, row: 2)]
    #[nwg_events( OnButtonClick: [ReplaceDialog::replace_all] )]
    pub btn_replace_all: nwg::Button,

    pub notice_sender: RefCell<Option<nwg::NoticeSender>>,
    pub action: RefCell<ReplaceAction>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReplaceAction {
    None,
    FindNext,
    Replace,
    ReplaceAll,
}

impl Default for ReplaceAction {
    fn default() -> Self { ReplaceAction::None }
}

pub use replace_dialog_ui::ReplaceDialogUi;

impl ReplaceDialog {
    fn find_next(&self) {
        *self.action.borrow_mut() = ReplaceAction::FindNext;
        if let Some(sender) = self.notice_sender.borrow().as_ref() {
            sender.notice();
        }
    }

    fn replace(&self) {
        *self.action.borrow_mut() = ReplaceAction::Replace;
        if let Some(sender) = self.notice_sender.borrow().as_ref() {
            sender.notice();
        }
    }

    fn replace_all(&self) {
        *self.action.borrow_mut() = ReplaceAction::ReplaceAll;
        if let Some(sender) = self.notice_sender.borrow().as_ref() {
            sender.notice();
        }
    }

    fn close(&self) {
        self.window.set_visible(false);
    }
}
