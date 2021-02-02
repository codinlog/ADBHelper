import QtQuick 2.0

Item {
    id:appHeader
    width: parent.width
    height: 30
    Rectangle{
        id:container
        width: appHeader.width
        height: appHeader.height
        color: "#009966"
        Text {
            id: topTitle
            color: "#ffffff"
            text: qsTr("QADBHelper")
            font.bold: true
            anchors{
                top: container.top
                left:container.left
                margins: {
                    left:5
                    top:5
                }
            }
        }
        Rectangle{
            id:closeRect
            width:16
            height:16
            radius: 8
            property string hoveredColor: "#FF0000"
            property string normalColor: "#FA5858"
            color: normalColor
            anchors{
                right:container.right
                top:container.top
                margins: {
                    top:5
                    right:5
                }
            }
            Image {
                id: closeImg
                source: "qrc:/close"
                width: 16
                height: 16
            }
            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onEntered: {
                    closeRect.color = closeRect.hoveredColor
                }
                onExited: {
                    closeRect.color = closeRect.normalColor
                }
                onClicked: {
                    root.close()
                }
            }
        }
    }



}
