pragma Singleton
import QtQuick
import Quickshell.Hyprland
QtObject {
    readonly property var workspaces: Hyprland.workspaces
    readonly property var focusedWorkspace: Hyprland.focusedWorkspace
    readonly property var focusedMonitor: Hyprland.focusedMonitor
    readonly property var activeToplevel: Hyprland.activeToplevel
    function workspace(id) { for (const ws of Hyprland.workspaces.values) if (ws.id === id) return ws; return null }
}