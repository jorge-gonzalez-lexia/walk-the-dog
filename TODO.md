# TODO

- [] Convert to using more published events
  - DogTooFar and DogTooClose maybe during Jumping?
  - DogLandsOnGround
  - See other events below
- [] Consider using a more proper Observer pattern by creating a GameObject trait with process_event method that the various game objects (dog, boy, Obstacle?) would implement

Formal events:

- Flee. The event here really is Boy starts running (or game starts)
- Jump. This is actually hitting a mark (which may trigger a Jump)
- Land
- OffPlatform
- Update. But this one we ideally do not fire continuously.
- Worry

Informal events:

- Toggle (reverse) direction
- JumpToRun: when landing from a Jump onto the Ground. (Landing on a platform is handled via formal event when Dog hits the platform). But we could maybe combine these too.

What about Land (on ground)

- Jumping.update detects dog is falling and has reached floor (either ground or not). Queues Land event. It does not transition to a new state yet.
- Jumping.update returns and stack unwinds etc
- On next update iteration, Dog reacts to Land event by calling dog.land which sets the floor and transitions to Running
- Rest of updates continue, but now Dog is running on new floor

What about a formal event like Land?

- Platform.navigate detects dog has hit platform. We could raise an event, but it will be off by one frame. We can alter the bounding box height to compensate? So we queue the Land event
- Next update iteration:
  - Platform reacts to Land event by setting has_dog to true. (Note the event must have an identifier so Platform instance can match to that)
  - Dog reacts to Land event

## Issues

- Issue: when dropping from platform onto stone while returning. Need a left marker on the right of the stone that reaches past platform
- Issue: Hitting OffPlatform while Jumping. Hard to reproduce
  - Dog Running on platform (offscreen) and toggles direction. This can happen when Dog starts fleeing when offscreen (so Boy catches up to Dog and Dog sprints away, probably to fast?)
  - Dog then Jumps (why? it was already on the platform)
  - OffPlatform while Jumping
