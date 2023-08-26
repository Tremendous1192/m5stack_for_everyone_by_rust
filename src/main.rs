//! 組込みRustのおまじない
#![no_std]
#![no_main]
extern crate alloc; // 配列を扱う
use esp32_hal as hal; // 必須
use esp_backtrace as _; // エラーハンドリング
use esp_println::println; // Espressif devicesでprintlnマクロを扱う
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
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::{Baseline, Text},
};
use embedded_graphics as eg;

#[hal::prelude::entry]
fn main() -> ! {
    println!("Hello World プログラム");
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

    let spi /*Serial Peripheral Interface*/= hal::spi::Spi::new(
        peripherals.SPI3,
        /*sck*/ io.pins.gpio18,
        /*mosi*/ io.pins.gpio23,
        /*miso*/ io.pins.gpio19,
        /*cs*/ io.pins.gpio14,
        50_u32.MHz(),
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
    let _ = display.fill_solid(
        &Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240)),
        Rgb565::BLACK,
    );

    // Hello World !
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::with_baseline("Hello World !", Point::new(100, 120), style, Baseline::Top)
        .draw(&mut display)
        .ok()
        .unwrap();

    // 処理が最後まで進んだかの確認
    delay.delay_ms(20_000_u32); // 20秒の待機
    backlight.set_low().unwrap();

    // ここまで 初期化

    // 組込みはloop必須
    loop {}
    // ここまでloop処理
}
// ここまでmain関数
