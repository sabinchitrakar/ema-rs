# Exponential Moving Average (EMA)
```
 use ta_common::traits::Indicator;
 use ema_rs::EMA;
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
```
###Calculation

ema= (1-k) ema<sub>prev.</sub> + k * input;  
k=2/(N+1);  
N=period;

