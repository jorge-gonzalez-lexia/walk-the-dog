# TODO

- [] GameEvent::BoyKnockedOut should only fire once
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

## Issues

- Saw at least one time dog running right but facing left
- Issue: when dropping from platform onto stone while returning. Need a left marker on the right of the stone that reaches past platform
- Issue: Hitting OffPlatform while Jumping. Hard to reproduce
  - Dog Running on platform (offscreen) and toggles direction. This can happen when Dog starts fleeing when offscreen (so Boy catches up to Dog and Dog sprints away, probably to fast?)
  - Dog then Jumps (why? it was already on the platform)
  - OffPlatform while Jumping
