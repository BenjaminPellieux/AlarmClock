mod view;
mod model;


use qt_widgets::QApplication;
use std::rc::Rc;
use std::cell::RefCell;



fn main() {
    QApplication::init(|_| unsafe {
        let view: Rc<RefCell<view::MyView>> = view::MyView::new();
        view.borrow().update();
        QApplication::exec()
    });
}
