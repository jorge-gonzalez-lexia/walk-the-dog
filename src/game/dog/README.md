# Dog

## GameState(Ready)

```mermaid
stateDiagram-v2
  [*] --> Running
  state running_update <<choice>>
  Running --> running_update: Update
  running_update --> Running: too close
  running_update --> Returning: too far
  Running --> Jumping: Jump
  Running --> Fleeing: Flee

  Jumping --> Running: Land
  state jumping_update <<choice>>
  Jumping --> jumping_update: Update
  jumping_update --> Jumping: above floor
  jumping_update --> Running: on floor

  state returning_update <<choice>>
  Returning --> returning_update: Update
  returning_update --> Running: too close
  returning_update --> Returning: too far
  Returning --> Fleeing: Flee

  Returning --> JumpingReturn: Jump
  state jumping_return_update <<choice>>
  JumpingReturn --> jumping_return_update: Update
  jumping_return_update --> JumpingReturn: above floor
  jumping_return_update --> Returning: on floor
  JumpingReturn --> Returning: Land


```

- Game listens for right arrow key
- `BoyState(Idle)`. Boy is idling at left of screen
- `DogState(Running)`. Dog is running away from boy

Game Event: On arrow right, `WalkTheDogState(Ready)` calls:

- boy::run_right() => `Boy Event::Run` => `Boy(Running)`
- dog::flee() => `Dog Event::Flee`

### DogState(Running):

- On `Dog Event::Update` and dog has run too far off screen => Transitions to `Dog(Returning)`
- On `Dog Event::Flee`:

  - if dog is off screen => Transition to `DogState(ReturningToFlee)`
  - if dog is on screen => Transition to `DogState(Fleeing)`

### DogState(Returning)

- On `Dog Event::Update` and dog has approached too close to boy => Transition to `DogState(Running)`
- On `Dog Event::Flee` > Transition to `DogState(Fleeing)`

## GameState(Walking)

```mermaid
stateDiagram-v2
  state running_flee <<choice>>
  [*] --> running_flee: Flee
  running_flee --> Fleeing: close
  running_flee --> ReturningToFlee: too far

  Fleeing --> Worried: Worry
  ReturningToFlee --> Worried: Worry

  Fleeing --> JumpingFlee: Jump
  JumpingFlee --> Fleeing: Land
  JumpingFlee --> Worried: Worry*
  state jumping_flee_update <<choice>>
  JumpingFlee --> jumping_flee_update: Update
  jumping_flee_update --> JumpingFlee: above floor
  jumping_flee_update --> Fleeing: on floor

  state returning_to_flee_update <<choice>>
  ReturningToFlee --> returning_to_flee_update: Update
  returning_to_flee_update --> ReturningToFlee: too far
  returning_to_flee_update --> Fleeing: close

  ReturningToFlee --> JumpingFleeReturn: Jump
  JumpingFleeReturn --> ReturningToFlee: Land
  JumpingFleeReturn --> Worried: Worry*
  state jumping_flee_return_update <<choice>>
  JumpingFleeReturn --> jumping_flee_return_update: Update
  jumping_flee_return_update --> JumpingFleeReturn: above floor
  jumping_flee_return_update --> ReturningToFlee: on floor
```

- `BoyState(Running)`. Boy is chasing Dog. If Dog is initially off screen, it returns on screen and then flees. Otherwise, it flees.
- if Boy transitions to `BoyState(KnockedOut)`, `GameState(Walking)` detects that and => Transitions to:
  - `GameState(GameOver)`
  - `BoyState(KnockedOut)` (already in this state)
  - Dispatches `Dog Event::Worry`

### DogState(ReturningToFlee)

- On `Dog Event::Update` and dog has approached too close to boy => Transition to `DogState(Fleeing)`
- On `Dog Event::Worry` => Transition to `DogState(ReturningWorried)`

### DogState(Fleeing)

- Dog is fleeing from boy at the same speed as boy and so always on screen

- On `Dog Event::Worry` => Transition to `DogState(RunningWorried)`

## GameState(GameOver)

```mermaid
stateDiagram-v2
  [*] --> ReturningWorried: Worry
  state returning_worried_update <<choice>>
  ReturningWorried --> returning_worried_update: Update
  returning_worried_update --> ReturningWorried: far
  returning_worried_update --> RunningWorried: too close

  ReturningWorried --> JumpingWorriedReturn: Jump
  JumpingWorriedReturn --> ReturningWorried: Land
  state jumping_worried_return_update <<choice>>
  JumpingWorriedReturn --> jumping_worried_return_update: Update
  jumping_worried_return_update --> JumpingWorriedReturn: above floor
  jumping_worried_return_update --> ReturningWorried: on floor

  state running_worried_update <<choice>>
  RunningWorried --> running_worried_update: Update
  running_worried_update --> RunningWorried: close
  running_worried_update --> ReturningWorried: too far

  RunningWorried --> JumpingWorried: Jump
  JumpingWorried --> RunningWorried: Land
  state jumping_worried_update <<choice>>
  JumpingWorried --> jumping_worried_update: Update
  jumping_worried_update --> JumpingWorried: above floor
  jumping_worried_update --> RunningWorried: on floor
```

- `BoyState(KnockedOut)`
- `DogState(RunningWorried | ReturningWorried)`. Dog loops between running away and returning

### DogState(ReturningWorried)

- Dog returns to boy

- On `Dog Event::Update` and dog has approached too close to boy => Transition to `DogState(RunningWorried)`

### DogState(RunningWorried)

- Dog worriedly runs away from boy

- On `Dog Event::Update` and dog has gone too far => Transition to `DogState(ReturningWorried)`
