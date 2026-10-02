pragma Singleton
import Quickshell.Services.SystemTray
import QtQuick

QtObject {
    readonly property var items: SystemTray.items
}