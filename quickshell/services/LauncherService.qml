pragma Singleton
import QtQuick
import Quickshell

QtObject {
    id: root
    readonly property var applications: DesktopEntries.applications
    property string query: ""
    readonly property var filtered: {
        const q = root.query.trim().toLowerCase()
        if (!q) return root.applications.values.slice(0, 80)
        return root.applications.values.filter(app => {
            const hay = [app.name, app.genericName, app.comment, ...(app.keywords || [])].join(" ").toLowerCase()
            return hay.includes(q)
        }).slice(0, 80)
    }
    function launch(entry) { if (entry) entry.execute() }
}