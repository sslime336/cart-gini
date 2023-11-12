//! CART 算法生成决策树

#![feature(lazy_cell)]

use std::sync::LazyLock;

use cart_gini::{Color, Grain, KnockSound, Melon, Navel, Root, Touch};
use lazy_static::lazy_static;

lazy_static! {
    static ref DS: LazyLock<Vec<Melon>> = LazyLock::new(|| {
        vec![
            // Good
            Melon::from_str("青绿", "蜷缩", "浊响", "清晰", "凹陷", "硬滑", true),
            Melon::from_str("乌黑", "蜷缩", "沉闷", "清晰", "凹陷", "硬滑", true),
            Melon::from_str("乌黑", "蜷缩", "浊响", "清晰", "凹陷", "硬滑", true),
            Melon::from_str("青绿", "蜷缩", "沉闷", "清晰", "凹陷", "硬滑", true),
            Melon::from_str("浅白", "蜷缩", "浊响", "清晰", "凹陷", "硬滑", true),
            Melon::from_str("青绿", "稍蜷", "浊响", "清晰", "稍凹", "软粘", true),
            Melon::from_str("乌黑", "稍蜷", "浊响", "稍糊", "稍凹", "软粘", true),
            Melon::from_str("乌黑", "稍蜷", "浊响", "清晰", "稍凹", "硬滑", true),
            // Bad
            Melon::from_str("乌黑", "稍蜷", "沉闷", "稍糊", "稍凹", "硬滑", false),
            Melon::from_str("青绿", "硬挺", "清脆", "清晰", "平坦", "软粘", false),
            Melon::from_str("浅白", "硬挺", "清脆", "模糊", "平坦", "硬滑", false),
            Melon::from_str("浅白", "蜷缩", "浊响", "模糊", "平坦", "软粘", false),
            Melon::from_str("青绿", "稍蜷", "浊响", "清晰", "凹陷", "硬滑", false),
            Melon::from_str("浅白", "稍蜷", "沉闷", "稍糊", "凹陷", "硬滑", false),
            Melon::from_str("乌黑", "稍蜷", "浊响", "稍糊", "稍凹", "软粘", false),
            Melon::from_str("浅白", "蜷缩", "浊响", "模糊", "平坦", "硬滑", false),
            Melon::from_str("青绿", "蜷缩", "沉闷", "稍糊", "凹陷", "硬滑", false),
        ]
    });
}

fn main() {
    apart(&DS);
}

fn color_gini_rate(v: &Vec<Melon>) -> f32 {
    let black = calculate_gini_rate_color(v, Color::Black);
    let green = calculate_gini_rate_color(v, Color::Green);
    let white = calculate_gini_rate_color(v, Color::White);

    let color = black + green + white;
    dbg!(color);
    color
}
fn grain_gini_rate(v: &Vec<Melon>) -> f32 {
    let clear = calculate_gini_rate_grain(v, Grain::Clear);
    let obscure = calculate_gini_rate_grain(v, Grain::Obscure);
    let slight_obscure = calculate_gini_rate_grain(v, Grain::SlightObscure);

    let grain = clear + obscure + slight_obscure;
    dbg!(grain);
    grain
}
fn knock_sound_gini_rate(v: &Vec<Melon>) -> f32 {
    let crisp = calculate_gini_rate_knock_sound(v, KnockSound::Crisp);
    let dull = calculate_gini_rate_knock_sound(v, KnockSound::Dull);
    let turbidity = calculate_gini_rate_knock_sound(v, KnockSound::Turbidity);

    let knock_sound = crisp + dull + turbidity;
    dbg!(knock_sound);
    knock_sound
}
fn navel_gini_rate(v: &Vec<Melon>) -> f32 {
    let concavity = calculate_gini_rate_navel(v, Navel::Concavity);
    let flat = calculate_gini_rate_navel(v, Navel::Flat);
    let slight_concavity = calculate_gini_rate_navel(v, Navel::SlightConcavity);

    let navel = concavity + flat + slight_concavity;
    dbg!(navel);
    navel
}
fn root_gini_rate(v: &Vec<Melon>) -> f32 {
    let endure = calculate_gini_rate_root(v, Root::Endure);
    let huddle = calculate_gini_rate_root(v, Root::Huddle);
    let slight_huddle = calculate_gini_rate_root(v, Root::SlightHuddle);

    let root = endure + huddle + slight_huddle;
    dbg!(root);
    root
}
fn touch_gini_rate(v: &Vec<Melon>) -> f32 {
    let hard = calculate_gini_rate_touch(v, Touch::Hard);
    let soft = calculate_gini_rate_touch(v, Touch::Soft);

    let touch = hard + soft;
    dbg!(touch);
    touch
}

fn apart(v: &Vec<Melon>) {
    color_gini_rate(v);
    grain_gini_rate(v);
    knock_sound_gini_rate(v);
    navel_gini_rate(v);
    root_gini_rate(v);
    touch_gini_rate(v);
}

/// 计算给定`色泽`基尼值
fn calculate_gini_rate_color(v: &Vec<Melon>, color: Color) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.color == color {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
/// 计算给定`根蒂`基尼值
fn calculate_gini_rate_root(v: &Vec<Melon>, root: Root) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.root == root {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
/// 计算给定`敲声`基尼值
fn calculate_gini_rate_knock_sound(v: &Vec<Melon>, knock_sound: KnockSound) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.knock_sound == knock_sound {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
/// 计算给定`纹理`基尼值
fn calculate_gini_rate_grain(v: &Vec<Melon>, grain: Grain) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.grain == grain {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
/// 计算给定`脐部`基尼值
fn calculate_gini_rate_navel(v: &Vec<Melon>, navel: Navel) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.navel == navel {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
/// 计算给定`触感`基尼值
fn calculate_gini_rate_touch(v: &Vec<Melon>, touch: Touch) -> f32 {
    let cnt = v.len();
    let mut target_cnt = 0;
    let mut target_good_cnt = 0;
    let mut other_cnt = 0;
    let mut other_good_cnt = 0;
    for melon in v {
        if melon.touch == touch {
            target_cnt += 1;
            if melon.is_good {
                target_good_cnt += 1;
            }
        } else {
            other_cnt += 1;
            if melon.is_good {
                other_good_cnt += 1;
            }
        }
    }

    let cnt = cnt as f32;
    let target_cnt = target_cnt as f32;
    let target_good_cnt = target_good_cnt as f32;
    let other_cnt = other_cnt as f32;
    let other_good_cnt = other_good_cnt as f32;

    (target_cnt / cnt)
        * (1.
            - (target_good_cnt / target_cnt).powi(2)
            - ((target_cnt - target_good_cnt) / target_cnt).powi(2))
        + (other_cnt / cnt)
            * (1.
                - (other_good_cnt / other_cnt).powi(2)
                - ((other_cnt - other_good_cnt) / other_cnt).powi(2))
}
