pragma Singleton
import QtQuick
import Quickshell.Services.Mpris
QtObject {
    id: root
    property var activePlayer: null
    readonly property var players: Mpris.players
    function choosePlayer() {
        const values = Mpris.players.values
        for (const p of values) if (p.isPlaying) { root.activePlayer = p; return }
        root.activePlayer = values.length ? values[0] : null
    }
    Instantiator {
        model: Mpris.players
        delegate: QtObject {
            required property var modelData
            Connections {
                target: modelData
                function onIsPlayingChanged() { root.choosePlayer() }
                function onTrackTitleChanged() { if (root.activePlayer === modelData) root.activePlayer = modelData }
            }
        }
        onObjectAdded: root.choosePlayer()
        onObjectRemoved: root.choosePlayer()
    }
    Component.onCompleted: choosePlayer()
}