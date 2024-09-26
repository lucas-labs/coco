use {rust_i18n::set_locale, sys_locale::get_locale as sys_get_locale};

pub fn setup_locale() {
    set_locale(&get_locale());
}

#[inline]
pub fn get_locale() -> String {
    #[cfg(debug_assertions)]
    {
        if let Ok(locale) = std::env::var("COCO_LOCALE") {
            return locale;
        }
    }

    let locale = sys_get_locale().unwrap_or_else(|| "en".to_string()).replace('_', "-");
    locale.split('-').next().unwrap_or("en").to_string()
}
