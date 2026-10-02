pragma Singleton
import QtQuick
import Quickshell.Io

QtObject {
    id: root
    property int value: 0
    property bool available: false

    Process {
        id: readProc
        command: ["brightnessctl", "-m"]
        running: true
        stdout: StdioCollector {
            onStreamFinished: {
                const line = this.text.trim().split("\n").pop()
                const match = line.match(/,(\d+)%/)
                if (match) { root.value = parseInt(match[1]); root.available = true }
            }
        }
    }
    function refresh() { readProc.running = true }
    function setValue(v) {
        if (!root.available) return
        const n = Math.max(1, Math.min(100, Math.round(v)))
        Quickshell.execDetached(["brightnessctl", "set", n + "%"])
        root.value = n
    }
    Timer { interval: 1500; running: true; repeat: true; onTriggered: root.refresh() }
}