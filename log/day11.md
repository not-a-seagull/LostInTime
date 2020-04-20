# Day 11

I realized that we would need to make the drawing handle mutable, however, the handle reference naturally decays. I had to do:

```rust
// as draw handle minus the need for the state  
#[inline]
fn as_draw_handle_mut_no_state<'a>(&'a mut self) -> Result<&'a mut dyn DrawHandle, LitError> {
    match self {
        &mut BytecodeObject::ImgMaterial(ref mut i) => Ok(i),
        _ => Err(LitError::IncorrectDataType(DataType::Unknown, DataType::ImgMaterial)),
    }
}

pub fn as_draw_handle_mut<'a>(
    &'a mut self,
    state: &'a mut ParserState,
) -> Result<&'a mut dyn DrawHandle, LitError> {
    match self {
        &mut BytecodeObject::ImgMaterial(ref mut i) => Ok(i),
        &mut BytecodeObject::VarInvocation(i) => {
            let mut var = state.get_variable_mut(i)?;
            var.as_draw_handle_mut_no_state()
        }
        _ => Err(LitError::IncorrectDataType(
	    self.data_type(state),
	    DataType::ImgMaterial,
        )),
    }
}
```

I also modified `eval::eval`, so that if it encounters the bytecode `0` it returns `Ok(false)`, and `Ok(true)` on other bytecodes. This ensures that we don't erroneously get an error result if we just reach the end of the stream.

This seems to run; however, the image is stored in the `ParserState`, which is dropped at the end of the game read function. I'd say that it would be prudent to store this in the `GameData`, which is the end result of the evaluation process. 

*Modifications to src/script/mod.rs*

```rust
impl ParserState {
    /* ... */

    pub fn into_resource_dict(self) -> Result<ResourceDictionary, LitError> {
        let mut rd = ResourceDictionary::new();
        let ParserState { variables, dependency_relations, img_material_ids, .. } = self;

        Ok(rd)
    }  
}

/* ... */

#[derive(Debug)]
pub struct GameData {
    name: String,
    resource_dict: Option<ResourceDictionary>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            name: String::from("Unnamed"),
            resource_dict: None,
        }
    }

    /* ... */
}

impl Bytecode for GameData {
    fn read<T: Read>(stream: &mut T) -> Result<Self, LitError> {
        let mut data = Self::new();
        let mut state = ParserState::new();

        'parse: loop {
            match eval::eval(stream, &mut data, &mut state) {
                Err(e) => {
                    eprintln!("Error encountered: {}", e);
                    break 'parse;
                }
                Ok(false) => break 'parse,
                Ok(true) => {}
            } // go until error is encountered
        }

        data.resource_dict = Some(state.into_resource_dict()?);

        Ok(data)
    }
}
```

This compiles, and in the output we see:

```
GameData { name: "Lost in Time", resource_dict: Some(ResourceDictionary { next_id: 0, loaded_ids: [], prev_loaded_ids: [], mat_img: {}, res_img: {} }) }
```

We can now generate an empty resource dictionary. Let's fill it.

*Modifications to src/script/mod.rs*

```rust
impl ParserState {
    /* ... */

    pub fn into_resource_dict(self) -> Result<ResourceDictionary, LitError> {
        let mut rd = ResourceDictionary::new();
        let ParserState {
            mut variables,
            dependency_relations,
            img_material_ids,
            ..
        } = self;

        insert_materials::<ImgMaterial>(
            &mut rd,
            &dependency_relations,
            &mut variables,
            img_material_ids,
        )?;

        Ok(rd)
    }
}

// helper function: insert resources for a certain type
#[inline]
fn insert_material<T: Material>(
    rd: &mut ResourceDictionary,
    dep_rels: &HashMap<u32, Vec<Dependancy>>,
    variables: &mut HashMap<u32, BytecodeObject>,
    id: u32,
) -> Result<u32, LitError> {
    // loop through dependencies to get ids for these dependencies
    let dep_ids: Vec<Result<u32, LitError>> = if let Some(deps) = dep_rels.get(&id) {
        deps.iter()
            .map(|dep| match dep.kind {
                ImgMaterial => insert_material::<ImgMaterial>(rd, dep_rels, variables, dep.id),
            })
            .collect()
    } else {
        vec![]
    };

    if let Some(mat) = variables.remove(&id) {
        Ok(rd.add_mat(T::from_bytecode_object(mat, &dep_ids)?))
    } else {
        Err(LitError::VariableNotFound(id))
    }
}

#[inline]
fn insert_materials<T: Material>(
    rd: &mut ResourceDictionary,
    dep_rels: &HashMap<u32, Vec<Dependancy>>,
    variables: &mut HashMap<u32, BytecodeObject>,
    ids: Vec<u32>,
) -> Result<(), LitError> {
    for id in ids {
        insert_material::<T>(rd, dep_rels, variables, id)?;
    }

    Ok(())
}
```

Of course, I had to add some extras to `Material` and `ImgTexture`.

*Modifications to src/resource/mod.rs*

```rust
/// An object that can be used to build resources.
pub trait Material: Sized {
    /* ... */
 
    fn from_bytecode_object(
        bobj: BytecodeObject,
        dependants: &[Result<u32, LitError>],
    ) -> Result<Self, LitError>;
}
```

*Modifications to src/gl_utils/texture/material.rs*

```rust
impl Material for ImgMaterial {
    /* ... */
 
    #[inline]
    fn from_bytecode_object(bobj: BytecodeObject, _d: &[Result<u32, LitError>]) -> Result<Self, LitError> {
        if let BytecodeObject::ImgMaterial(img) = bobj {
            Ok(img)
        } else {
            Err(LitError::IncorrectDataType(DataType::Unknown, DataType::ImgMaterial))
        }
    }
}
```

Finally, we have to ensure that everything is added properly to the state:

*Modifications to src/script/eval.rs*

```rust
state.img_material_ids.push(id);
```

Finally, when we run the program, we can see:

```
GameData { name: "Lost in Time", resource_dict: Some(ResourceDictionary { next_id: 1, loaded_ids: [], prev_loaded_ids: [], mat_img: {0: ImgMaterial { width: 5, height: 5, bg_color: Color { r: 255, g: 0, b: 1, is_transparent: false }, draws: [Pixel { x: 1, y: 1, color: Color { r: 0, g: 0, b: 1, is_transparent: false } }, Rectangle { x: 2, y: 2, w: 4, h: 4, color: Color { r: 0, g: 255, b: 1, is_transparent: false } }], buffer: None }}, res_img: {} }) }
```

Now, we just need a way to display it. This feels like it would fall under the renderer, so let's put it there.

*In src/renderer.rs*

```rust
use crate::{Game, ImgTexture, LitError};
use nalgebra::base::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel;

pub trait Renderer {
    fn main_loop(&self, game: &Game) -> Result<(), LitError>;
    fn draw_sprite(
        &mut self,
        position: Vector2<Pixel>,
        size: Vector2<Pixel>,
        rotation: f32,
    ) -> Result<(), LitError>;
}
```

Unforunately I spent too much time trying to figure out how `nalgebra` worked and I couldn't finish it.
