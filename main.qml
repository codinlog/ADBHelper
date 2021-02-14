import QtQuick 2.12
import QtQuick.Controls 2.5
import ADBEngine.module 1.0
ApplicationWindow {
    id:root
    width: 320
    height: 640
    visible: true
    title: qsTr("QADBHelper")
    flags: Qt.FramelessWindowHint

    header:AppHeader{
        id:appHeader
        width: parent.width
        height: 30
        MouseArea{
            property int mY: 0
            property int mX: 0
            anchors.fill: parent
            onPressed: {
                mX = mouseX
                mY = mouseY
            }
            onPositionChanged: {
                root.x+=mouseX-mX
                root.y+=mouseY-mY
            }
        }
        onCloseImgClicked: {
            root.close()
        }

        Component.onCompleted: {

        }
    }
    Button{
        id:btn
        text: "click"
        onClicked: {
            helper.tag = "Hello World!";
            helper.tagChanged();
        }
    }


    Component.onCompleted: {
        console.log(ADBEngine.mADBPath)

    }
}
