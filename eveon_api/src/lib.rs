use mlua::prelude::*;

mod eveon;
mod eveon_ui;

pub trait Component {
    fn when_start(&self) {}
    fn when_update(&self, dt: u32) {}
}

impl FromLua<'lua> for (dyn Component + 'static) {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        todo!()
    }
}

#[mlua::lua_module]
fn eveon(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    exports
        .set(
            "get_component",
            lua.create_function(eveon::get_component).unwrap(),
        )
        .unwrap();
    Ok(exports)
}
