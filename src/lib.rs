use convert_case::{Case, Casing};
use slint::SharedString;

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

pub struct QProperty {
    the_type: SharedString,
    the_name: SharedString,
    settable: bool,
    notifiable: bool,
    const_ref: bool,
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

impl QProperty {
    pub fn declaration(&self) -> String {
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

    pub fn getter(&self) -> String {
        std::format!("{} {}() const;", self.the_type, self.the_name)
    }

    pub fn parameters(&self) -> String {
        if self.const_ref {
            std::format!("const {} &{}", self.the_type, self.the_name)
        } else {
            std::format!("{} {}", self.the_type, self.the_name)
        }
    }
    pub fn setter(&self) -> String {
        std::format!(
            "void {}({});",
            std::format!("set_{}", self.the_name).to_case(Case::Camel),
            self.parameters()
        )
    }

    pub fn notifier(&self) -> String {
        std::format!(
            "void {}({});",
            std::format!("{}_changed", self.the_name).to_case(Case::Camel),
            self.parameters()
        )
    }

    pub fn summery(&self) -> String {
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

#[cfg(test)]
mod test{
    #[test]
    fn test_closure() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        fn f<F>(g: F) where F: FnOnce() -> () {
            g();
        }
        let borrows_mutably  = || list.push(7);
        f(borrows_mutably);

        println!("After calling closure: {list:?}");
    }
}

