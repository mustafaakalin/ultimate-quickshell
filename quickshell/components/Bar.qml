import QtQuick
import QtQuick.Layouts
import Quickshell
import "../config"
import "../services"

Rectangle {
    id: bar
    required property ShellScreen screen
    required property var root
    color: "transparent"
    RowLayout {
        anchors.fill: parent; anchors.margins: 7; spacing: Theme.gap
        Rectangle { Layout.preferredWidth: workspaces.implicitWidth + 18; Layout.fillHeight: true; radius: Theme.radius; color: Theme.bgElevated; Workspaces { id: workspaces; anchors.centerIn: parent } }
        Rectangle { Layout.preferredWidth: 260; Layout.fillHeight: true; radius: Theme.radius; color: Theme.bgElevated
            Text { anchors.fill: parent; anchors.margins: 12; verticalAlignment: Text.AlignVCenter; elide: Text.ElideRight; color: Theme.fgMuted; text: HyprlandService.activeToplevel ? HyprlandService.activeToplevel.title : "Desktop" }
        }
        Item { Layout.fillWidth: true }
        Rectangle { Layout.preferredWidth: 210; Layout.fillHeight: true; radius: Theme.radius; color: Theme.bgElevated
            Row { anchors.centerIn: parent; spacing: 10
                Text { text: NetworkService.connected ? "󰤨" : "󰤭"; color: NetworkService.connected ? Theme.accent : Theme.fgMuted; font.pixelSize: 16 }
                Text { text: BluetoothService.connected ? "󰂱" : "󰂲"; color: BluetoothService.connected ? Theme.accent : Theme.fgMuted; font.pixelSize: 16 }
                Text { text: AudioService.muted ? "󰝟" : "󰕾"; color: Theme.fg; font.pixelSize: 16 }
                Text { text: Math.round(AudioService.volume * 100) + "%"; color: Theme.fg }
                Text { text: PowerService.charging ? "󰂄" : "󰁹"; color: Theme.fg; font.pixelSize: 16 }
                Text { text: PowerService.percentage + "%"; color: Theme.fg }
            }
        }
        Rectangle { Layout.preferredWidth: Math.max(150, tray.implicitWidth + 28); Layout.fillHeight: true; radius: Theme.radius; color: Theme.bgElevated
            SystemTray { id: tray; anchors.centerIn: parent }
        }
        Rectangle { Layout.preferredWidth: 135; Layout.fillHeight: true; radius: Theme.radius; color: Theme.bgElevated; Clock { anchors.centerIn: parent } }
    }
}