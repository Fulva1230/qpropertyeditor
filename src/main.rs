#![windows_subsystem = "windows"]
use convert_case::{Case, Casing};
use slint::SharedString;
use std::error::Error;
use std::fmt::{format, Debug};
use std::fs;
use std::io::prelude::*;

slint::slint! {
    import { CheckBox, VerticalBox, HorizontalBox, AboutSlint } from "std-widgets.slint";
    import { StandardButton, Button } from "std-widgets.slint";

    component TheMainWindow inherits Window {
        title: "QPropertyEditor";
        default-font-size: 13pt;
        callback save;
        callback generateProperty;
        in property <string> declarationText;
        in property <string> getterText;
        in property <string> setterText;
        in property <string> notifierText;

        out property <string> valueType: typeTextInput.text;
        out property <string> valueName: nameTextInput.text;
        out property <bool> settable: settableCheck.checked;
        out property <bool> notifiable: notifiableCheck.checked;
        out property <bool> constRef: constRefCheck.checked;
        property<length> widthOfDisplay: 80pt;
        max-height: main_layout.min-height;

        MenuBar {
            Menu {
                title: @tr("File");
                MenuItem {
                    title: @tr("Save");
                    activated => { root.save() }
                }
            }

            Menu {
                title: @tr("Help");
                MenuItem {
                    title: @tr("About");
                    activated => { aboutPopup.show() }
                }
            }
        }

        aboutPopup := PopupWindow {
            Rectangle{
                AboutSlint{}
                background: #315afd;
            }
        }

        main_layout := VerticalBox {
            spacing: 5px;
            padding: 5px;

            HorizontalBox {
                settableCheck := CheckBox {
                    text: "Settable";
                    checked: true;
                    toggled => {
                        root.generateProperty();
                    }
                }
                notifiableCheck := CheckBox {
                    text: "Notifiable";
                    checked: true;
                    toggled => {
                        root.generateProperty();
                    }
                }
                constRefCheck := CheckBox {
                    text: "Const ref";
                    checked: true;
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
                    read-only: true;
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
                    read-only: true;
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
                    read-only: true;
                    horizontal-stretch: 1;
                }
            }
            if notifiableCheck.checked: HorizontalBox {
                Text {
                    text: "Notifier: ";
                    min-width: widthOfDisplay;
                }
                TextInput {
                    text: notifierText;
                    read-only: true;
                    horizontal-stretch: 1;
                }
            }
        }
    }

    export {TheMainWindow}
}
struct QProperty {
    the_type: SharedString,
    the_name: SharedString,
    settable: bool,
    notifiable: bool,
    const_ref: bool,
}

impl QProperty {
    fn declaration(&self) -> String {
        let mut str_buf = String::with_capacity(64);
        str_buf.push_str(&std::format!(
            "Q_PROPERTY({} {}",
            self.the_type,
            self.the_name
        ));
        if self.settable {
            str_buf.push_str(&std::format!(
                " WRITE {}",
                std::format!("set_{}", self.the_name).to_case(Case::Camel)
            ));
        }
        if self.notifiable {
            str_buf.push_str(&std::format!(
                " NOTIFY {}",
                std::format!("{}Changed", self.the_name).to_case(Case::Camel)
            ));
        }
        str_buf.push_str(")");
        str_buf
    }

    fn getter(&self) -> String {
        std::format!("{} {}() const;", self.the_type, self.the_name)
    }

    fn parameters(&self) -> String {
        if self.const_ref {
            std::format!("const {} &{}", self.the_type, self.the_name)
        } else {
            std::format!("{} {}", self.the_type, self.the_name)
        }
    }
    fn setter(&self) -> String {
        std::format!(
            "void {}({});",
            std::format!("set_{}", self.the_name).to_case(Case::Camel),
            self.parameters()
        )
    }

    fn notifier(&self) -> String {
        std::format!(
            "void {}({});",
            std::format!("{}_changed", self.the_name).to_case(Case::Camel),
            self.parameters()
        )
    }

    fn summery(&self) -> String {
        std::format!(
            "{}\n\
            {}\n\
            {}\n",
            self.getter(),
            self.setter(),
            self.notifier()
        )
    }
}

fn on_save_callback(main_window: TheMainWindow) {
    if let Ok(mut file) = fs::File::create("save.txt") {
        let qproperty = QProperty {
            the_type: main_window.get_valueType(),
            the_name: main_window.get_valueName(),
            settable: main_window.get_settable(),
            notifiable: main_window.get_notifiable(),
            const_ref: main_window.get_constRef(),
        };
        let summery = qproperty.summery();
        file.write_all(summery.as_bytes()).unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let main_window = TheMainWindow::new()?;

    main_window.on_save({
        let main_window_wk_ref = main_window.as_weak();
        move || {
            if let Some(main_window) = main_window_wk_ref.upgrade() {
                on_save_callback(main_window)
            }
        }
    });
    main_window.on_generateProperty({
        let main_window_wk_ref = main_window.as_weak();
        move || {
            if let Some(main_window) = main_window_wk_ref.upgrade() {
                let qproperty = QProperty {
                    the_type: main_window.get_valueType(),
                    the_name: main_window.get_valueName(),
                    settable: main_window.get_settable(),
                    notifiable: main_window.get_notifiable(),
                    const_ref: main_window.get_constRef(),
                };
                main_window.set_declarationText(qproperty.declaration().into());
                main_window.set_getterText(qproperty.getter().into());
                main_window.set_setterText(qproperty.setter().into());
                main_window.set_notifierText(qproperty.notifier().into());
            }
        }
    });
    main_window.invoke_generateProperty();
    main_window.run()?;
    Ok(())
}
