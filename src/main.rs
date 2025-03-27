#![windows_subsystem = "windows"]
use arboard::Clipboard;
use convert_case::{Case, Casing};
use slint::SharedString;
use std::error::Error;
use std::fmt::{Debug, format};
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
        callback copyDeclaration;
        callback copyGetter;
        callback copySetter;
        callback copyNotifier;
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
                Button {
                    text: "📑";
                    clicked() => {
                        root.copyDeclaration()
                    }
                }
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
                Button {
                    text: "📑";
                    clicked() => {
                        root.copyGetter()
                    }
                }
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
                Button {
                    text: "📑";
                    clicked() => {
                        root.copySetter()
                    }
                }
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
                Button {
                    text: "📑";
                    clicked() => {
                        root.copyNotifier()
                    }
                }
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
            {}\n\
            {}\n",
            self.declaration(),
            self.getter(),
            self.setter(),
            self.notifier()
        )
    }
}

impl From<&TheMainWindow> for QProperty {
    fn from(value: &TheMainWindow) -> Self {
        QProperty {
            the_type: value.get_valueType(),
            the_name: value.get_valueName(),
            settable: value.get_settable(),
            notifiable: value.get_notifiable(),
            const_ref: value.get_constRef(),
        }
    }
}

fn on_save_callback(main_window: TheMainWindow) {
    if let Ok(mut file) = fs::File::create("save.txt") {
        let qproperty: QProperty = (&main_window).into();
        file.write_all(qproperty.summery().as_bytes()).unwrap()
    }
}

fn on_generate_property(main_window: TheMainWindow) {
    let qproperty: QProperty = (&main_window).into();
    main_window.set_declarationText(qproperty.declaration().into());
    main_window.set_getterText(qproperty.getter().into());
    main_window.set_setterText(qproperty.setter().into());
    main_window.set_notifierText(qproperty.notifier().into());
}
fn on_copy_declaration(main_window: TheMainWindow) -> Result<(), Box<dyn Error>> {
    let qproperty: QProperty = (&main_window).into();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(qproperty.declaration())?;
    Ok(())
}

fn on_copy_getter(main_window: TheMainWindow) -> Result<(), Box<dyn Error>> {
    let qproperty: QProperty = (&main_window).into();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(qproperty.getter())?;
    Ok(())
}

fn on_copy_setter(main_window: TheMainWindow) -> Result<(), Box<dyn Error>> {
    let qproperty: QProperty = (&main_window).into();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(qproperty.setter())?;
    Ok(())
}

fn on_copy_notifier(main_window: TheMainWindow) -> Result<(), Box<dyn Error>> {
    let qproperty: QProperty = (&main_window).into();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(qproperty.notifier())?;
    Ok(())
}

fn callback<T>(
    main_window: &TheMainWindow,
    functor: impl Fn(TheMainWindow) -> T + 'static
) -> impl FnMut() -> () + 'static{
    let main_window_wk_ref = main_window.as_weak();
    move || {
        if let Some(main_window) = main_window_wk_ref.upgrade() {
            functor(main_window);
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let main_window = TheMainWindow::new()?;
    main_window.on_save(callback(&main_window, on_save_callback));
    main_window.on_generateProperty(callback(&main_window, on_generate_property));
    main_window.on_copyDeclaration(callback(&main_window, on_copy_declaration));
    main_window.on_copyGetter(callback(&main_window, on_copy_getter));
    main_window.on_copySetter(callback(&main_window, on_copy_setter));
    main_window.on_copyNotifier(callback(&main_window, on_copy_notifier));
    main_window.invoke_generateProperty();
    main_window.run()?;
    Ok(())
}
