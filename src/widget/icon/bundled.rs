use crate::widget::icon;

#[cfg(not(target_os = "linux"))]
pub struct BundledIcon(icon::Handle);

#[cfg(not(target_os = "linux"))]
impl BundledIcon {
    pub fn fallback(self, fallback: Option<icon::IconFallback>) -> Self {
        self
    }

    pub fn scale(self, scale: u16) -> Self {
        self
    }

    pub fn size(self, size: u16) -> Self {
        self
    }

    pub fn symbolic(self, symbolic: bool) -> Self {
        Self(self.0.symbolic(symbolic))
    }

    pub fn prefer_svg(self, prefer_svg: bool) -> Self {
        self
    }

    pub fn handle(self) -> icon::Handle {
        self.0
    }

    pub fn icon(self) -> icon::Icon {
        self.0.icon()
    }
}

#[cfg(target_os = "linux")]
pub struct BundledIcon(icon::Named);

#[cfg(target_os = "linux")]
impl BundledIcon {
    pub fn fallback(self, fallback: Option<icon::IconFallback>) -> Self {
        Self(self.0.fallback(fallback))
    }

    pub fn scale(self, scale: u16) -> Self {
        Self(self.0.scale(scale))
    }

    pub fn size(self, size: u16) -> Self {
        Self(self.0.size(size))
    }

    pub fn symbolic(self, symbolic: bool) -> Self {
        Self(self.0.symbolic(symbolic))
    }

    pub fn prefer_svg(self, prefer_svg: bool) -> Self {
        Self(self.0.prefer_svg(prefer_svg))
    }

    pub fn handle(self) -> icon::Handle {
        self.0.handle()
    }

    pub fn icon(self) -> icon::Icon {
        self.0.icon()
    }
}

macro_rules! bundled_icon {
    ($icon_name:tt as $function_name:ident) => {
        /// If icons are bundled, returns the bundled icon. Otherwise, the system icon will be used.
        pub fn $function_name() -> BundledIcon {
            #[cfg(target_os = "linux")]
            {
                BundledIcon(icon::from_name($icon_name))
            }
            #[cfg(not(target_os = "linux"))]
            {
                static ICON: std::sync::LazyLock<icon::Handle> = std::sync::LazyLock::new(|| {
                    icon::from_svg_bytes(include_bytes!(concat!(
                        "../../../res/icons/",
                        $icon_name,
                        ".svg"
                    )))
                    .symbolic(true)
                });
                BundledIcon(ICON.clone())
            }
        }
    };
}

bundled_icon!("pan-down-symbolic" as pan_down_symbolic);
