pragma Singleton
import QtQuick
import Quickshell.Services.Pipewire
QtObject {
    readonly property var sink: Pipewire.defaultAudioSink
    readonly property bool ready: Pipewire.ready
    readonly property real volume: sink && sink.audio ? sink.audio.volume : 0
    readonly property bool muted: sink && sink.audio ? sink.audio.muted : false
    function setVolume(value) { if (sink && sink.audio) sink.audio.volume = Math.max(0, Math.min(1, value)) }
    function toggleMute() { if (sink && sink.audio) sink.audio.muted = !sink.audio.muted }
}