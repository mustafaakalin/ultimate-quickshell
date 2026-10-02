import QtQuick
import "../config"
import "../services"
Column {
    spacing: 0
    Text { anchors.horizontalCenter: parent.horizontalCenter; text: ClockService.time; color: Theme.fg; font.pixelSize: 14; font.bold: true }
    Text { anchors.horizontalCenter: parent.horizontalCenter; text: ClockService.date; color: Theme.fgMuted; font.pixelSize: 10 }
}