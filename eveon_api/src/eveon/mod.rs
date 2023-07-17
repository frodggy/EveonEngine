use mlua::prelude::*;

use crate::Component;

pub fn get_component<T>(lua: &Lua, component_name: String) -> LuaResult<impl Component>
where
    T: Component,
{
    let x = lua.globals().get::<_, T>(component_name).unwrap();

    return Ok(x);
}
