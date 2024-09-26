# Dog

```mermaid
stateDiagram-v2
  [*] --> Running
  Running --> Fleeing: Flee
  Running --> Running: OffPlatform

  state toggle_direction <<choice>>
  Running --> toggle_direction: Update
  toggle_direction --> Running: too far
  toggle_direction --> Running: too close

  Running --> Jumping: Jump
  Jumping --> Jumping: Update
  Jumping --> Running: Land

```

## Platform Navigation

A segment with a platform onto which a Dog must jump has a mark set on either side of the platform. The mark may be on the Platform object itself (or, if a Barrier immediately precedes a Platform, on a Barrier). The mark (left or right) is an indicator that the Dog should Jump. Hence, both a Barrier and Platform Obstacle has logic that fires the Dog Event::Jump when the Dog hits a mark.

The Platform Obstacle also checks if the Dog hits the platform itself, which we assume must mean the Dog was on the descending cycle of a jump and has landed on the platform (since a Dog should never otherwise hit the platform). The Platform then:

- sets its `has_dog` flag
- notifies the Dog via `dog.on_platform`
- which in turn fires `Dog Event::Land`
- which calls `state.land_on`
  - which calls `context.set_floor`
    - which stores the floor value
      At this point, every context update will check to ensure the Dog does not drop below the Platform floor.
  - and returns `DogState(Running)` (or similar)
