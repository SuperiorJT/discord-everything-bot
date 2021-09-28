use std::fmt::Display;

use twilight_model::{
    channel::message::sticker::StickerId,
    id::{ApplicationId, EmojiId, GuildId, UserId},
    oauth::id::TeamId,
};

pub const BASE_URL: &str = "https://cdn.discordapp.com/";

macro_rules! cdn_format {
    ($path:literal, $p1:ident) => {
        fn format(&self) -> String {
            format!($path, BASE_URL, self.0)
        }
    };
    ($path:literal, $p1:ident, $p2:ident) => {
        fn format(&self) -> String {
            format!($path, BASE_URL, self.0, self.1)
        }
    };
    ($path:literal, $p1:ident, $p2:ident, $p3:ident) => {
        fn format(&self) -> String {
            format!($path, BASE_URL, self.0, self.1, self.2)
        }
    };
}

macro_rules! cdn_struct {
    ($s:ident, $param:ident) => {
        pub struct $s(pub $param);
    };
    ($s:ident, $param1:ident, $param2:ident) => {
        pub struct $s(pub $param1, pub $param2);
    };
    ($s:ident, $param1:ident, $param2:ident, $param3:ident) => {
        pub struct $s(pub $param1, pub $param2, pub $param3);
    };
}

macro_rules! impl_cdn_supports {
    ($s: ident, $sup:ident) => {
        impl $sup for $s {}
    };
    ($s: ident, $sup1:ident, $sup2:ident) => {
        impl $sup1 for $s {}
        impl $sup2 for $s {}
    };
    ($s: ident, $sup1:ident, $sup2:ident, $sup3:ident) => {
        impl $sup1 for $s {}
        impl $sup2 for $s {}
        impl $sup3 for $s {}
    };
    ($s: ident, $sup1:ident, $sup2:ident, $sup3:ident, $sup4:ident) => {
        impl $sup1 for $s {}
        impl $sup2 for $s {}
        impl $sup3 for $s {}
        impl $sup4 for $s {}
    };
}

macro_rules! cdn_endpoint {
    ($s:ident, $($param:ident),+, $path:literal, $($sup:ident),+) => {
        cdn_struct!($s, $($param),+);
        impl UrlString for $s {
            cdn_format!($path, $($param),+);
        }
        impl_cdn_supports!($s, $($sup),+);
        impl Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // All formats currently support png, so default to that
                f.write_str(self.as_png().as_str())
            }
        }
    };
}

pub trait UrlString {
    fn format(&self) -> String;
}

pub trait SupportsPng: UrlString {
    fn as_png(&self) -> String {
        self.format() + ".png"
    }
}
pub trait SupportsJpeg: UrlString {
    fn as_jpeg(&self) -> String {
        self.format() + "jpeg"
    }
}
pub trait SupportsWebp: UrlString {
    fn as_webp(&self) -> String {
        self.format() + "webp"
    }
}
pub trait SupportsGif: UrlString {
    fn as_gif(&self) -> String {
        self.format() + "gif"
    }
}
pub trait SupportsLottie: UrlString {
    fn as_lottie(&self) -> String {
        self.format() + "json"
    }
}

cdn_endpoint!(
    EmojiUrl,
    EmojiId,
    "{}emojis/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp,
    SupportsGif
);

cdn_endpoint!(
    GuildIconUrl,
    GuildId,
    String,
    "{}icons/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp,
    SupportsGif
);

cdn_endpoint!(
    GuildSplashUrl,
    GuildId,
    String,
    "{}splashes/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    GuildDiscoverySplashUrl,
    GuildId,
    String,
    "{}discovery-splashes/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    GuildBannerUrl,
    GuildId,
    String,
    "{}banners/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    UserBannerUrl,
    UserId,
    String,
    "{}banners/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp,
    SupportsGif
);

cdn_endpoint!(
    DefaultUserAvatarUrl,
    String,
    "{}embed/avatars/{}",
    SupportsPng
);

cdn_endpoint!(
    UserAvatarUrl,
    UserId,
    String,
    "{}avatars/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp,
    SupportsGif
);

cdn_endpoint!(
    ApplicationIconUrl,
    ApplicationId,
    String,
    "{}app-icons/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    ApplicationCoverUrl,
    ApplicationId,
    String,
    "{}app-icons/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    ApplicationAssetUrl,
    ApplicationId,
    String,
    "{}app-assets/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    AchievementIconUrl,
    ApplicationId,
    String,
    String,
    "{}app-assets/{}/achievements/{}/icons/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    StickerPackBannerUrl,
    String,
    "{}app-assets/710982414301790216/store/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    TeamIconUrl,
    TeamId,
    String,
    "{}team-icons/{}/{}",
    SupportsPng,
    SupportsJpeg,
    SupportsWebp
);

cdn_endpoint!(
    StickerUrl,
    StickerId,
    "{}stickers/{}",
    SupportsPng,
    SupportsLottie
);
