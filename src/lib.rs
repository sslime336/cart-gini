use std::sync::atomic::AtomicU32;

/// 色泽
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    /// 青绿
    Green,
    /// 乌黑
    Black,
    /// 浅白
    White,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "青绿" => Self::Green,
            "乌黑" => Self::Black,
            "浅白" => Self::White,
            _ => panic!(),
        }
    }
}

/// 根蒂
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Root {
    /// 蜷缩
    Huddle,
    /// 稍蜷
    SlightHuddle,
    /// 硬挺
    Endure,
}

impl From<&str> for Root {
    fn from(value: &str) -> Self {
        match value {
            "蜷缩" => Self::Huddle,
            "稍蜷" => Self::SlightHuddle,
            "硬挺" => Self::Endure,
            _ => panic!(),
        }
    }
}

/// 敲声
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KnockSound {
    /// 浊响
    Turbidity,
    /// 沉闷
    Dull,
    /// 清脆
    Crisp,
}

impl From<&str> for KnockSound {
    fn from(value: &str) -> Self {
        match value {
            "浊响" => Self::Turbidity,
            "沉闷" => Self::Dull,
            "清脆" => Self::Crisp,
            _ => panic!(),
        }
    }
}

/// 纹理
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Grain {
    /// 清晰
    Clear,
    /// 稍糊
    SlightObscure,
    /// 模糊
    Obscure,
}

impl From<&str> for Grain {
    fn from(value: &str) -> Self {
        match value {
            "清晰" => Self::Clear,
            "稍糊" => Self::SlightObscure,
            "模糊" => Self::Obscure,
            _ => panic!(),
        }
    }
}

// Navel 脐部
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Navel {
    /// 凹陷
    Concavity,
    /// 稍凹
    SlightConcavity,
    /// 平坦
    Flat,
}

impl From<&str> for Navel {
    fn from(value: &str) -> Self {
        match value {
            "凹陷" => Self::Concavity,
            "稍凹" => Self::SlightConcavity,
            "平坦" => Self::Flat,
            _ => panic!(),
        }
    }
}

/// 触感
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Touch {
    /// 硬滑
    Hard,
    /// 软粘
    Soft,
}

impl From<&str> for Touch {
    fn from(value: &str) -> Self {
        match value {
            "硬滑" => Self::Hard,
            "软粘" => Self::Soft,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Melon {
    pub id: String,

    /// 色泽
    pub color: Color,

    /// 根蒂
    pub root: Root,

    /// 敲声
    pub knock_sound: KnockSound,

    /// 纹理
    pub grain: Grain,

    /// 脐部
    pub navel: Navel,

    /// 触感
    pub touch: Touch,

    /// 是否是好瓜
    pub is_good: bool,
}

static mut MELON_ID_ALLOC: AtomicU32 = AtomicU32::new(1);

impl Melon {
    pub fn new(
        color: Color,
        root: Root,
        knock_sound: KnockSound,
        grain: Grain,
        navel: Navel,
        touch: Touch,
        is_good: bool,
    ) -> Self {
        let id =
            unsafe { MELON_ID_ALLOC.fetch_add(1, std::sync::atomic::Ordering::AcqRel) }.to_string();
        Self {
            id,
            color,
            root,
            knock_sound,
            grain,
            navel,
            touch,
            is_good,
        }
    }

    pub fn from_str<C, R, K, G, N, T>(
        color: C,
        root: R,
        knock_sound: K,
        grain: G,
        navel: N,
        touch: T,
        is_good: bool,
    ) -> Self
    where
        C: Into<Color>,
        R: Into<Root>,
        K: Into<KnockSound>,
        G: Into<Grain>,
        N: Into<Navel>,
        T: Into<Touch>,
    {
        let id =
            unsafe { MELON_ID_ALLOC.fetch_add(1, std::sync::atomic::Ordering::AcqRel) }.to_string();
        Self {
            id,
            color: color.into(),
            root: root.into(),
            knock_sound: knock_sound.into(),
            grain: grain.into(),
            navel: navel.into(),
            touch: touch.into(),
            is_good,
        }
    }
}
