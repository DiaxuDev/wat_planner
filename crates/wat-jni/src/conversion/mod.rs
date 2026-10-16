// Inspired by https://github.com/mullvad/jnix

use jni::{
    Env, JValue,
    objects::{JList, JMap, JObject, JString},
};

mod data;
mod external;
mod std;

pub(crate) trait IntoJava<'local> {
    type JavaType: Into<JObject<'local>>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType>;
}

pub(crate) trait AsJValue {
    fn as_jvalue(&self) -> JValue<'_>;
}

impl AsJValue for JObject<'_> {
    fn as_jvalue(&self) -> JValue<'_> {
        JValue::Object(self)
    }
}

macro_rules! as_jvalue_for {
    ($($t:ty),+) => {
        $(
            impl AsJValue for $t {
                fn as_jvalue(&self) -> JValue<'_> {
                    JValue::Object(self.as_ref())
                }
            }
        )+
    };
}

as_jvalue_for!(JList<'_>, JMap<'_>, JString<'_>);
