
A simple driver created for the SSD1351 OLED 1.5 inch rgb screen for the Pi Pico W

Important fixes to come:
    - Spi bindings are currently hardcoded. when migrating rp2040_hal versions, DynPinID broke and have not found a nice fix yet.
    - 

Basic usage example:


```

    #[global_allocator]
    static HEAP: Heap = Heap::empty();

    const FONT: FONT11X18 = FONT11X18 {
        width: 11,
        height: 18,
    };

    ...

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32768;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let spi_sclk: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio2.reconfigure();
    let spi_mosi: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio3.reconfigure();

    let cs: gpio::Pin<_, gpio::FunctionSio<gpio::SioOutput>, gpio::PullDown> =
        pins.gpio6.into_push_pull_output();
    let dc: gpio::Pin<_, gpio::FunctionSio<gpio::SioOutput>, gpio::PullDown> =
        pins.gpio7.into_push_pull_output();
    let rst: gpio::Pin<_, gpio::FunctionSio<gpio::SioOutput>, gpio::PullDown> =
        pins.gpio8.into_push_pull_output();

    let spi_bindings = SPIPins { spi_sclk, spi_mosi };
    let spi_driver_bindings = SPIDriverPins { cs, dc, rst };

    let spi_display_controller = SSD1351_SPI::SPIDisplayInterface::new(
        spi_bindings,
        spi_driver_bindings,
        pac.SPI0,
        &mut pac.RESETS,
        &mut clocks.peripheral_clock.freq(),
    );

    let mut oled: SSD1351_OLED = SSD1351_OLED {
        spi_controller: spi_display_controller,
        height: 128,
        width: 128,
    };

    // Create buffer filed with "black" using a Vec<>
    let buff_oled: Vec<SSD1351_Color> = vec![
        SSD1351_Color {
            red: 0,
            green: 0,
            blue: 0
        };
        16384
    ];

    let mut fbo_oled: SSD1351_FBO = SSD1351_FBO {
        width: 128,
        height: 128,
        pixels: buff_oled,
    };

    oled.init(&mut delay, true);
    oled.clear(&mut fbo_oled);


    loop
    {
        oled.clear(&mut fbo_oled);

        fbo_oled.draw_string(Point { x: 0, y: 20 }, "OLED rockz", &FONT, &SSD1351_Color{red: 178, green: 102, blue: 255});

        fbo_oled.draw_line(Point { x: 50, y: 40 }, Point { x: 60, y: 50 }, &SSD1351_Color{red: 255, green:255, blue: 153});
        fbo_oled.draw_circle(Point { x: 64, y: 60 }, 5, &SSD1351_Color{red: 204, green: 255, blue: 204});
        fbo_oled.draw_circle_fill(Point { x: 30, y: 100 }, 8, &SSD1351_Color{red: 255, green: 128, blue: 0});

        fbo_oled.draw_rect(Point { x: 64, y: 90 }, 20, 10, &SSD1351_Color{red: 0, green: 153, blue: 76});
        fbo_oled.draw_rect_fill(Point { x: 64, y: 100 }, 30, 10, &SSD1351_Color{red: 102, green: 0, blue: 102});

        oled.update(&mut fbo_oled);
    }

```