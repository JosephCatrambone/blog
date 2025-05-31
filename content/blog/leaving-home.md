+++
title = "Leaving Home"
date = "2014-05-08"

[taxonomies]
tags=["Math,","Status"]
+++

Gliese 581 g is an Earth-like planet orbiting around Gliese 581 in the constellation Libra. It's around 20.3 lightyears away -- 1.92048727x10^17 meters, or 1.19333546x10^14 miles. Let's assume that a ship accelerates at around 2.9g, the maximum sustainable by humans for any prolonged period. As a note: we're only going to calculate the time to get half-way there. We can't keep accelerating at 2.9g for the whole trip, since we need to land on the other planet with a velocity of 0. If we keep accelerating, we'll plow into the planet with tremendous force and kill everyone on board. Instead, let's assume that we turn around half way and start accelerating in the opposite direction.What information do we have?

- Distance: 20.3 lightyears = 1.9x10^17meters
- Acceleration: 2.9g = 9.81m/s^2 = 28.449m/s^2

How do we figure out how long it will take? If it we knew the speed, we could use speed = distance/time, then plug in speed and distance to get time. Instead, we need to calculate the final speed of our ship.

- Acceleration = deltaVelocity / deltaTime
- Velocity = deltaPosition / deltaTime
- Acceleration = delta (deltaPosition / deltaTime) / deltaTime = deltaPosition / deltaTime^2
- deltaPosition = positionFinal - positionInitial
- Acceleration = (positionFinal - positionInitial) / deltaTime^2
- 28.449m/s^2 = (9.5x10^16m - 0m) / deltaTime^2 \[Remember, half way, so we divide positionFinal by two.]
- 1/28.448m/s^2 = deltaTime^2 / 9.5x10^16m
- 9.5x10^16m/28.448m/s^2 = deltaTime^2
- 3339426321709786.0s^2 = deltaTime^2
- sqrt(3339426321709786.0s^2) = deltaTime
- 57787769.65509039 seconds = deltaTime
- 1.83122 years = deltaTime

A perfectly reasonable 3.6 years of travel. Let's see how that works out for 1g.

- 9.5x10^16m/9.81m/s^2 = deltaTime^2
- 9683995922528032.0s^2 = deltaTime^2
- sqrt(9683995922528032.0s^2) = deltaTime
- 98407296.08381703 seconds = deltaTime
- 3.1184 years = deltaTime

Six years at normal human gravity. This doesn't take into account the effects of time dilation.
