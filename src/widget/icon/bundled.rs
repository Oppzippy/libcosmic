use crate::widget::icon;
use std::sync::LazyLock;

macro_rules! bundled_icon {
    ($icon_name:tt as $function_name:ident) => {
        /// If icons are bundled, returns the bundled icon. Otherwise, the system icon will be used.
        pub fn $function_name() -> icon::Handle {
            static ICON: LazyLock<icon::Handle> = LazyLock::new(|| {
                #[cfg(target_os = "linux")]
                {
                    icon::from_name($icon_name).handle()
                }
                #[cfg(not(target_os = "linux"))]
                {
                    icon::from_svg_bytes(include_bytes!(concat!(
                        "../../../res/icons/",
                        $icon_name,
                        ".svg"
                    )))
                    .symbolic(true)
                }
            });
            ICON.clone()
        }
    };
}

bundled_icon!("pan-down-symbolic" as pan_down_symbolic);
