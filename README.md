### freeliner-rs
Freeliner is a geometric animation software. The first version was made with Processing and you can find it [here](https://github.com/maxdee/alc_freeliner). Freeliner-rs is complete rewrite / rethink in Rust. This is extreme **Work In Progress**.

The rest of the document are notes and stuff.

#### Animation Nodes
A set of nodes that turns some geometry into rendering lists.
- livecodeable?

```
animation(event, LOOP || TRIGGER) // starts an animation event
 .group(random(selected)) // picks group from the "selected" ie any group associated to the render
 .segments(ALL)           // select all the segments         
 .iterate(5)              // draw 5 things on 1 segment
 .get_position()          // get position from segment
 .dots(size: 20, noFill())// draw dots on positions
 .position_noise(0.3)     // add noise to positions
 .squares(size: 30)       // draw squares on positions
```

#### Commands
`context_name command_name args`
The current plan is that the bulk of freeliner-rs would be based on commands.
Mouse and keyboard input makes commands,
Optionnaly return a string to display with the result of the command.
`command.execute(&mut self, state: &mut State) -> Result<Option<&str>, &str>`

#### Contexts
Context geometric state and "render state" grouped together,
Things would be broken out into contexts,
ie
 - animation context
 - control context -> controls animation context

#### todo
- use cfg-if? for native vs browser stuff

#### done
- use serde_json to save "state" and load back, magical compared to old freeliner
