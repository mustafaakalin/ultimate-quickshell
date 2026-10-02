pragma Singleton
import QtQuick
import Quickshell.Networking

QtObject {
    id: root
    readonly property var devices: Networking.devices
    readonly property var deviceList: devices ? devices.values : []
    readonly property var wifiDevice: findDevice(DeviceType.Wifi)
    readonly property var wiredDevice: findDevice(DeviceType.Wired)
    readonly property var connectedNetwork: findConnectedNetwork()
    readonly property bool connected: !!connectedNetwork || !!(wiredDevice && wiredDevice.connected)
    readonly property string name: connectedNetwork ? connectedNetwork.name : (wiredDevice && wiredDevice.connected ? "Ethernet" : "Offline")

    function findDevice(type) {
        for (const device of deviceList) if (device.type === type) return device
        return null
    }
    function findConnectedNetwork() {
        for (const device of deviceList) {
            const networks = device.networks ? device.networks.values : []
            for (const network of networks) if (network.connected) return network
        }
        return null
    }
    function toggle() {
        if (connectedNetwork) connectedNetwork.disconnect()
    }
}