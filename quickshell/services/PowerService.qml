pragma Singleton
import QtQuick
import Quickshell.Services.UPower
QtObject {
    readonly property var device: UPower.displayDevice
    readonly property int percentage: device ? Math.round(device.percentage) : 0
    readonly property bool charging: device ? (device.state === UPowerDeviceState.Charging || device.state === UPowerDeviceState.PendingCharge) : false
    readonly property bool onBattery: UPower.onBattery
}