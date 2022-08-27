Ignition is trying to be (one day), a beautifully simple graphics engine. Hopefully....

## TODO
### Refactoring
- Add benches that compare performance with and without ignition
- Try loading shaders in a more friendly way
- Add back VertexGroups

### ECS
- Add entity
- Add component
- Remove component
- Toggle component
- Implement the idea behind RendererCommands using ECS

### Refactoring
- More data-driven programming
 
## Code layout
- *lib.rs*: Home of the infamous **Engine** with it's configuration and constructors
  - *manifestation.rs*: Definition of the Renderer trait
    - *lift_off.rs*: List of useful utilities for initializing a Renderer
      - *headless.rs*: Headless renderer (run without any output)
      - *screen.rs*: Screen renderer (linked to a window)
      - *image.rs*: Image renderer (linked to an image on the cpu)

    - *artist.rs*: A collection of small functions that aid the rendering process
      - *commands.rs*: Record a command buffer (which regroups all the necessary steps that the gpu has to take to render everything)
      - *pipeline.rs*: Create a pipeline (which describes how a particular object should be rendered)

    - *nostalgia.rs*: Everything to do with allocating memory on the gpu (think buffers or textures)

  - *liberty.rs*: Configuration