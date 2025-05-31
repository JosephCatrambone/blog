---
title: 'Unity 4.1 Particle Collision Tracking'
description: ""
published: 2013-07-01
redirect_from: 
            - https://www.josephcatrambone.com/?p=319
categories: "Game, Rant"
hero: ../../../defaultHero.jpg
---
I've had the pleasure of working with Unity 4.1 recently for a game jam. One of the confounding factors was this: although the Shuriken particle system collided nicely with meshes and physical bodies, no collision events were generated. (This is set to be fixed in Unity 4.2, no ETA.) This proved problematic, as I needed to count the number of particles which hit a specific target. "No big deal," I thought. "I'll just send out a tracer particle every n particles and let it do the checking. It might even be faster." I came up with the following code:

```
// Since Unity 4.1 doesn't support having shuriken particles collide/send signals, we make due with simulated 'representative' particles.
void spawnCollisionDetector() {
	// If, for some reason, another object deletes our simulation object, recover by making a new one.
	if(simulationObjectPool[simulationObjectIndex] == null) {
		Debug.LogWarning("Sim Particle at index " + simulationObjectIndex + " has been nulled.  Recovering with respawn.");
		simulationObjectPool[simulationObjectIndex] = (GameObject)Instantiate(simulationParticle);
	}

	// Take a sim particle from our pool, give it the same properties as our stream, let it go.  Advance index to next particle.
	simulationObjectPool[simulationObjectIndex].transform.position = new Vector3(beam.transform.position.x, beam.transform.position.y, beam.transform.position.z);
	simulationObjectPool[simulationObjectIndex].transform.forward = new Vector3(beam.transform.forward.x, beam.transform.forward.y, beam.transform.forward.z);
	simulationObjectPool[simulationObjectIndex].rigidbody.velocity = beam.transform.forward * beam.startSpeed;
	simulationObjectIndex = (simulationObjectIndex + 1) % simulationObjectPool.Length;
}
```

This works well when the forward is close to center, but the discrepancy between the probes and the actual particle stream get larger and larger as the forward diverges from center. Normalization doesn't seem to make much of a difference. I'm still a bit stuck.
