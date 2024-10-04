# TODO

- [] Consider using a more proper Observer pattern by creating a GameObject trait with process_event method that the various game objects (dog, boy, Obstacle?) would implement

Landing

- Jump
- DogContext: Dog reaches floor (always Ground, but no explicit check)
  - Can be in Jumping (when jumped over stone) or Running (when dropping from platform)
  - DogContext publishes GameEvent::DogLanded
  - dog.process_event -> Event::LandOnGround -> DogState(Jumping).land_on_ground()
    - Transition to Running
    - Note floor never changes from Ground
- OR Platform.navigate: Dog first hits platform (assume from top, but no explicit check)
  - Always in Jumping state
  - Platform publishes GameEvent::DogLandedOnPlatform {id, platform_top}
  - Platform.process_event -> set has_dog to true if event platform id matches current platform id
  - dog.process_event -> Event::DogLand(platform_top) -> DogState(Jumping).land_on(platform)
    - Floor set based on platform top
    - Transition to Running
  - Eventually dog reaches end of platform. OffPlatform -> eventually lands on ground (1st scenario)
    - There's no state change here. It is a no-op

## Bugs

### LandOn event fires in Running state.

- Reproduce:
  1. set segment REPEAT to 0
  2. Run boy into rock. When Dog is on the return path, somehow hits this state. (If boy dies earlier, this does not happen)

Issue is that speed gets set to -1 when fleeing, which means the dog moves at 3 relative to the left scrolling of -4. If we set to < -4 (.e.g. -6), Dog will run to boy in look right, but when it turns around, it will go to +8 and run too fast. So either:

1. Adjust moving left to account for scrolling. And also adjust turn_around to account for scrolling.

- Start with scrolling velocity = 0
- On flee, set scrolling velocity to -4
- On worry, set back to 0

Moving left: dog vx + scrolling vc < 0
Turn around: if scrolling == 0, vx toggles between -4 and 4. Otherwise toggle between -8 and 0 (in reality it will only ever toggle once from -8 to 0)

### Hitting OffPlatform while Jumping.

- Dog Running on platform (offscreen) and toggles direction. This can happen when Dog starts fleeing when offscreen (so Boy catches up to Dog and Dog sprints away, probably to fast?)
- Dog then Jumps (why? it was already on the platform)
- OffPlatform while Jumping

- Issue: when dropping from platform onto stone while returning. Need a left marker on the right of the stone that reaches past platform
