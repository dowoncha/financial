use common::DataPoint;

// mod ma;
// mod bb;

// pub use self::ma::*;

pub trait Technical {
    fn update(&self, data: DataPoint) { }

    fn seen_enough_data(&self) -> bool { false }

    fn num_data_missing(&self) -> usize { 0 }
}

