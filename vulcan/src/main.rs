#![no_main]
#![no_std]
#![deny(unused_imports)]
#![deny(unsafe_code)]
#![deny(warnings)]

use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32f4xx_hal as _; // memory layout

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

use core::sync::atomic::{AtomicUsize, Ordering};
static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});

mod types;
mod views;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use arducam_stm32::{Arducam, CameraType};
    use display_interface_spi::SPIInterface;
    use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
    use embedded_hal::spi::{Mode, Phase, Polarity};
    use st7789::{Orientation, ST7789};

    use stm32f4xx_hal::time::KiloHertz;
    use stm32f4xx_hal::{delay::Delay, i2c::I2c, prelude::*, spi::Spi, time::MegaHertz};

    use crate::types::{BacklightLED, Display};
    use crate::views;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        delay: Delay,
        display: Display,
        backlight: BacklightLED,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        ctx.core.DCB.enable_trace();
        ctx.core.DWT.enable_cycle_counter();

        // Set up the system clock.
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();

        let mut delay = Delay::new(ctx.core.SYST, clocks);

        let mut display = {
            let sck2 = gpiob.pb10.into_alternate_af5();
            let miso2 = gpiob.pb14.into_alternate_af5();
            let mosi2 = gpiob.pb15.into_alternate_af5();
            let spi2 = Spi::spi2(
                ctx.device.SPI2,
                (sck2, miso2, mosi2),
                Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                MegaHertz(8).into(),
                clocks,
            );

            let chip_select = gpioa.pa4.into_push_pull_output(); // chip select
            let data_control = gpioa.pa1.into_push_pull_output(); // data control
            let display_interface = SPIInterface::new(spi2, data_control, chip_select);

            let lcd_reset = gpioa.pa3.into_push_pull_output();

            ST7789::new(display_interface, lcd_reset, 240, 240)
        };

        // a certain amount of delay is necessary so the display responds on first boot
        delay.delay_ms(250u16);

        // Initialise the display and clear the screen
        display.init(&mut delay).unwrap();

        display.set_orientation(Orientation::Landscape).unwrap();

        delay.delay_ms(250u16);

        // backlight control
        let backlight = gpioa.pa2.into_push_pull_output();

        let mut camera = {
            let sck1 = gpioa.pa5.into_alternate_af5();
            let miso1 = gpioa.pa6.into_alternate_af5();
            let mosi1 = gpioa.pa7.into_alternate_af5();
            let spi1 = Spi::spi1(
                ctx.device.SPI1,
                (sck1, miso1, mosi1),
                Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                MegaHertz(8).into(),
                clocks,
            );

            let chip_select = gpiob.pb0.into_push_pull_output(); // chip select
            let slc1 = gpiob.pb6.into_alternate_af4_open_drain(); // i2c slc1
            let sda1 = gpiob.pb7.into_alternate_af4_open_drain(); // i2c sda1
            let i2c1 = I2c::i2c1(ctx.device.I2C1, (slc1, sda1), KiloHertz(100), clocks);

            Arducam::new(CameraType::OV5642, chip_select, spi1, i2c1)
        };

        // camera
        //     .set_jpeg_size(&mut delay, arducam_stm32::OV5642Size::S320x240)
        //     .unwrap();

        if let Some(err) = camera
            .init(&mut delay, arducam_stm32::CameraFormat::JPEG)
            .err()
        {
            defmt::error!("{}", err);
        }
        // camera.spi_send(0x03, 0x02).unwrap();
        // camera
        //     .set_jpeg_size(&mut delay, arducam_stm32::OV5642Size::S320x240)
        //     .unwrap();
        // delay.delay_ms(1000_u16);
        // camera.clear_fifo_flag().unwrap();
        // camera.spi_send(0x01, 0x00).unwrap();

        // backlight.set_low().unwrap();
        // display.clear(Rgb565::BLACK).unwrap();
        // let image = DynamicBmp::<Rgb565>::from_slice(&buffer).unwrap();
        // Image::new(&image, display.bounding_box().center() - Point::new(43, 40))
        //     .draw(&mut display)
        //     .unwrap();
        // backlight.set_high().unwrap();

        // let mut buffer: [u8; 128] = [0; 128];
        // camera.get_frame(&mut delay, &mut buffer).unwrap();
        // defmt::debug!("{}", buffer);

        defmt::info!("INIT DONE");

        (
            Shared {},
            Local {
                display,
                delay,
                backlight,
            },
            init::Monotonics(),
        )
    }

    fn clear_screen(display: &mut Display, backlight: &mut BacklightLED) {
        backlight.set_low().unwrap();
        display.clear(Rgb565::BLACK).unwrap();
    }
    fn show_screen(backlight: &mut BacklightLED) {
        backlight.set_high().unwrap();
    }

    #[idle(local = [display, delay, backlight])]
    fn idle(ctx: idle::Context) -> ! {
        let display: &'static mut Display = ctx.local.display;
        let delay: &'static mut Delay = ctx.local.delay;
        let backlight: &'static mut BacklightLED = ctx.local.backlight;

        // draw splash screen
        clear_screen(display, backlight);
        views::splash_screen(display);
        show_screen(backlight);

        delay.delay_ms(4000u16);

        // draw home screen
        clear_screen(display, backlight);
        views::home(display);
        show_screen(backlight);

        loop {
            cortex_m::asm::nop();
        }
    }

    // #[task(local = [led])]
    // fn blink(ctx: blink::Context) {
    //     ctx.local.led.toggle();
    //     defmt::info!("Blink!");
    //     blink::spawn_after(Seconds(1_u32)).ok();
    // }
}
