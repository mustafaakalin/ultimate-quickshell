import QtQuick
import QtQuick.Controls
import "../config"
import "../services"
import "ControlPanel.qml"

Rectangle {
    id: island
    required property var root
    color: Theme.bg
    radius: 28
    border.width: 1
    border.color: Theme.bgStrong
    implicitHeight: content.implicitHeight + 32
    Column { id: content; anchors.fill: parent; anchors.margins: 18; spacing: 12
        Row { width: parent.width; spacing: 10
            Text { text: root.islandMode === "launcher" ? "Applications" : root.islandMode === "media" ? "Now Playing" : root.islandMode === "control" ? "Control Center" : "Notifications"; color: Theme.fg; font.pixelSize: 18; font.bold: true }
            Item { width: parent.width - 210; height: 1 }
            Button { text: "×"; onClicked: root.islandOpen = false }
        }
        Loader { width: parent.width; sourceComponent: root.islandMode === "launcher" ? launcher : root.islandMode === "media" ? media : root.islandMode === "control" ? control : notificationList }
    }
    Component { id: launcher; Column { spacing: 12
        TextField { id: input; width: parent.width; placeholderText: "Search applications…"; focus: true; onTextChanged: LauncherService.query = text }
        ListView { width: parent.width; height: 330; clip: true
            model: ScriptModel { values: LauncherService.filtered; objectProp: "id" }
            delegate: Rectangle { required property var modelData; width: ListView.view.width; height: 58; radius: 14; color: mouse.containsMouse ? Theme.bgStrong : "transparent"
                Row { anchors.fill: parent; anchors.margins: 9; spacing: 12
                    Rectangle { width: 40; height: 40; radius: 12; color: Theme.bgElevated; Text { anchors.centerIn: parent; text: modelData.name.charAt(0).toUpperCase(); color: Theme.accent; font.bold: true; font.pixelSize: 17 } }
                    Column { anchors.verticalCenter: parent.verticalCenter; width: parent.width - 52; Text { text: modelData.name; color: Theme.fg; font.bold: true; elide: Text.ElideRight; width: parent.width }; Text { text: modelData.genericName || modelData.comment || modelData.id; color: Theme.fgMuted; font.pixelSize: 11; elide: Text.ElideRight; width: parent.width } }
                }
                MouseArea { id: mouse; anchors.fill: parent; hoverEnabled: true; onClicked: { LauncherService.launch(modelData); root.islandOpen = false } }
            }
        }
    } }
    Component { id: control; ControlPanel { } }
    Component { id: media; Column { spacing: 10
        Text { text: MediaService.activePlayer ? MediaService.activePlayer.trackTitle : "No player"; color: Theme.fg; font.pixelSize: 19; font.bold: true; elide: Text.ElideRight; width: parent.width }
        Text { text: MediaService.activePlayer ? MediaService.activePlayer.trackArtist : ""; color: Theme.fgMuted }
        Row { spacing: 8
            Button { text: "󰒮"; onClicked: if (MediaService.activePlayer) MediaService.activePlayer.previous() }
            Button { text: MediaService.activePlayer && MediaService.activePlayer.isPlaying ? "󰏤" : "󰐊"; onClicked: if (MediaService.activePlayer) MediaService.activePlayer.togglePlaying() }
            Button { text: "󰒭"; onClicked: if (MediaService.activePlayer) MediaService.activePlayer.next() }
        }
    } }
    Component { id: notificationList; Column { spacing: 8
        Row { width: parent.width; Text { text: NotificationService.unread + " unread"; color: Theme.fg; font.bold: true }; Item { width: parent.width - 140; height: 1 }; Button { text: "Clear"; onClicked: NotificationService.clearUnread() } }
        Repeater { model: NotificationService.server.trackedNotifications; delegate: Rectangle { required property var modelData; width: content.width; height: 72; radius: 14; color: Theme.bgElevated
            Column { anchors.fill: parent; anchors.margins: 10; Text { text: modelData.appName + " · " + modelData.summary; color: Theme.fg; font.bold: true; elide: Text.ElideRight; width: parent.width }; Text { text: modelData.body; color: Theme.fgMuted; wrapMode: Text.WordWrap; maximumLineCount: 2; width: parent.width } }
            MouseArea { anchors.fill: parent; onClicked: modelData.dismiss() }
        } }
    } }
}