Dog states

- Document states (note interesting effect of velocity 1/-1 when Flee happens while offscreen)
- Issue: when dropping from platform onto stone while returning. Need a left marker on the right of the stone that reaches past platform
- Issue: Hitting OffPlatform while Jumping. Hard to reproduce

Other

- [] jumping animation
- [] draw dog running left when vx<0
- [] How would it work to make it more redux where all transitions were triggered by an Event. Is there a single master event list. Or a master event handler that dispatches to game object handlers with object specific events (transformed). Or?
  It is not great that an Update event can transition to other states. The transition method is not clear about that. In general, when an event can lead to several different states, things get unclear. you have to dive into the method. But the borrow checker makes it super difficult for one event being processed to fire of another event.
