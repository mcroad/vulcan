use display_interface_spi::SPIInterface;
use st7789::ST7789;
use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA1, PA2, PA3, PA4},
        gpiob::{PB10, PB14, PB15},
        Alternate, Output, PushPull, AF5,
    },
    spi::Spi,
    stm32::SPI2,
};

pub type Display = ST7789<
    SPIInterface<
        Spi<
            SPI2,
            (
                PB10<Alternate<AF5>>,
                PB14<Alternate<AF5>>,
                PB15<Alternate<AF5>>,
            ),
        >,
        PA1<Output<PushPull>>,
        PA4<Output<PushPull>>,
    >,
    PA3<Output<PushPull>>,
>;
pub type BacklightLED = PA2<Output<PushPull>>;
