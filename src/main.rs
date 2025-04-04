#![windows_subsystem = "windows"]
use arboard::Clipboard;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io::prelude::*;
use qpropertyeditor::*;

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
