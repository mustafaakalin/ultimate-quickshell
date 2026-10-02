import QtQuick
import "../config"
import "../services"

Column {
    spacing: 12
    Grid {
        columns: 2; spacing: 8; width: parent.width
        Tile { title: "Audio"; value: AudioService.muted ? "Muted" : Math.round(AudioService.volume * 100) + "%"; action: function() { AudioService.toggleMute() } }
        Tile { title: "Wi-Fi"; value: NetworkService.name; action: function() { NetworkService.toggle() } }
        Tile { title: "Bluetooth"; value: BluetoothService.enabled ? (BluetoothService.connectedCount + " connected") : "Off"; action: function() { BluetoothService.toggle() } }
        Tile { title: "Power"; value: PowerProfileService.name; action: function() { PowerProfileService.cycle() } }
        Tile { title: "Brightness"; value: BrightnessService.available ? BrightnessService.value + "%" : "N/A"; action: function() { BrightnessService.setValue(BrightnessService.value + 10) } }
        Tile { title: "Battery"; value: PowerService.percentage + "%"; action: function() {} }
    }
    Rectangle { width: parent.width; height: 1; color: Theme.bgStrong }
    Text { text: "Connected devices"; color: Theme.fgMuted; font.pixelSize: 12 }
    Column {
        width: parent.width; spacing: 4
        Repeater { model: BluetoothService.devices
            delegate: Text { required property var modelData; text: "󰂱  " + modelData.name + (modelData.batteryAvailable ? " · " + Math.round(modelData.battery * 100) + "%" : ""); color: Theme.fg; visible: modelData.connected }
        }
    }
    component Tile: Rectangle {
        property string title
        property string value
        property var action
        width: (parent.width - 8) / 2; height: 72; radius: 16; color: Theme.bgElevated
        Column { anchors.fill: parent; anchors.margins: 12; spacing: 4
            Text { text: title; color: Theme.fgMuted; font.pixelSize: 11 }
            Text { text: value; color: Theme.fg; font.pixelSize: 15; font.bold: true; elide: Text.ElideRight; width: parent.width }
        }
        MouseArea { anchors.fill: parent; onClicked: action() }
    }
}