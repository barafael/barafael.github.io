A PID-controller governs a system so that the difference between a setpoint
and a measured value is minimized. This function is ubiquitous and has
applications in many areas of engineering. In this article, I want to present a
simple PID-based multi-purpose flight controller that runs on the Teensy 3.2
board using Arduino libraries. It can control PWM-based actuators like servos
and electronic speed controllers driving brushless motors.

To get the most use out of this article, you should have a basic understanding
about PID, standard RC components like ESCs (electronic speed controllers),
brushless motors, RX/TX, and some C/C++ programming skills.

## Why use PID for flight control?

In quadcopters, planes, or many other vehicle types, we have to constantly
monitor and manipulate the attitude, thrust, velocity and similar parameters to
achieve the desired result. To influence those parameters, there are control
surfaces, levers or propellers/jets which inflict forces on our craft in any
necessary direction.

For example, the elevator on a plane diverts airflow, causing a force on the
long lever that is the aircraft's tail and pitching the plane up or down. A
hydrofoil watercraft is very similar to an airplane in that it uses foils to
generate lift and basically identical (but smaller) control surfaces to control
attitude. The propellers on a multicopter produce all the lift, but they also
influence the angle and angular rate at which the copter is moving in each
axis. A more modern example would be the small thrusters on spacecraft
which are used for attitude control. Hovercraft thrusters are conceptually
similar in 2 dimensions (no up/down).

But even more different control modes are thinkable - a rover might raise and
lower the suspension in order to achieve a given attitude, which conceptually
isn't much different from free-floating vehicles. Let's conclude that
applications of attitude/rate control for air, ground and water vehicles are
numerous.

All those properties (pitch, yaw, roll angles or angular rates, thrust,
ascent/descent...) are suitable for regulation through a PID controller - we
have a setpoint, and we can use an inertial measurement unit to find out their
current value.


# Cascaded PID

It is useful to use not only one but two PID controllers because the sensor
output from the gyroscope is way faster than the output from the
Kalman/Madgwick algorithm that fuses gyroscope and accelerometer data and
produces an accurate attitude estimation. We can cascade the controllers: the
first loop receives the setpoint interpreted as attitude and the currently
most accurate estimation of the attitude and produces a needed angular rate as
a result. This rate is fed into another loop as setpoint, and this rate loop
receives the faster angular rate readings from the gyroscope directly. If the
second loop executes more often, then it will ensure that the overall loop rate
is fast and not limited by the time it takes to fuse the IMU data. Gyroscope
data drifts only over time, whereas accelerometer data does not drift but is
subject to high-frequency noise.

The setpoint can either be interpreted as a desired absolute angle or as a
desired angular rate. In the first case, the PID controller will attempt to
stabilize the craft to this angle, whereas in the second case, it will try to
rotate the vehicle at the given rate. This is known as 'rate mode' or 'acro
mode' in some flight controllers.

To achieve 'rate mode', the attitude loop simply needs to be ignored. A
constant input on the roll stick of a plane in rate mode would result in a
continuous roll, or a multicopter yaw stick would control the rate of
z-rotation of the copter. In 'stabilize mode', a constant stick deviation is
mapped to a constant vehicle deviation.

## Hardware

I am using a Teensy 3.2 as the brains. It has an ARM Cortex M4 chip that can
be overclocked to 120MHz which is more than sufficient for this control task.
The board also has plenty of GPIO pins and can tolerate a wide range of supply
voltages (such as 5V from a standard RC electronic speed controller) with its
internal voltage regulator.

As IMU sensor, I am using the widely available and familiar MPU6050. This
sensor offers measurement of absolute orientation as well as angular rates from
the gyroscope and provides data moderately quickly using the I2C protocol. I
chose this sensor because it is initially very simple, but I would like to use
one of the newer Invensense sensors (ICM-20608) and implement my own
Kalman/Madgwick filter at some point.

As basic peripheral hardware (actuators), any standard ESCs, servos and TX/RX
combo using PPM should work.

# Four simple steps to fun (and control)

Here are the steps that need to be done to stabilize and control:

* Read input from the RC receiver
* Read sensor values from IMU
* Calculate PID response with those measurements as input
* Write the result to the connected actuators

## Reading input from the receiver

The receiver sends pulses of varying on-time corresponding to the stick
positions to our board. Using interrupts, we can measure the duration between a
rising flank and a falling flank of one signal, which should always be between
1000us and 2000us (RC standard). We can simply hook an interrupt to each input
pin, log the system time on a rising flank, and calculate duration in
microseconds since rising flank when the signal is falling again. The code for
this is in ```src/PWMReceiver.cpp```.

