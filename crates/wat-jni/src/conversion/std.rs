use std::collections::HashMap;

use jni::{
    JValue, jni_sig, jni_str,
    objects::{JList, JMap, JObject, JObjectArray, JString},
    sys::{jint, jlong, jshort},
};

use crate::conversion::AsJValue;

use super::IntoJava;

impl<'local> IntoJava<'local> for u8 {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("java.lang.Short"))?;
        env.new_object(class, jni_sig!(sig = (value: jint)), &[self.as_jvalue()])
    }
}

impl AsJValue for u8 {
    fn as_jvalue(&self) -> JValue<'_> {
        JValue::Short(*self as jshort)
    }
}

impl<'local> IntoJava<'local> for u16 {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("java.lang.Integer"))?;
        env.new_object(class, jni_sig!(sig = (value: jint)), &[self.as_jvalue()])
    }
}

impl AsJValue for u16 {
    fn as_jvalue(&self) -> JValue<'_> {
        JValue::Int(*self as jint)
    }
}

impl<'local> IntoJava<'local> for u32 {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("java.lang.Long"))?;
        env.new_object(class, jni_sig!(sig = (value: jlong)), &[self.as_jvalue()])
    }
}

impl AsJValue for u32 {
    fn as_jvalue(&self) -> JValue<'_> {
        JValue::Long(*self as jlong)
    }
}

impl AsJValue for i32 {
    fn as_jvalue(&self) -> JValue<'_> {
        JValue::Int(*self)
    }
}

impl<'local> IntoJava<'local> for String {
    type JavaType = JString<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        env.new_string(self)
    }
}

impl<'local, T> IntoJava<'local> for Option<T>
where
    T: IntoJava<'local>,
{
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        match self {
            Some(t) => t.into_java(env).map(Into::into),
            None => Ok(JObject::null()),
        }
    }
}

impl<'local, T> IntoJava<'local> for Vec<T>
where
    T: IntoJava<'local>,
{
    type JavaType = JList<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let cap = JValue::Int(self.len() as jint);

        let class = env.load_class(jni_str!("java.util.ArrayList"))?;
        let object = env.new_object(class, jni_sig!(sig = (cap: jint)), &[cap])?;
        let list = JList::cast_local(env, object)?;

        for elem in self {
            let x = elem.into_java(env)?;
            list.add(env, &x.into())?;
        }

        Ok(list)
    }
}

// This technically also covers arrays of integers, which should be put into JIntArray and not the generic JObjectArray, but who cares
impl<'local, T> IntoJava<'local> for [T; 7]
where
    T: IntoJava<'local> + Default + PartialEq,
{
    type JavaType = JObjectArray<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let default = T::default().into_java(env)?;
        let arr = JObjectArray::<JObject>::new(env, self.len(), default.into())?;

        for (i, elem) in self.into_iter().enumerate() {
            if elem != T::default() {
                let val = elem.into_java(env)?;
                arr.set_element(env, i, val.into())?;
            }
        }

        Ok(arr)
    }
}

impl<'local, K, V> IntoJava<'local> for HashMap<K, V>
where
    K: IntoJava<'local>,
    V: IntoJava<'local>,
{
    type JavaType = JMap<'local>;

    fn into_java(self, env: &mut jni::Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let cap = JValue::Int(self.len() as jint);

        let class = env.load_class(jni_str!("java.util.HashMap"))?;
        let object = env.new_object(class, jni_sig!(sig = (cap: jint)), &[cap])?;
        let map = JMap::cast_local(env, object)?;

        for elem in self {
            let key = elem.0.into_java(env)?;
            let value = elem.1.into_java(env)?;
            map.put(env, &key.into(), &value.into())?;
        }

        Ok(map)
    }
}
