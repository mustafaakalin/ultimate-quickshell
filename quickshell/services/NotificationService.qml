pragma Singleton
import QtQuick
import Quickshell.Services.Notifications
QtObject {
    id: root
    property int unread: 0
    readonly property var server: serverObject
    NotificationServer {
        id: serverObject
        bodySupported: true
        bodyMarkupSupported: false
        actionsSupported: true
        imageSupported: true
        persistenceSupported: true
        onNotification: notification => { notification.tracked = true; root.unread++ }
    }
    function clearUnread() { unread = 0 }
}