pub mod components {
    mod logo;

    pub use logo::LogoComponent;
}

pub mod widgets {
    pub mod coco_help;
    mod coco_textarea;
    mod commit_msg;
    pub use {coco_textarea::LabeledTextArea, commit_msg::CommitMessage};
}
