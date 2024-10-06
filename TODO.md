# TODO

- [] understand the weirdness of adding items to event_subscribers. I wonder if event subscribers need to be Rc<RefCell>. Maybe we just have the Box<dyn EventSubscriber> directly (clones)
  - \*\*walk::new lines 46-57 (esp need of EventSubscriber wrappers)
  - \*\*walk::generate_next_segment lines 128-133
  - to a lesser extent:
    - walk::navigate_obstacles
    - walk::process_events
- [] weird walk::reset lines 89-93

## Bugs