The duration measurement from our interrupt routines is written each time the
interrupt routine executes on a falling flank. That means, we have to be
careful when reading those values! I used shared volatile variables which are
written to by the interrupts to store the measurements. The main loop copies
the data to variables which it can use undisturbed. This way, it is always
clear that the interrupt writes while the main loop reads the shared variables.
When the variables are read, no interrupts are allowed since they might
overwrite the values while reading them, violating the 'sharing XOR mutability'
principle and thus inviting race conditions.

For a really good description on how to read RC receiver PPM output, look at
[this excellent article by Ryan Boland](https://ryanboland.com/blog/reading-rc-receiver-values/).
He explains it better than I will ever be able to - and with oscilloscope screenshots!

## Read sensor values from IMU

To read the MPU6050, I 'adapted' large parts from
[Jeff Rowbergs example code for his (unfortunately abandoned and a little outdated) project I2CDevLib](https://github.com/jrowberg/i2cdevlib/blob/master/Arduino/MPU6050/examples/MPU6050_DMP6/MPU6050_DMP6.ino).
I also added a function to request the raw gyro rate reading, which I found
in [Joop Brokking's YMFC V2 source code](http://www.brokking.net/ymfc-3d_v2_main.html).
His quadcopter firmware is a bit messy coding-wise but easy to understand and a
good read if you are interested. His understanding of the topic is great and it
shoes in his helpful youtube videos.

Sensor attitude data is read whenever the IMU signals that data is ready,
signalled by an interrupt. The gyroscope is read on every loop.

## Calculate PID response

This is where the magic happens, but this is actually very simple. The PID
algorithm in this discrete form (really the one everyone uses) is short and
sweet, using the "poor man's derivative (-)" and the "poor man's integral (+)".
The implementation of dual rates complicates things a bit but is useful in
the long run.

I adapted some ideas presented in
[Brett Beauregard's series "Improving the beginners PID"](http://brettbeauregard.com/blog/2011/04/improving-the-beginners-pid-introduction/)

Eventually, PID coefficients will depend on the currently chosen mode (if I get
to the implementation).

## Write result to connected motors

The output is finished. It only needs to be scaled a bit and then written out
using the ``writeMicroseconds(int micros)`` method of the Arduino ``Servo``
class.

Eventually, this will need more work: depending on the current flight mode,
output mixer volumes will have to be chosen for each output channel. Possible
parameters for the output channel mixers are throttle, roll/pitch/yaw attitude
PID output, roll/pitch/yaw rate output, rate/stabilize mode, and even motor
type (servo/ESC). More explanation on this in the next part.

## Give me the Code!

The code of the current implementation can be found at
[https://github.com/barafael/raPID](https://github.com/barafael/raPID)

# Further Ideas/Inspiration

I am thinking about allowing for several different flight/operation modes which
consist mostly of a set of PID-coefficients and settings for mixers on each
actuator output. A mixer would control how much impact each PID output should
have on each actuator output.

Transitioning between flight modes could interpolate the values of the two
mixers so that modes transition smoothly into each other.

If this transitional flight mode interpolation sounds familiar to you, it might
be because of
[OpenAeroVTOL](https://www.rcgroups.com/forums/showthread.php?1972686-OpenAeroVTOL-with-transitional-mixers-(perfect-for-VTOLs)).
It runs (really well) on the
[Hobbyking KK2.1.5 Multi-Rotor LCD Flight Control Board](https://hobbyking.com/de_de/hobbyking-kk2-1-5-multi-rotor-lcd-flight-control-board-with-6050mpu-and-atmel-644pa.html?___store=de_de).

There is active and friendly support on the RCgroups forums, and it is often
stated that this board is able to stabilize almost anything small-scale.
However, development is confined by the hardware used (who knows for how long
HK will be making the kk-board), the development style (code drops on release
by the single author every couple months), and the word 'VTOL' in the name. It
would be nice to really have a general controller not for just one niche,
sacrificing some features (Tailsitter option, Heli-style rotor control) for a
more general approach (also with more than the 3 flight modes which OAV has).
Additionally, OAV does not support a derivative gain (the D in PID), arguing
that this form of feedback is unnecessary for vehicles that naturally have a
high damping factor due to having a higher mass or large wings. The accuracy of
this statement is debatable, and to provide a more general approach a D gain is
nice to have (if only for meta-stable systems or niche cases like multicopters).

Another feature of OAV is that it can entirely be configured via the onboard
LCD screen (that is a miracle, really). Since version 1.5, you may also use
(exclusively) Microsoft Excel to configure it, using a macro-based GUI in .xlsx
format (yes, really). I am not at the point to even think about this by a long
shot, but it would be nice to have a graphical configuration interface on a
computer that can set options on the board over a serial or even wireless
connection.

