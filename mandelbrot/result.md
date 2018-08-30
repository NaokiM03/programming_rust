## spec

```
cpu: Intel core i7 8700K  
memory: DDR4-2133 32GB
```

## command

```
cargo build --release

Measure-Command { target\release\mandelbrot.exe test.png 4000x3000 -1.20,0.35 -1,0.2 }
```

## single

```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 3
Milliseconds      : 594
Ticks             : 35942353
TotalDays         : 4.15999456018519E-05
TotalHours        : 0.000998398694444444
TotalMinutes      : 0.0599039216666667
TotalSeconds      : 3.5942353
TotalMilliseconds : 3594.2353
```

## 8thread

```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 1
Milliseconds      : 53
Ticks             : 10536261
TotalDays         : 1.21947465277778E-05
TotalHours        : 0.000292673916666667
TotalMinutes      : 0.017560435
TotalSeconds      : 1.0536261
TotalMilliseconds : 1053.6261
```

## 12thread

```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 755
Ticks             : 7559462
TotalDays         : 8.74937731481481E-06
TotalHours        : 0.000209985055555556
TotalMinutes      : 0.0125991033333333
TotalSeconds      : 0.7559462
TotalMilliseconds : 755.9462
```

## 1thread

```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 3
Milliseconds      : 582
Ticks             : 35820430
TotalDays         : 4.14588310185185E-05
TotalHours        : 0.000995011944444444
TotalMinutes      : 0.0597007166666667
TotalSeconds      : 3.582043
TotalMilliseconds : 3582.043
```
