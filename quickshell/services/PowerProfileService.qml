pragma Singleton
import QtQuick
import Quickshell.Services.UPower

QtObject {
    id: root
    readonly property var profile: PowerProfiles.profile
    readonly property bool performanceAvailable: PowerProfiles.hasPerformanceProfile
    readonly property string name: root.profile ? PowerProfile.toString(root.profile) : "Unavailable"

    function cycle() {
        if (!root.profile) return
        if (root.profile === PowerProfile.PowerSaver) PowerProfiles.profile = PowerProfile.Balanced
        else if (root.profile === PowerProfile.Balanced && root.performanceAvailable) PowerProfiles.profile = PowerProfile.Performance
        else PowerProfiles.profile = PowerProfile.PowerSaver
    }
}