
# Introduction

This blogpost is about a simple PID controller that eventually will be able to
stabilize small aircraft and multicopters.  For now, it works fine on one axis
(roll), but I will keep working on it to make it more customizable.  I am
thinking about different flight modes, which will each have their own mixers
(controlling which PID coefficients to use for which axis). Transitioning
between flight modes would interpolate the values of the two mixers.

For the [Hobbyking KK2.1.5 Multi-Rotor LCD Flight Control
Board](https://hobbyking.com/de_de/hobbyking-kk2-1-5-multi-rotor-lcd-flight-control-board-with-6050mpu-and-atmel-644pa.html?___store=de_de),
there is a firmware that does this very well:
[OpenAeroVTOL](https://www.rcgroups.com/forums/showthread.php?1972686-OpenAeroVTOL-with-transitional-mixers-(perfect-for-VTOLs)),
which was a big part of the inspiration for this project. The hardware this
project is based on is showing it's age. I will also be using an MPU6050 for my
project, but since I am not coupled with any specific board, it would be easy
to change to a better (and more expensive) sensor. The OpenAeroVTOL firmware can be configured using the onboard LCD of the kk board (that is an admirable feat!) or, more recently, using Excel Macros (yes, really).

The aim for this project is to make a simple multicopter firmware for the teensy 3.2 and expand on it.
Ideally, the basic features (transitional mixers and multiple flight modes) from openAeroVTOL should also be possible, but that is a long shot. This blog post describes a 1-dimensional controller.

The code can be found at: https://github.com/medium-endian/multipid

# Overview

A PID-controller regulates a system in such a way that the difference between a
set point and a measured point is minimized. This ability is extremely useful
and has many applications across most engineering disciplines. In this article,
I want to present a simple PID-based multi-purpose flight controller that runs
on the Teensy 3.2 board using Arduino software but should also run on other
Arduino-compatible hardware with only slight modifications.

To make best use of this article, you should have a basic understanding about
PID, standard RC components like ESC's (electronic speed controllers), Motors
and RX/TX, and some C programming skills.

This PID controller only works on one axis (roll) for now. It would be easy to
generalize.

## Why use PID for flight control?

In a quadrocopter, plane, or a rocket for that matter, there are control
surfaces and propellers which interact with airflow and inflict forces on our
craft. For example, the elevator on a plane diverts airflow, causing a force on
the long lever that is the aircrafts tail and pitching the plane up or down.
The propellers on a multicopter produce lift, and they also influence the angle
and angular rate at which the copter is moving.

All those properties (pitch, yaw, roll angles or angular rates, thrust,
ascent/descent...) are perfect candidates for regulation through a PID
controller - we have a setpoint, and we can use an Inertial Measurement Unit to
find out their current value.

The setpoint can either be given as an absolute angle, or as an angular rate.
In the first case, it will attempt to stabilize the craft to this angle,
whereas in the second case, it will try to move the vehicle at the given rate.
This is known as 'rate mode' or 'acro mode' in some flight controllers.

## Hardware

I am using a Teensy 3.2 as control board. It houses an ARM Cortex M4 chip that
can be overclocked up to 120MHz which should be more than sufficient for a
basic control task. The board also has very many GPIO pins, and can tolerate a
wide range of supply voltages (such as 5V from a standard RC electronic speed
controller) with its internal stabilizer.

As IMU sensor, I am using the widely available and familiar good ol' MPU6050.
But it would be simple to adapt the software to better and more expensive
sensors. The MPU6050 offers measurement of absolute orientation as well as
angular rates from the gyroscope and can be read very quickly using the I2C
protocol (we will see how).

As basic RC hardware, any standard ESC's, servos and TX/RX combo should work.

# Four simple steps

We need to do four things:

* Read input from the RC receiver
* Read sensor values from IMU
* Calculate PID
* response with previous measurements as input Write the result to the connected motors.

## Reading input from the receiver

I use interrupts to record the time an input signal changed state, at which
point I log the system time with millis() if it went ``HIGH`` or calculate the
elapsed time in case it went ``LOW``. I use shared volatile variables to copy
the values to the main loop. This way, it is always clear that the interrupt
writes while the main loop reads the shared variables. When the variables are
read in the main loop, no interrupts are allowed (since they might overwrite
the values while reading them!). The read values are in milliseconds, so
roughly between 1000 and 2000. This is the standard for RC Pulse Position
Modulated signals.

For a really good description, look at [this excellent article by Ryan
Boland](https://ryanboland.com/blog/reading-rc-receiver-values/). He explains
it better than I will ever be able to - and with oscilloscope screenshots!

## Read sensor values from IMU

To read the MPU6050, I adapted large parts from [Jeff Rowbergs example code for
his
library](https://github.com/jrowberg/i2cdevlib/blob/master/Arduino/MPU6050/examples/MPU6050_DMP6/MPU6050_DMP6.ino).
I also added a function to request the raw gyro rate reading, which I found [in
Joop Brokking's YMFC V2 source
code](http://www.brokking.net/ymfc-3d_v2_main.html). His firmware is a great
read if you are interested.

Sensor data is read whenever the IMU signals that data is ready, signaled by
interrupt.

## Calculate PID response

Relatively basic PID control code here. The setpoint is interpreted as an
absolute angle, not an angular rate. That would also be possible, though.

## Write result to connected motors

The output is finished. It only needs to be scaled a bit and then written out
using the ``writeMicroseconds(int micros)`` method of the Arduino ``Servo``
class.
