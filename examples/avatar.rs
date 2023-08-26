// 描画
use eg::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics as eg;

use core::fmt::Debug;

// 英語でも自分の気持ちを正確に伝えられる、喜怒哀楽の英語表現42選 | English Lab（イングリッシュラボ）┃レアジョブ英会話が発信する英語サイト
// https://www.rarejob.com/englishlab/column/20190622/
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Emotion {
    Happy,   // 幸せ
    Sad,     // 悲しい
    Neutral, // 初期状態
}

/// M5Stackちゃんのような表情を描画する構造体
pub struct Avatar {
    emotion: Emotion,
    previous_emotion: Emotion,
}

impl Avatar {
    pub fn new<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Self
    where
        <T as DrawTarget>::Error: Debug,
    {
        draw_neutral(display, Rgb565::WHITE);

        Avatar {
            emotion: Emotion::Neutral,
            previous_emotion: Emotion::Neutral,
        }
    }

    pub fn change_emotion<T: DrawTarget<Color = Rgb565>>(
        &mut self,
        next_emotion: &Emotion,
        display: &mut T,
    ) where
        <T as DrawTarget>::Error: Debug,
    {
        // 同じ状態の場合描画しない
        if self.emotion == *next_emotion {
            return;
        }

        // 状態の遷移
        self.previous_emotion = self.emotion;
        self.emotion = *next_emotion;

        // 前回の表情を黒で塗りつぶす
        match self.previous_emotion {
            Emotion::Happy => {
                draw_happy(display, Rgb565::BLACK);
            }
            Emotion::Sad => {
                draw_sad(display, Rgb565::BLACK);
            }
            _ => {
                draw_neutral(display, Rgb565::BLACK);
            }
        }

        // 表情を変える
        match self.emotion {
            Emotion::Happy => {
                draw_happy(display, Rgb565::WHITE);
            }
            Emotion::Sad => {
                draw_sad(display, Rgb565::WHITE);
            }
            _ => {
                draw_neutral(display, Rgb565::WHITE);
            }
        }
    }
}

// 初期状態の表情を描画する
pub(crate) fn draw_neutral<T: DrawTarget<Color = Rgb565>>(display: &mut T, color: Rgb565)
where
    <T as DrawTarget>::Error: Debug,
{
    let style = PrimitiveStyleBuilder::new().fill_color(color).build();

    // 口
    Rectangle::with_corners(Point::new(120, 140), Point::new(120 + 80, 140 + 30))
        .into_styled(style)
        .draw(display)
        .unwrap();
    // 左目
    Circle::new(Point::new(90, 80), 10)
        .into_styled(style)
        .draw(display)
        .unwrap();
    // 右目
    Circle::new(Point::new(230, 80), 10)
        .into_styled(style)
        .draw(display)
        .unwrap();
}

// 幸せ状態の表情を描画する
pub(crate) fn draw_happy<T: DrawTarget<Color = Rgb565>>(display: &mut T, color: Rgb565)
where
    <T as DrawTarget>::Error: Debug,
{
    // Arc with styled stroke with top-left point at (15, 25) with a diameter of 20
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(color)
        .stroke_width(5)
        .build();

    // 口
    Arc::new(Point::new(120, 140 - 50), 80, 195.0.deg(), 150.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();

    // 左目
    Arc::new(Point::new(90, 80), 10, 180.0.deg(), -180.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();
    // 右目
    Arc::new(Point::new(230, 80), 10, 180.0.deg(), -180.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();
}

// 悲しい状態の表情を描画する
pub(crate) fn draw_sad<T: DrawTarget<Color = Rgb565>>(display: &mut T, color: Rgb565)
where
    <T as DrawTarget>::Error: Debug,
{
    // Arc with styled stroke with top-left point at (15, 25) with a diameter of 20
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(color)
        .stroke_width(5)
        .build();

    // 口
    Arc::new(Point::new(120, 140), 80, 15.0.deg(), 150.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();

    // 左目
    Arc::new(Point::new(90, 80), 10, 0.0.deg(), -180.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();
    // 右目
    Arc::new(Point::new(230, 80), 10, 0.0.deg(), -180.0.deg())
        .into_styled(style)
        .draw(display)
        .unwrap();
}
