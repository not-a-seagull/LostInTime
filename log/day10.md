# Day 10

I'm not feeling well today, so let's just implement the image software on the Rust side and be done with it.

```rust
4 => {
    // create a new texture material
    let mut buffer = [0; 4];
    stream.read(&mut buffer)?;
    let id = u32::from_be_bytes(buffer);

    let width = BytecodeObject::read(stream)?;
    let width = width.as_number(state)?.try_into()?;

    let height = BytecodeObject::read(stream)?;
    let height = height.as_number(state)?.try_into()?;

    let bg_color = BytecodeObject::read(stream)?.as_color(state)?;

    let mat = ImgMaterial::new(width, height, bg_color);
    state.register_variable(id, BytecodeObject::ImgMaterial(mat));

    Ok(())
}
5 => {
    // assign a color id to an invocation
    let buf_id = BytecodeObject::read(stream)?;
    let buf_id = buf_id.get_var_id(state)?;

    let clr_id = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;

    let color = BytecodeObject::read(stream)?.as_color(state)?;

    state.register_color_id(buf_id, clr_id, color);
    Ok(())
}
6 => {
    // draw a single pixel
    let mut draw_buffer = BytecodeObject::read(stream)?;
    let draw_id = draw_buffer.get_var_id(state)?;

    let x = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
    let y = BytecodeObject::read(stream)?.as_number(state)?.try_into()?;
    let color = match state.get_color(
	draw_id,
	BytecodeObject::read(stream)?.as_number(state)?.try_into()?,
    ) {
	Ok(c) => *c,
	Err(_e) => BytecodeObject::read(stream)?.as_color(state)?,
    };

    let draw_handle = draw_buffer.as_draw_handle_mut(state)?;

    draw_handle.draw_pixel(x, y, color)
}
```

I also changed some types in `BytecodeObject` and encountered some issues with the borrow checker, but it's late and I have headache so I'll talk about it more tomorrow.
