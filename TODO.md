# TODO

- [] Consider using a more proper Observer pattern by creating a GameObject trait with process_event method that the various game objects (dog, boy, Obstacle?) would implement

## Bugs

### Dog hits platform on jump

If dog turns around right when dropping from platform, it can hit the platform.

Reproduce by starting to run right when dog is dropping from platform
