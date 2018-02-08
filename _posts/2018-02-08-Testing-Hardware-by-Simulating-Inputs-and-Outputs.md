I am building a drone flight controller from scratch. It runs on a fairly small
microcontroller. Like many projects involving hardware, it is somewhat
difficult to test it. While thinking about it, I had this pretty funny
idea and I am not even sure it is feasible.

Bear with me, the setup is a little complex. First, some context about what the
flight controller does:

The flight controller gets some input from the operator (think game
controller). It also measures the current attitude and angular velocities using
some Inertial Measurement Unit solution (technically more complex, involving
sensor data fusion, but the result is just attitude and angular  velocities).
Using some control mechanisms (cascaded PID on attitude and velocity in each
axis, where the setpoint is the input from the user), it figures out a needed
response that is sent to the motors, which then spin up or down, affecting
attitude and angular velocity. If the system works correctly, the craft
stabilizes in the air, reacts to the inputs by the operator, and is stable
against disturbance by wind gusts or collisions or differences in motor power.

Note that there is a closed loop here: the sensor measures the environment,
then then takes some steps to influence it, observes the effect, and keeps
adjusting.

It is hard to test the controller without actually flying on a drone. Here is
the funny idea: why not mock the input/output to the flight controller?

## Simula-ception

The motor commands from the controller are continuously sent to a physics
simulation, which simulates the effect of the commands on some vehicle in real
time and (if possible) displays a (crude) render of it. During operation, the
fake IMU on the simulated craft takes some measurements of attitude and angular
velocity, adds some noise and drift, and sends them back to the flight
controller. There is another board between PC and flight controller that acts
as the middleman (it needs to speak I2C and PWM with the flight controller, and
Serial with the PC). To the controller, it pretends to be the IMU unit (on the
I2C bus, this is some work, but possible). It also receives the motor commands.
To the PC, it acts as the source of the commands for the simulation and as a
receiver of the fake IMU in the simulation.

The flight controller should ideally not notice that anything is different from
normal operation. The loop rate should not be affected, but I do not know if
simulations on a computer and the serial communication might be a bottleneck
here.

It would be great to be able to simulate wildly different configurations. Not
Kerbal Space Program level, but I want to change the airframe, change weights
and motor power, possibly even propeller type. It would be great to attach
motors to other configurations like blimps or boats (which the controller can
balance as well) and simulate those. In my book, it would be ok to have to
edit a 3D model for this.

In any case, it should be possible to do some IPC with another program or read
the serial port directly in order to interface with the mock board. The
simulation needs to read the values from that board, simulate, measure, send
data back in 'real time'.

I hope I described it clear enough, but here is a nice image/schematic:

<img src="https://raw.githubusercontent.com/barafael/raPID/master/controller_simulator_test_rig.png" alt="schematic" class="inline"/>

[Image link in case it is too small](https://raw.githubusercontent.com/barafael/raPID/5d0a6826fd667db651619c629991c87a2e64a979/controller_simulator_test_rig.png)

## Tower of Babel (Language/Protocol Confusion)

The middleman board needs to talk I2C (or whatever the flight controller thinks
the IMU is using), PWM (to pretend to be a motor) and serial (to talk to the
computer that runs the simulation). PWM is simple enough, such an interface
exists on the flight controller anyway to read from the RC receiver. But to
fake an IMU, the precise protocol of the specific IMU needs to be adhered to.
It would be possible to mount an actual hardware unit on the middleman board
and just manipulate the data that comes from it, while leading everything else
through. As for the serial connection, I need to think about an appropriate
format - how to send the data quickly, while preserving debuggability (binary
vs. string) and how do I ensure that the format is open for extension? There
should also be the option to send telemetry from the controller over the
middleman board to the PC, so some general protocol extensibility would be
nice.  tl;dr I need to run a pretty accurate physics simulation of a vehicle
(think multicopter) that can react to input from the serial port (or IPC) and
send data about the vehicle (attitude, angular velocity) back over the same
channel.
