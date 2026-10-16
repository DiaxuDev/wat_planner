use jni::{
    EnvUnowned,
    errors::ThrowRuntimeExAndDefault,
    jni_mangle,
    objects::{JClass, JObject, JString},
};

use crate::conversion::IntoJava;

mod conversion;

#[jni_mangle("dev.diaxu.watplanner.parser.WatParser")]
pub fn parse<'local>(mut env: EnvUnowned<'local>, _class: JClass, raw: JString) -> JObject<'local> {
    env.with_env(|env| match wat_parser::parse(&raw.to_string()) {
        Ok(data) => data.into_java(env),
        Err(why) => {
            env.throw(format!("parser error: {why}"))?;
            return Ok(JObject::null());
        }
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
