import QtQuick
import Quickshell.Hyprland
import "../config"
Row {
    spacing: 4
    Repeater {
        model: Hyprland.workspaces
        delegate: Rectangle {
            required property var modelData
            width: modelData.focused ? 36 : 29
            height: 30
            radius: 11
            color: modelData.focused ? Theme.accent : (modelData.urgent ? Theme.danger : "transparent")
            Behavior on width { NumberAnimation { duration: 140; easing.type: Easing.OutCubic } }
            Text { anchors.centerIn: parent; text: modelData.name || modelData.id; color: modelData.focused ? Theme.bg : Theme.fg; font.bold: modelData.focused }
            MouseArea { anchors.fill: parent; onClicked: modelData.activate() }
        }
    }
}