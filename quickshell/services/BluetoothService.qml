pragma Singleton
import QtQuick
import Quickshell.Bluetooth

QtObject {
    id: root
    readonly property var adapter: Bluetooth.defaultAdapter
    readonly property var devices: Bluetooth.devices
    readonly property bool available: !!adapter
    readonly property bool enabled: !!adapter && adapter.enabled
    readonly property bool connected: adapter ? adapter.devices.values.some(device => device.connected) : false
    readonly property int connectedCount: adapter ? adapter.devices.values.filter(device => device.connected).length : 0

    function toggle() { if (adapter) adapter.enabled = !adapter.enabled }
}