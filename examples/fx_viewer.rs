extern crate csv;
use csv::{ReaderBuilder, Reader};

extern crate chrono;
use chrono::naive::NaiveDateTime;

extern crate financial;
use financial::instruments::{CandlestickData};

use std::env;

struct CandleStickGraph {
    height: i32,
    data: Vec<CandlestickData>,
    global_min: f32,
    global_max: f32
}

impl CandleStickGraph {
    const SYMBOL_STICK: char = '\u{2502}';
    const SYMBOL_CANDLE: char = '\u{2053}';
    const SYMBOL_HALF_TOP: char = '\u{257d}';
    const SYMBOL_HALF_BOTTOM: char = '\u{257f}';
    const SYMBOL_HALF_CANDLE_TOP: char = '\u{257b}';
    const SYMBOL_HALF_CANDLE_BOTTOM: char = '\u{2579}';
    const SYMBOL_HALF_STICK_TOP: char = '\u{2577}';
    const SYMBOL_HALF_STICK_BOTTOM: char = '\u{2575}';
    const SYMBOL_NOTHING: char = ' ';

    pub fn new(data: Vec<CandlestickData>, height: i32) -> Self {
        let min = data.iter().fold(None, |min, x| match min {
            None => Some(x),
            Some(y) => Some(if x.l < y.l { x } else { y })
        }).unwrap().l;

        let max = data.iter().fold(None, |max, x| match max {
            None => Some(x),
            Some(y) => Some(if x.h > y.h { x } else { y })
        }).unwrap().h;

       println!("# Data: {}, min: {}, max: {}", data.len(), min, max);

        Self {
            data,
            height,
            global_min: min,
            global_max:  max
        }
    }

    pub fn draw(&self, colorize: bool) -> String {
        let mut output = String::new();

        for y in (0..self.height).rev() {
            if y % 4 == 0 {
                let marker = self.global_min + (y as f32 * self.global_max - self.global_min) / (self.height as f32);
                output.push_str(&format!("{:9}", marker));
            } else {
                output.push_str("         ");
            }

            for c in &self.data {
                output.push(self.render_candle_at(c, y, colorize));
            }

            output.push('\n');
        }

        output
    }

    fn to_height_units(&self, x: f32) -> f32 {
        return (x - self.global_min) / (self.global_max - self.global_min) * self.height as f32;
    }

    fn render_candle_at(&self,candle: &CandlestickData, height_unit: i32, colorize: bool) -> char {
        let height_unit = height_unit as f32;
        let ts = self.to_height_units(candle.h);
        let tc = self.to_height_units(candle.o);

        let bs = self.to_height_units(candle.l);
        let bc = self.to_height_units(candle.c);

        if ts.ceil() >= height_unit && height_unit >= tc.floor() {
            if tc - height_unit > 0.75 {
                return Self::SYMBOL_CANDLE;
            } else if tc - height_unit > 0.25 {
                if ts - height_unit > 0.75 {
                    return Self::SYMBOL_HALF_TOP;
                } else {
                    return Self::SYMBOL_HALF_CANDLE_TOP;
                }
            } else {
                if ts - height_unit > 0.75 {
                    return Self::SYMBOL_STICK;
                } else if ts - height_unit > 0.25 {
                    return Self::SYMBOL_HALF_STICK_TOP;
                }
            }
        } else if tc.floor() >= height_unit && height_unit >= bc.ceil() {
            return Self::SYMBOL_CANDLE;
        } else if bc.ceil() >= height_unit && height_unit >= bs.floor() {
            if bc - height_unit < 0.25 {
                return Self::SYMBOL_CANDLE;
            } else if bc - height_unit < 0.75 {
                if bs - height_unit < 0.25 {
                    return Self::SYMBOL_HALF_BOTTOM;
                } else {
                    return Self::SYMBOL_HALF_CANDLE_BOTTOM;
                }
            } else {
                if bs - height_unit < 0.25 {
                    return Self::SYMBOL_STICK;
                } else if bs - height_unit < 0.75 {
                    return Self::SYMBOL_HALF_STICK_BOTTOM;
                }
            }
        }
        
        Self::SYMBOL_NOTHING
    }
}

fn try_main() -> Result<(), Box<std::error::Error>> {
    let filename = "EURUSD_M1_201804.csv";

    let mut path = env::current_dir().unwrap();
    path.push("examples");
    path.push(filename);

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)?;

    let mut data = Vec::new();

    // Read CSV File 
    for result in rdr.records() {
        let record = result?;
        let datetime = NaiveDateTime::parse_from_str(record.get(0).unwrap(), "%Y%m%d %H%M%S")?;
        let open = record.get(1).unwrap().parse::<f32>().unwrap();
        let high = record.get(2).unwrap().parse::<f32>().unwrap();
        let low = record.get(3).unwrap().parse::<f32>().unwrap();
        let close = record.get(4).unwrap().parse::<f32>().unwrap();

        let candle = CandlestickData {
            o: open,
            h: high,
            l: low,
            c: close
        };

        data.push(candle);

            // No use for volume
    }

    let graph = CandleStickGraph::new(data, 30);
    println!("{}", graph.draw(false));

    Ok(())
}

fn main () {
    try_main().unwrap();
}