pragma Singleton
import QtQuick
QtObject {
    property string time: Qt.formatDateTime(new Date(), "HH:mm")
    property string date: Qt.formatDateTime(new Date(), "ddd, dd MMM")
    Timer { interval: 1000; running: true; repeat: true; onTriggered: { time = Qt.formatDateTime(new Date(), "HH:mm"); date = Qt.formatDateTime(new Date(), "ddd, dd MMM") } }
}