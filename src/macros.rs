macro_rules! define_display_macro {
    ($name:ident, $level:ident, $d:tt) => {
        macro_rules! $name {
            ($d($d arg:tt)*) => {
                if log::Level::$level <= *$crate::app::verbosity() {
                    eprintln!($d($d arg)*);
                }
            };
        }
    };
}

define_display_macro!(trace, Trace, $);
define_display_macro!(debug, Debug, $);
define_display_macro!(info, Info, $);
define_display_macro!(warn, Warn, $);
define_display_macro!(error, Error, $);
