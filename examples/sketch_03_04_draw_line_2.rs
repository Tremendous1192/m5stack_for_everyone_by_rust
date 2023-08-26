//! 組込みRustのおまじない
#![no_std]
#![no_main]
extern crate alloc; // 配列を扱う
use esp32_hal as hal; // 必須
use esp_backtrace as _; // エラーハンドリング
use hal::prelude::*; // tomlファイルにて as 構文でesp32-halから名前を変えている

#[global_allocator] // ヒープの初期化
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    // C言語を扱う
    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    // メモリ割り当て
    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        assert!(
            heap_end - heap_start > HEAP_SIZE,
            "Not enough available heap memory."
        );
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

// 入出力
use display_interface_spi::SPIInterfaceNoCS;

// 描画
use eg::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
};
use embedded_graphics as eg;

#[hal::prelude::entry]
fn main() -> ! {
    // 初期化
    // 必須
    init_heap();
    let peripherals = hal::peripherals::Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let mut clocks = hal::clock::ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = hal::delay::Delay::new(&clocks);

    // タイマー
    let mut rtc = hal::Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = hal::timer::TimerGroup::new(
        peripherals.TIMG0, // 0x3ff5f000
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = hal::timer::TimerGroup::new(
        peripherals.TIMG1, // 0x3ff60000
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // GPIO
    let io = hal::IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // バックライト
    let mut backlight = io.pins.gpio32.into_push_pull_output();
    backlight.set_high().unwrap();

    // データシートより
    // Typical values are setting example when used with panel resolution 320 x 240 (LQVGA),
    // clock frequency 6.35MHz and frame frequency about 70Hz.
    // https://m5stack.oss-cn-shenzhen.aliyuncs.com/resource/docs/datasheet/core/ILI9342C-ILITEK.pdf
    let spi /*Serial Peripheral Interface*/= hal::spi::Spi::new(
        peripherals.SPI3,
        /*sck*/ io.pins.gpio18,
        /*mosi*/ io.pins.gpio23,
        /*miso*/ io.pins.gpio19,
        /*cs*/ io.pins.gpio14,
        6_u32.MHz(),
        hal::spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &mut clocks,
    );
    let di /*Display Interface*/= SPIInterfaceNoCS::new(
        spi,
        /*dc(output)*/ io.pins.gpio27.into_push_pull_output(),
    );

    // ディスプレイ
    let mut display = mipidsi::builder::Builder::ili9342c_rgb565(di)
        .with_invert_colors(/*invert_colors: bool*/ true)
        .init(
            &mut delay,
            Some(/*rst*/ io.pins.gpio33.into_push_pull_output()),
        )
        .unwrap();
    display
        .fill_solid(
            &Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240)),
            Rgb565::BLACK,
        )
        .unwrap();

    // ここまで 初期化

    // 組込みはloop必須
    loop {
        // 白い直線
        for y in (0..240).step_by(10) {
            Line::new(Point::new(0, 0), Point::new(319, y as i32))
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
                .draw(&mut display)
                .unwrap();
            delay.delay_ms(50_u32);
        }

        // 赤い直線のはずが青い直線
        for x in (0..320).rev().step_by(10) {
            Line::new(Point::new(0, 0), Point::new(x as i32, 239))
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
                .draw(&mut display)
                .unwrap();
            delay.delay_ms(50_u32);
        }

        // 青い直線のはずが赤い直線
        for x in (0..320).rev().step_by(10) {
            Line::new(Point::new(319, 0), Point::new(x as i32, 239))
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 1))
                .draw(&mut display)
                .unwrap();
            delay.delay_ms(50_u32);
        }

        // 緑の直線
        for y in (0..240).rev().step_by(10) {
            Line::new(Point::new(319, 0), Point::new(0, y as i32))
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
                .draw(&mut display)
                .unwrap();
            delay.delay_ms(50_u32);
        }

        // 描画が終わったら画面を初期化する
        delay.delay_ms(2_000_u32);
        display
            .fill_solid(
                &Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240)),
                Rgb565::BLACK,
            )
            .unwrap();
    }
    // ここまでloop処理
}
// ここまでmain関数
