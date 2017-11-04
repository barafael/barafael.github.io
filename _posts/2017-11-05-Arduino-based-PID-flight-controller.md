A PID-controller regulates a system in such a way that the difference between a
set point and a measured point is minimized. This ability is extremely useful
and has many applications across most engineering disciplines. In this article,
I want to present a simple PID-based multi-purpose flight controller that runs
on the Teensy 3.2 board using Arduino software but should also run on other
Arduino-compatible hardware with only slight modifications.

To make best use of this article, you should have a basic understanding about
PID, standard RC components like ESC's (electronic speed controllers), Motors
and RX/TX, and some C programming skills.

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
can be overclocked up to 120MHz which should be more than sufficient for a basic control
task. The board also has very many GPIO pins, and can tolerate a wide range of
supply voltages (such as 5V from a standard RC electronic speed controller)
with its internal stabilizer.

As IMU sensor, I am using the widely available and familiar good ol' MPU6050. But it would be simple to adapt the software to better and more expensive sensors. The MPU6050 offers measurement of absolute orientation as well as angular rates from the gyroscope and can be read very quickly using the I2C protocol (we will see how).

As basic RC hardware, any standard ESC's and servos should work.
