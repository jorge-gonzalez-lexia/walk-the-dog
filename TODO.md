Dog states

- handle Land while Running
- Document states (note interesting effect of velocity 1/-1 when Flee happens while offscreen)
- Check all segments

Other

- [] add jump l and r markers to obstacles (from segment creation)
- [] try single landing event on platform. When dog is over platform, it becomes the floor. When not over platform, revert to ground floor.
- [] when dog no longer on platform, decide whether to jump based on upcoming marker
- [] refactor dog.navigate. Glitchy when dropping from platforms. Dog needs to be smarter with obstacles ahead. Each segment
- [] Flee event when jumping or jumping return
- [] glitches when returning worried and jumping and landing or when platforms are close to each other
- [] jumping animation
- [] returning state should draw dog running left
- [] How would it work to make it more redux where all transitions were triggered by an Event. Is there a single master event list. Or a master event handler that dispatches to game object handlers with object specific events (transformed). Or?
  It is not great that an Update event can transition to other states. The transition method is not clear about that. In general, when an event can lead to several different states, things get unclear. you have to dive into the method. But the borrow checker makes it super difficult for one event being processed to fire of another event.

Game Ready

- [x] Running and Returning

Game Walking

- [x] Running, Flee -> if on screen, flee
- [x] Running, Flee -> if off screen, return
- [x] Returning, Flee -> if on screen, toggle direction and start fleeing
- [x] Returning, Flee -> if off screen, keep returning
- [x] Returning, Update -> if on screen and in flee mode, Flee
- [x] Returning, Update -> if off screen, keep returning

Game Over

- [x] Running and Returning
