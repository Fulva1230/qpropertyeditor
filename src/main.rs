use convert_case::{Case, Casing};
use std::error::Error;

slint::slint! {
    import { CheckBox, VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component TheMainWindow inherits Window {
        title: "QPropertyEditor";
        default-font-size: 13pt;
        callback generateProperty;
        in property <bool> trueProp;
        in property <string> declarationText;
        in property <string> getterText;
        in property <string> setterText;

        out property <string> valueType: typeTextInput.text;
        out property <string> valueName: nameTextInput.text;
        out property <bool> settable: settableCheck.checked;
        out property <bool> notifiable: notifiableCheck.checked;
        property<length> widthOfDisplay: 80pt;
        max-height: main_layout.min-height;

        main_layout := VerticalBox {
            spacing: 5px;
            padding: 5px;
            HorizontalBox {
                settableCheck := CheckBox {
                    text: "Settable";
                    checked: trueProp;
                    toggled => {
                        root.generateProperty();
                    }
                }
                notifiableCheck := CheckBox {
                    text: "Notifiable";
                    checked: trueProp;
                    toggled => {
                        root.generateProperty();
                    }
                }
                Rectangle {
                    horizontal-stretch: 1;
                }
            }

            HorizontalBox {
                Text {
                    text: "Type: ";
                    min-width: widthOfDisplay;
                }
                typeTextInput := TextInput{
                    text: "bool";
                    horizontal-stretch: 1;
                    edited => {
                        root.generateProperty();
                    }
                }
            }
            HorizontalBox {
                Text {
                    text: "Name: ";
                    min-width: widthOfDisplay;
                }
                nameTextInput := TextInput{
                    text: "value";
                    horizontal-stretch: 1;
                    edited => {
                        root.generateProperty();
                    }
                }
            }
            HorizontalBox {
                Text {
                    text: "Declaration: ";
                    min-width: widthOfDisplay;
                }
                TextInput {
                    text: declarationText;
                    read-only: trueProp;
                    horizontal-stretch: 1;
                }
            }
            HorizontalBox {
                Text {
                    text: "Getter: ";
                    min-width: widthOfDisplay;
                }
                TextInput {
                    text: getterText;
                    read-only: trueProp;
                    horizontal-stretch: 1;
                }
            }
            if settableCheck.checked: HorizontalBox {
                Text {
                    text: "Setter: ";
                    min-width: widthOfDisplay;
                }
                TextInput {
                    text: setterText;
                    read-only: trueProp;
                    horizontal-stretch: 1;
                }
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let main_window = TheMainWindow::new()?;
    main_window.set_trueProp(true);
    let main_window_wk_ref = main_window.as_weak();
    main_window.on_generateProperty(move || {
        if let Some(main_window) = main_window_wk_ref.upgrade() {
            let the_type = main_window.get_valueType();
            let the_name = main_window.get_valueName();
            main_window.set_declarationText(
                std::format!(
                    "Q_PROPERTY({} {} READ {}{}{})",
                    the_type,
                    the_name,
                    the_name,
                    if main_window.get_settable() {
                        std::format!(
                            " WRITE {}",
                            std::format!("set_{}", the_name).to_case(Case::Camel)
                        )
                    } else {
                        String::new()
                    },
                    if main_window.get_notifiable() {
                        std::format!(
                            " NOTIFY {}",
                            std::format!("{}Changed", the_name).to_case(Case::Camel)
                        )
                    } else {
                        String::new()
                    }
                )
                    .into(),
            );
            main_window.set_getterText(
                std::format!(
                    "{} {}() const;",
                    the_type,
                    the_name
                )
                    .into(),
            );
            main_window.set_setterText(
                std::format!(
                    "void {}({});",
                    std::format!("set_{}", the_name).to_case(Case::Camel),
                    std::format!("{} {}", the_type, the_name)
                )
                    .into(),
            );
        }
    });
    main_window.invoke_generateProperty();
    main_window.run()?;
    Ok(())
}


