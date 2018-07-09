According to Earnshaw's Theorem, there exist no stable equilibrium points in any static magnetic or electric field.
Paraphrasing this: at no point in a magnetic field can you position another magnet such that it stays still even when slightly disturbed.
But this is what control systems are supposed to solve, right?

# Physical Structure of The Suspender

This is simple: there is an electric magnet lifted high about 20cm, which has a constant-current power source, driven by a microcontroller.
Below and above the magnet, there are Hall sensors which can be sampled by the microcontroller.
Below the magnet, there is a platform which can move up and down, driven by stepper motors. On that platform, there is a load cell (scale), which measures the weight of a small permanent magnet.

# 'State-Space Exploration'

The lifting platform can be used to explore different situations, in order to infer a control scheme.
We can drive the platform up and down and sample the Hall sensors at each level of the upper magnet, for example.
Modeled mathematically, the Hall sensors measure the magnetic field strength, which decreases with the square of distance.
We also have to consider the strength of the upper magnet field, for which superposition can be assumed (the sensor's measurement
is the sum of the two magnetic fields in the vicinity).

```python
for each distance in range:
    for each powerlevel in magnetPower:
        lookup[readHall()][powerlevel] = distance
```

This calibration procedure would result in a 2d-lookup table.

We can also measure the effect of the upper magnet on the suspended one with this procedure:

```python
for each distance in range:
    for each powerlevel in magnetPower:
        weight = readWeight();
        force = weight_of_magnet - weight
        lookup[distance][force] = powerlevel
```

This calibration procedure allows for lookup of necessary power level at a given distance and a known resulting force on the magnet.
To measure this force, we use the scale on the platform.

## What to do with the data?

There are 2 options: either a mathematical model can be inferred from the data, and the sensors and control algorithm's output is transformed through that model.
The other option is to use the raw, averaged lookup tables together with interpolation to drive the system.

The first approach will be faster because the resulting mathematical functions allow for direct calculation without interpolation.
The second approach has the advantage that it can be easily automated!
To calibrate the suspender for a new magnet, all we need to do is run a calibration procedure to populate the lookup tables.

The possible third option would be to automatically infer the parameters of the mathematical model from the data, but that seems difficult for now.

## Parts List

[x] Large Electromagnet
[ ] Constant current power supply for magnet, controlled by a microcontroller
[x] 2x linear Hall sensor
[x] Small permanent magnet
[ ] Microcontroller with sufficient memory for lookup tables, FPU, several ADCs, ...
[x] HX711 load cell amplifier + breakout
[x] Load cell
[ ] Threaded bolt for elevator platform + nuts + bearings
[ ] Scaffold to hold magnet and platform
[x] 3x: Stepper motor + driver
[x] Linear Potentiometer for absolute sensing
