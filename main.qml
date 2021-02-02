import QtQuick 2.12
import QtQuick.Controls 2.5

ApplicationWindow {
    id:root
    width: 320
    height: 640
    visible: true
    title: qsTr("QADBHelper")
    flags: Qt.FramelessWindowHint

    MouseArea{
        anchors.fill: parent
        property int mX: 0
        property int mY: 0
        onPressed: {
            mX = mouseX
            mY = mouseY
        }
        onPositionChanged: {
            root.x+=mouseX-mX
            root.y+=mouseY-mY
        }
    }

    AppHeader{}
}
