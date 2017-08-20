#![no_std]
#![no_main]

#[macro_use(reset_fn)]
extern crate mkl27z;

const NUM_COLS: usize = 8;
const NUM_ROWS: usize = 18;

struct FilcoMJ2TKLConfig {
  column_pins: [mkl27z::gpio::InputPin; NUM_COLS],
  row_pins: [mkl27z::gpio::OutputPin; NUM_ROWS],
}

fn get_column_pins() -> [mkl27z::gpio::InputPin; NUM_COLS] {
  [mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTB_PCR16.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     pdir: mkl27z::gpio::GPIOB_PDIR.into(),
     num: 16,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTC_PCR0.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     pdir: mkl27z::gpio::GPIOB_PDIR.into(),
     num: 0,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTB_PCR1.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     pdir: mkl27z::gpio::GPIOB_PDIR.into(),
     num: 1,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTB_PCR17.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     pdir: mkl27z::gpio::GPIOB_PDIR.into(),
     num: 17,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTE_PCR24.into(),
     pddr: mkl27z::gpio::GPIOE_PDDR.into(),
     pdir: mkl27z::gpio::GPIOE_PDIR.into(),
     num: 24,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTA_PCR1.into(),
     pddr: mkl27z::gpio::GPIOA_PDDR.into(),
     pdir: mkl27z::gpio::GPIOA_PDIR.into(),
     num: 1,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTA_PCR19.into(),
     pddr: mkl27z::gpio::GPIOA_PDDR.into(),
     pdir: mkl27z::gpio::GPIOA_PDIR.into(),
     num: 19,
   },
   mkl27z::gpio::InputPin {
     port: mkl27z::gpio::PORTA_PCR2.into(),
     pddr: mkl27z::gpio::GPIOA_PDDR.into(),
     pdir: mkl27z::gpio::GPIOA_PDIR.into(),
     num: 2,
   }]
}

fn get_row_pins() -> [mkl27z::gpio::OutputPin; NUM_ROWS] {
  [mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTB_PCR0.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     psor: mkl27z::gpio::GPIOB_PSOR.into(),
     pcor: mkl27z::gpio::GPIOB_PCOR.into(),
     num: 0,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR7.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 7,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTC_PCR2.into(),
     pddr: mkl27z::gpio::GPIOC_PDDR.into(),
     psor: mkl27z::gpio::GPIOC_PSOR.into(),
     pcor: mkl27z::gpio::GPIOC_PCOR.into(),
     num: 2,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR5.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 5,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR4.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 4,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR3.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 3,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR2.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 2,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR1.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 1,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR0.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 0,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTC_PCR7.into(),
     pddr: mkl27z::gpio::GPIOC_PDDR.into(),
     psor: mkl27z::gpio::GPIOC_PSOR.into(),
     pcor: mkl27z::gpio::GPIOC_PCOR.into(),
     num: 7,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTB_PCR2.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     psor: mkl27z::gpio::GPIOB_PSOR.into(),
     pcor: mkl27z::gpio::GPIOB_PCOR.into(),
     num: 2,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTB_PCR3.into(),
     pddr: mkl27z::gpio::GPIOB_PDDR.into(),
     psor: mkl27z::gpio::GPIOB_PSOR.into(),
     pcor: mkl27z::gpio::GPIOB_PCOR.into(),
     num: 3,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTC_PCR1.into(),
     pddr: mkl27z::gpio::GPIOC_PDDR.into(),
     psor: mkl27z::gpio::GPIOC_PSOR.into(),
     pcor: mkl27z::gpio::GPIOC_PCOR.into(),
     num: 1,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTC_PCR6.into(),
     pddr: mkl27z::gpio::GPIOC_PDDR.into(),
     psor: mkl27z::gpio::GPIOC_PSOR.into(),
     pcor: mkl27z::gpio::GPIOC_PCOR.into(),
     num: 6,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTA_PCR18.into(),
     pddr: mkl27z::gpio::GPIOA_PDDR.into(),
     psor: mkl27z::gpio::GPIOA_PSOR.into(),
     pcor: mkl27z::gpio::GPIOA_PCOR.into(),
     num: 18,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTC_PCR5.into(),
     pddr: mkl27z::gpio::GPIOC_PDDR.into(),
     psor: mkl27z::gpio::GPIOC_PSOR.into(),
     pcor: mkl27z::gpio::GPIOC_PCOR.into(),
     num: 5,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTE_PCR25.into(),
     pddr: mkl27z::gpio::GPIOE_PDDR.into(),
     psor: mkl27z::gpio::GPIOE_PSOR.into(),
     pcor: mkl27z::gpio::GPIOE_PCOR.into(),
     num: 25,
   },
   mkl27z::gpio::OutputPin {
     port: mkl27z::gpio::PORTD_PCR6.into(),
     pddr: mkl27z::gpio::GPIOD_PDDR.into(),
     psor: mkl27z::gpio::GPIOD_PSOR.into(),
     pcor: mkl27z::gpio::GPIOD_PCOR.into(),
     num: 6,
   }]
}

fn main() -> ! {
  let conf = FilcoMJ2TKLConfig {
    column_pins: get_column_pins(),
    row_pins: get_row_pins(),
  };
  loop {
    let _ = conf.column_pins[0].read();
    let _ = conf.row_pins[0].high();
  }
}
reset_fn!(main);
