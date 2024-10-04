# TODO

- [] Consider using a more proper Observer pattern by creating a GameObject trait with process_event method that the various game objects (dog, boy, Obstacle?) would implement

## Bugs

### Hitting OffPlatform while Jumping.

- Dog Running on platform (offscreen) and toggles direction. This can happen when Dog starts fleeing when offscreen (so Boy catches up to Dog and Dog sprints away, probably to fast?)
- Dog then Jumps (why? it was already on the platform)
- OffPlatform while Jumping

- Issue: when dropping from platform onto stone while returning. Need a left marker on the right of the stone that reaches past platform
