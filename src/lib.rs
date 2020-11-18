#![feature(external_doc)]
use ta_common::traits::Indicator;

pub struct EMA {
    ema: f64,
    init: bool,
    k: f64,

}
#[doc(include = "../README.md")]
impl EMA {
    pub fn new(period: u32) -> EMA {
        let k = 2.0 / (period as f64 + 1.0);
        Self {
            k,
            init: false,
            ema: 0.0,
        }
    }
}

impl Indicator<f64, f64> for EMA {
    fn next(&mut self, input: f64) -> f64 {
        if self.init != true {
            self.ema = input;
            self.init = true;
        } else {
            self.ema = (1_f64 - self.k) * self.ema + self.k * input;
        }

        self.ema
    }

    fn reset(&mut self) {
        self.init = false;
        self.ema = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::EMA;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut ema = EMA::new(5);
        assert_eq!(ema.next(81.59), 81.59);
        assert_eq!(ema.next(81.06), 81.41333333333334);
        assert_eq!(ema.next(82.87), 81.8988888888889);
        assert_eq!(ema.next(83.00), 82.26592592592594);
        assert_eq!(ema.next(83.61), 82.71395061728396);
        assert_eq!(ema.next(83.15), 82.85930041152264);
        assert_eq!(ema.next(82.84), 82.8528669410151);
        assert_eq!(ema.next(83.99), 83.23191129401008);
        assert_eq!(ema.next(84.55), 83.67127419600672);
        assert_eq!(ema.next(84.36), 83.9008494640045);
        assert_eq!(ema.next(85.53), 84.44389964266966);
        assert_eq!(ema.next(86.54), 85.14259976177978);
        assert_eq!(ema.next(86.89), 85.7250665078532);
    }
}
