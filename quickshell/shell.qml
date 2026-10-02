import QtQuick
import Quickshell
import Quickshell.Hyprland
import Quickshell.Io
import "config"
import "components"
import "services"

ShellRoot {
    id: root
    property bool islandOpen: false
    property string islandMode: "launcher"

    Variants {
        model: Quickshell.screens
        delegate: Component {
            PanelWindow {
                required property ShellScreen modelData
                screen: modelData
                color: "transparent"
                anchors.top: true; anchors.left: true; anchors.right: true
                implicitHeight: Theme.barHeight
                exclusiveZone: Theme.barHeight
                aboveWindows: true
                Bar { anchors.fill: parent; screen: modelData; root: root }
            }
        }
    }

    PanelWindow {
        visible: root.islandOpen
        screen: Quickshell.screens.length ? Quickshell.screens[0] : null
        color: "transparent"
        anchors.top: true; anchors.left: true; anchors.right: true
        implicitHeight: 520
        aboveWindows: true
        Island { anchors.horizontalCenter: parent.horizontalCenter; anchors.top: parent.top; width: Math.min(parent.width - 48, 820); root: root }
    }

    IpcHandler {
        target: "ultimate"
        function launcher() { root.islandMode = "launcher"; LauncherService.query = ""; root.islandOpen = true }
        function notifications() { root.islandMode = "notifications"; NotificationService.clearUnread(); root.islandOpen = true }
        function control() { root.islandMode = "control"; root.islandOpen = true }
        function media() { root.islandMode = "media"; root.islandOpen = true }
        function close() { root.islandOpen = false }
        function reloadShell() { Quickshell.reload(false) }
    }

    GlobalShortcut { name: "ultimate-launcher"; description: "Open Ultimate launcher"; onPressed: { root.islandMode = "launcher"; LauncherService.query = ""; root.islandOpen = true } }
    GlobalShortcut { name: "ultimate-notifications"; description: "Open notifications"; onPressed: { root.islandMode = "notifications"; NotificationService.clearUnread(); root.islandOpen = true } }
    GlobalShortcut { name: "ultimate-control"; description: "Open control center"; onPressed: { root.islandMode = "control"; root.islandOpen = true } }
    GlobalShortcut { name: "ultimate-media"; description: "Open media"; onPressed: { root.islandMode = "media"; root.islandOpen = true } }
    GlobalShortcut { name: "ultimate-close"; description: "Close Ultimate overlay"; onPressed: root.islandOpen = false }
}