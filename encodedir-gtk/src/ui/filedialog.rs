use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use gtk4::FileChooserDialog;
use gtk4::FileChooserAction;
use gtk4::ResponseType;

pub fn new_filedialog(window: &ApplicationWindow) -> FileChooserDialog {
    let fchoose = FileChooserDialog::new(
        Some("Open directory to encode"),
        Some(window),
        FileChooserAction::SelectFolder,
        &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]
    );

    fchoose
}