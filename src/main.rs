use std::error::Error;

slint::slint! {
    export component MainWindow inherits Window{
        title: "QPropertyEditor";
        default-font-size: 13pt;
        callback generateProperty;
        in property <string> declarationText;
        out property <string> valueType: typeTextInput.text;
        out property <string> valueName: nameTextInput.text;
        property<length> widthOfDisplay: 80pt;
        max-height: mainLayout.min-height;

        mainLayout := GridLayout {
            spacing: 5px;
            padding: 5px;
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
                Text {
                    text: declarationText;
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
            mainWindow.set_declarationText(
                std::format!("Q_PROPERTY({} {})",
                             mainWindow.get_valueType(),
                             mainWindow.get_valueName()).into())
        }
    });
    main_window.invoke_generateProperty();
    main_window.run()?;
    Ok(())
}
