# Day 13

It's time to write the shaders for the sprite renderer.

*In src/gl_utils/shaders/sprite.vert*

```glsl
#version 330 core

layout (location = 0) in vec4 vertex;

out vec2 tex_coords;

uniform mat4 transf;
uniform mat4 ortho;

void main() {
    tex_coords = vertex.zw;
    gl_Position = transf * ortho * vec4(vertex.xy, 0.0, 1.0);
}
```

*In src/gl_utils/shaders/sprite.frag*

```glsl
#version 330 core

in vec2 tex_coords;
out vec4 color;

uniform sampler2D image;

void main() {
    color = texture(image, tex_coords);
}
```

I also decided to modify the shaders for the image rendering code. The code that determines what pixel each fragment is assigned to probably belongs in the vertex shader, rather than the fragment shader.

*In src/gl_utils/shaders/fb_to_texture.vert*

```glsl
#version 330 core

layout (location = 0) in vec4 vertex;

out ivec2 pix_coords;

uniform int s_width;
uniform int s_height;

int determine_pix(float scale, int length) {
    float real_scale = (scale + 1.0) / 2;
    return int(real_scale * length);
}

void main() {
    pix_coords = vec2(determine_pix(tex_coords.z), determine_pix(tex_coords.w));
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
}
```

*In src/gl_utils/shaders/fb_to_texture.frag*

```glsl
#version 330 core

in ivec2 tex_coords;
out vec4 color;

uniform int s_draw_len;
uniform sampler1d s_draws;
uniform fvec4 bg_color;

void main() {
    color = bg_color;

    // iterate over draw instructions and see which ones involve this pixel
    int base_index;
    int kind;
    ivec4 d_color;
    ivec4 coords;
    for (int i = 0; i < s_draw_len; i++) {
        base_index = i * 3;
        kind = texelFetch(s_draws, base_index, 0).x;
        coords = texelFetch(s_draws, base_index + 1, 0);
        d_color = texelFetch(s_draws, base_index + 2, 0);

        // if our coords are in the designated zone, we are good
        if ((kind == 1 && pix_coords.x == coords.x && pix_coords.y == coords.y) || // pixel
            ((kind == 2 || kind == 3) && pix_coords.x >= coords.x && pix_coords.x <= coords.x + coords.z
                       && pix_coords.y >= coords.y && pix_coords.y <= coords.y + coords.w) || // rectangle) {
            color = vec4(d_color.x / 255, d_color.y / 255, d_color.z / 255, d_color.w / 255);
            break;
        }

    }
}
```

Now, we just need to set up compilation for these shaders.

```rust
// sprite renderer program
lazy_static::lazy_static! {
    static ref SPRITE: Program = {
        let vert_source = include_str!("./shaders/sprite.vert");
        let frag_source = include_str!("./shaders/sprite.frag");
        let vert = Shader::from_source(&vert_source, ShaderType::Vertex)
            .expect("Vertex shader compilation failed");
        let frag = Shader::from_source(&frag_source, ShaderType::Fragment)
            .expect("Fragment shader compilation failed");
        Program::new(&[vert, frag]).expect("Shader linking failed")
    };
}

/* ... */

impl GlRenderer {
    // create a new GlRenderer
    pub fn init() -> Result<GlRenderer, LitError> {
        const width: u32 = 800;
        const height: u32 = 600;

        /* ... */

        // create the window
        let window = video_context
            .window("Lost in Time", width, height)
            .opengl()
            .build()?;

        /* ... */
 
        // initialize the sprite render process
        let ortho = Orthographic3::<GLfloat>::new(0.0, width as GLfloat, height as GLfloat, 0.0, -1.0, 1.0);
        SPRITE.set_uniform("ortho", ortho.into_inner());

        /* ... */
    }
}

impl Renderer for GlRenderer {
    /* ... */

    fn draw_sprite(
        &mut self,
        img: &ImgTexture,
        position: Point2<GLfloat>,
        size: Point2<GLfloat>,
        rotation: GLfloat,
    ) -> Result<(), LitError> {
        SPRITE.activate();

        let mut transform = Transform3::<GLfloat>::identity();

        // shift position
        transform *= Translation3::from(Vector3::new(position.x, position.y, 0.0));

        // shift size
        transform *= Translation3::from(Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0));

        // rotate matrix
        transform *= Rotation3::from_axis_angle(&Unit::new_normalize(Vector3::z()), rotation);
        transform *= Translation3::from(Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0));

        // resize with size
        transform *= Transform3::from_matrix_unchecked(Matrix4::from_diagonal(&Vector4::new(
            size.x, size.y, 1.0, 1.0,
        )));

        SPRITE.set_uniform("transf", transform);

        unsafe { gl::ActiveTexture(gl::TEXTURE0) };
        img.bind();

        self.quad.bind();
        self.quad.draw();
        self.quad.unbind();

        Ok(())
    }
}
```

Now, let's actually put something in the drawing loop that draws the sprite we set up earlier. Remember the `Game` object we made way back when? It's time to put that to use.

*In src/game.rs*

```rust
use crate::{GameData, LitError, Resource};

pub struct Game {
    data: GameData,
}

impl Game {
    pub fn new(data: GameData) -> Self {
        Self { data }
    }

    pub fn get_resource<T: Resource>(&mut self, id: u32) -> Result<&T, LitError> {
        self.data.get_resource(id)
    }
}
```

*Modifications to src/script/mod.rs*

```rust
impl GameData {
    /* ... */

    pub fn get_resource<T: Resource>(&mut self, id: u32) -> Result<&T, LitError> {
        self.resource_dict.as_mut().unwrap().load_res(id)
    }
}
```

*Modifications to src/main.rs*

```rust
fn classic_main() -> Result<(), LitError> {
    let mut renderer = GlRenderer::init()?;
    let mut data_file = BufReader::new(fs::File::open(
        env::args()
            .skip(1)
            .next()
            .ok_or_else(|| LitError::NoDataFile)?,
    )?);
    let game_data = script::GameData::read(&mut data_file)?;
    println!("{:?}", &game_data);
    let game = Game::new(game_data);

    renderer.main_loop(&game)
}
```

Now, let's just load the image at ID `0` and try displaying it.

*Modifications to src/gl_utils/render.rs*

```rust
impl Renderer for GlRenderer {
    fn main_loop(&mut self, mut game: Game) -> Result<(), LitError> {
        /* ... */

        // main loop
        'lit: loop {
            /* ... */
      
            self.draw_sprite(
                game.get_resource::<ImgTexture>(0)?,
                Point2::new(40.0, 40.0),
                Point2::new(100.0, 100.0),
                0.0,
            );

            self.window.gl_swap_window();
        }

        Ok(())
    }

    /* ... */
}
```

When we run it, we get our first shader error:

```
thread 'main' panicked at 'Vertex shader compilation failed: Msg("0:19(34): error: `tex_coords\' undeclared\n0:19(34): error: type mismatch\n0:19(20): error: no matching function for call to `determine_pix(error)\'; candidates are:\n0:19(20): error:    int determine_pix(float, int)\n0:19(15): error: cannot construct `vec2\' from a non-numeric data type\n\u{0}")', src/gl_utils/texture/render.rs:16:20
```

I forgot that shaders were compiled at runtime. This was followed by a short period of firefighting bugs in the shaders. I also took some time to clean up the codebase using clippy and take care of some warnings. This took the rest of the day.
