use qmetaobject::*;
use std::rc::Rc;

// Define the QmlEngine and register the QML code
fn main() {

    let mut engine = QmlEngine::new();
    engine.load_data("
        import QtQuick 2.0;
        import QtQuick.Controls 2.0;
        ApplicationWindow {
            visible: true;
            width: 640;
            height: 480;
            title: qsTr(\"Hello World\");
            Button {
                text: qsTr(\"Press me\");
                anchors.centerIn: parent;
                onClicked: {
                    console.log(\"Button pressed!\");
                }
            }
        }
    ".into());
    engine.exec();
}
