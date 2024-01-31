use std::error::Error;
use convert_case::{Case, Casing};

slint::slint! {
    import { CheckBox } from "std-widgets.slint";
    export component MainWindow inherits Window{
        title: "QPropertyEditor";
        default-font-size: 13pt;
        callback generateProperty;
        in property <string> declarationText;
        out property <string> valueType: typeTextInput.text;
        out property <string> valueName: nameTextInput.text;
        out property <bool> settable: settableCheck.checked;
        out property <bool> notifiable: notifiableCheck.checked;
        property<length> widthOfDisplay: 80pt;
        max-height: mainLayout.min-height;

        mainLayout := GridLayout {
            spacing: 5px;
            padding: 5px;
            Row {
                settableCheck := CheckBox {
                    text: "Settable";
                    checked: true;
                    toggled => {
                        root.generateProperty()
                    }
                }
                notifiableCheck := CheckBox {
                    text: "Notifiable";
                    checked: true;
                    toggled => {
                        root.generateProperty()
                    }
                }
                Rectangle {
                    horizontal-stretch: 1;
                }
            }
            Row {
                Text {
                    text: "Type: ";
                    min-width: widthOfDisplay;
                }
                typeTextInput := TextInput{
                    text: "bool";
                    horizontal-stretch: 1;
                    edited => {
                        root.generateProperty()
                    }
                }
            }
            Row {
                Text {
                    text: "Name: ";
                    min-width: widthOfDisplay;
                }
                nameTextInput := TextInput{
                    text: "value";
                    horizontal-stretch: 1;
                    edited => {
                        root.generateProperty()
                    }
                }
            }
            Row{
                Text {
                    text: "Declaration: ";
                    min-width: widthOfDisplay;
                }
                TextInput {
                    text: declarationText;
                    read-only: true;
                    horizontal-stretch: 1;
                }
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;
    let main_window_wk_ref = main_window.as_weak();
    main_window.on_generateProperty(move || {
        if let Some(mainWindow) = main_window_wk_ref.upgrade() {
            let the_type = mainWindow.get_valueType();
            let the_name = mainWindow.get_valueName();
            mainWindow.set_declarationText(
                std::format!("Q_PROPERTY({} {} READ {}{}{})",
                             the_type,
                             the_name,
                             the_name,
                             if mainWindow.get_settable() {
                                 std::format!(" WRITE {}", std::format!("set_{}", the_name).to_case(Case::Camel))
                             } else {
                                 String::new()
                             },
                             if mainWindow.get_notifiable() {
                                 std::format!(" NOTIFY {}", std::format!("{}Changed", the_name).to_case(Case::Camel))
                             } else {
                                 String::new()
                             }
                ).into()
            )
        }
    });
    main_window.invoke_generateProperty();
    main_window.run()?;
    Ok(())
}
