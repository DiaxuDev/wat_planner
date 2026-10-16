use chrono::Datelike;
use jni::{jni_sig, jni_str, objects::JObject, sys::jint};

use crate::conversion::{AsJValue, IntoJava};

impl<'local> IntoJava<'local> for chrono::NaiveDate {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("java.time.LocalDate"))?;

        let year = self.year();
        let day = self.ordinal() as jint;

        env.call_static_method(
            class,
            jni_str!("ofYearDay"),
            jni_sig!(sig = (year: jint, day: jint) -> "java.time.LocalDate"),
            &[year.as_jvalue(), day.as_jvalue()],
        )?
        .l()
    }
}
