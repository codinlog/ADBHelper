import QtQuick 2.0

Item {
    id:root
    signal closeImgClicked
    Rectangle{
        z:1
        id:container
        width: parent.width
        height: parent.height
        color: "#088A68"
        Text {
            id: topTitle
            color: "#000000"
            text: qsTr("QADBHelper")
            font.bold: true
            anchors{
                left:container.left
                margins:{
                    left: 10
                }
                verticalCenter: parent.verticalCenter
            }
        }
        Rectangle{
            id:closeRect
            width:20
            height:20
            radius: 10
            property string hoveredColor: "#FF0000"
            property string normalColor: "#FA5858"
            color: normalColor
            anchors{
                right:container.right
                verticalCenter: parent.verticalCenter
                margins:{
                    top:5
                }
            }
            Image {
                id: closeImg
                source: "qrc:/close"
                width: 16
                height: 16
                anchors.centerIn: parent
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
                    console.log("close")
                    root.closeImgClicked()
                }
            }
        }
    }
}
