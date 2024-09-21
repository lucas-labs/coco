pub mod components {
    mod header;

    pub use header::LogoComponent;
}

pub mod widgets {
    mod coco_textarea;
    mod commit_msg;
    pub use {coco_textarea::LabeledTextArea, commit_msg::CommitMessage};
}
