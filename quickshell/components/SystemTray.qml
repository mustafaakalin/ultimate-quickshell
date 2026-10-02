import QtQuick
import Quickshell.Services.SystemTray
import "../config"
import "../services"

Row {
    spacing: 4
    Repeater {
        model: TrayService.items
        delegate: Rectangle {
            required property var modelData
            width: 30; height: 30; radius: 9
            color: mouse.containsMouse ? Theme.bgStrong : "transparent"
            Image { anchors.centerIn: parent; width: 18; height: 18; source: modelData.icon; sourceSize.width: 36; sourceSize.height: 36; smooth: true }
            MouseArea {
                id: mouse; anchors.fill: parent; hoverEnabled: true
                onClicked: modelData.activate()
                onPressAndHold: modelData.secondaryActivate()
            }
        }
    }
}