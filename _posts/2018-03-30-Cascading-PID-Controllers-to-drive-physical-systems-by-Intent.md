If that title does not make sense to you, then that might be because it is rather airy.
But at the end of this article, it might make more sense.
Moreover, you might have a better understanding of cascaded PID and PID in general.

## Decoupling Intent from physical units

Often, a PID controller is described pretty detailed in what it does and how it does it.
While that is important, the big picture is crucial to understand how a PID-Controller can interact with other systems (like other Controllers).

Here is an alternate, more general description: A PID-Controller allows you to directly steer a physical property of a system instead of driving an actuator that might directly or indirectly influence this physical property.
What this means is that you can set a desired value for a physical property and not worry about how to drive any actuator. That is a form of abstraction.
This perspective makes it easier to understand cascades of PID-Controllers.

There is even more to the story! Usually, an actuator sets the rate of some process.
That means that a chain of controllers will have each stage control aspects of the system which are related by differentiation or integration.
We will use the example of a multicopter later, because it lends itself well to multiple PID cascades.

## Deriving/Integrating physical units in a concrete example

Lets look at some properties of a quadcopter. The force from a propeller causes the craft to roll or pitch, i.e. gives it angular momentum. If a quadcopter is at some angle that is not completely flat, it is going to move in some direction with some velocity.
On the control unit, there might be sensors or data fusion for angular momentum and angular acceleration, angular position, global velocity, and global position.

We want to move a multicopter from point A to point B in global space. We have a GPS sensor to measure position in global space.
That means we could use a PID-controller (with limited output) to fly in the direction of B from A with a certain velocity.
Ideally, that velocity should be controlled somehow - by another PID-controller that gets the desired velocity and outputs the necessary angle of the craft.
That angle can be controlled by a PID controller that measures (or rather, uses data fusion techniques to measure) the absolute angle relative to gravity and outputs a desired angular rate to put the system into that state.
And finally, the last PID-controller can measure the angular rate from a gyro and calculate the necessary force the motors need to exert on the quadcopter.

At each stage, there needs to be a sensor (or sensors+data fusion). The desired property(like angular absolute position) is in some way an integral of the controlled variable(angular momentum).
Note that the inner loops always run faster than the outer loops, partly because they have faster sensors (a gyro is faster than data fusion, and GPS is really slow), and partly because they need to sample and actuate quicker than the upper layer to bring the system into the desired state.
