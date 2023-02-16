// use tarantool::{tlua::LuaFunction, tuple::Tuple, uuid::Uuid};

// const UUID_STR: &str = "30de7784-33e2-4393-a8cd-b67534db2432";

use tarantool::{datetime::Datetime, tuple::Tuple, tlua::LuaFunction};

pub fn to_tuple() {
    let dt: Datetime = time::OffsetDateTime::now_utc().into();
    let t = Tuple::new(&[dt]).unwrap();
    dbg!(t.field(0).unwrap().unwrap())
    // let lua = tarantool::lua_state();
    // let f: LuaFunction<_> = lua.eval("return box.tuple.unpack").unwrap();
    // let u: Datetime = f.call_with_args(&t).unwrap();
    // assert_eq!(u.to_string(), UUID_STR);
}

// pub fn from_tuple() {
//     assert!(false);
//     let t: Tuple = tarantool::lua_state()
//         .eval(&format!(
//             "return box.tuple.new(require('uuid').fromstr('{}'))",
//             UUID_STR
//         ))
//         .unwrap();
//     let (u,): (Uuid,) = t.decode().unwrap();
//     assert_eq!(u.to_string(), UUID_STR);
// }

// pub fn to_lua() {
//     assert!(false);
//     let uuid: Uuid = tarantool::lua_state()
//         .eval(&format!("return require('uuid').fromstr('{}')", UUID_STR))
//         .unwrap();
//     assert_eq!(uuid.to_string(), UUID_STR);
// }

// pub fn from_lua() {
//     assert!(false);
//     let uuid = Uuid::parse_str(UUID_STR).unwrap();
//     let lua = tarantool::lua_state();
//     let tostring: LuaFunction<_> = lua.eval("return tostring").unwrap();
//     let s: String = tostring.call_with_args(uuid).unwrap();
//     assert_eq!(s, UUID_STR);
// }
