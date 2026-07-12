use embedded_graphics::{mono_font::{
    ascii::{FONT_6X10, FONT_10X20},
    MonoFont,
}, pixelcolor::BinaryColor, prelude::Point, text::{Baseline, Text}, Drawable};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyle;

// use embedded_picofont::FontPico;

// TODO Fix cargo build when using this file.

// Mostly adapted from my C++ functions here
// https://github.com/kelson8/KCNet-ESP32-SSD1306/blob/main/src/oled-functions.cpp

pub fn font_for_size(text_size: u8) -> &'static MonoFont<'static> {
    match text_size {
        0..=1 => &FONT_6X10,
        _ => &FONT_10X20,
    }
}

pub fn draw_line<D>(
    display: &mut D,
    text_size: u8,
    x: i32,
    y: i32,
    text: &str,
) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let font = font_for_size(text_size);
    let style = MonoTextStyle::new(font, BinaryColor::On);
    Text::with_baseline(text, Point::new(x, y), style, Baseline::Top).draw(display)
}

pub fn draw_line1<D>(display: &mut D, text_size: u8, x: i32, text: &str) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_line(display, text_size, x, 10, text)
}

pub fn draw_line2<D>(display: &mut D, text_size: u8, x: i32, text: &str) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_line(display, text_size, x, 25, text)
}

pub fn draw_line3<D>(display: &mut D, text_size: u8, x: i32, text: &str) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_line(display, text_size, x, 40, text)
}

pub fn draw_line4<D>(display: &mut D, text_size: u8, x: i32, text: &str) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_line(display, text_size, x, 55, text)
}
