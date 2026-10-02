import QtQuick
import QtQuick.Controls
import "../config"
import "../services"

Column {
    spacing: 12
    TextField {
        id: search
        width: parent.width
        placeholderText: "Search applications..."
        onTextChanged: LauncherService.query = text
    }
    ListView {
        width: parent.width
        height: 330
        clip: true
        model: ScriptModel { values: LauncherService.filtered; objectProp: "id" }
        delegate: Item {
            required property var modelData
            width: ListView.view.width
            height: 54
            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: modelData.name
                color: Theme.fg
            }
            MouseArea {
                anchors.fill: parent
                onClicked: LauncherService.launch(modelData)
            }
        }
    }
}