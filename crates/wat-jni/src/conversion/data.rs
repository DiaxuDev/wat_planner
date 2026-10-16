use jni::{
    Env, jni_sig, jni_str, objects::JObject, signature::RuntimeFieldSignature, strings::JNIString,
};
use wat_parser::{
    Class,
    ClassKind::{self, Unknown},
    Data, Professor, Subject,
};

use super::{AsJValue, IntoJava};

impl<'local> IntoJava<'local> for Data {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("dev.diaxu.watplanner.parser.Data"))?;

        let year_one = self.year.0;
        let year_two = self.year.1;
        let legend = self.legend.into_java(env)?;
        let classes = self.classes.into_java(env)?;

        env.new_object(
            class,
            jni_sig!(sig = (year_one: jint, year_two: jint, legend: "java.util.HashMap", classes: "java.util.HashMap")),
            &[
                year_one.as_jvalue(),
                year_two.as_jvalue(),
                legend.as_jvalue(),
                classes.as_jvalue(),
            ],
        )
    }
}

impl<'local> IntoJava<'local> for Subject {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("dev.diaxu.watplanner.parser.Subject"))?;

        let name = self.name.into_java(env)?;
        let color = self.color;
        let professors = self.professors.into_java(env)?;

        env.new_object(
            class,
            jni_sig!(sig = (name: JString, color: jlong, professors: "java.util.HashMap")),
            &[name.as_jvalue(), color.as_jvalue(), professors.as_jvalue()],
        )
    }
}

impl<'local> IntoJava<'local> for ClassKind {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let name = match self {
            ClassKind::None => "None",
            ClassKind::Lecture => "Lecture",
            ClassKind::Practice => "Practice",
            ClassKind::Lab => "Lab",
            ClassKind::Seminar => "Seminar",
            ClassKind::Project => "Project",
            ClassKind::Exam => "Exam",
            ClassKind::MakeUpExam => "MakeUpExam",
            ClassKind::Pass => "Pass",
            ClassKind::MakeUpPass => "MakeUpPass",
            ClassKind::Retake => "Retake",
            Unknown(_) => "Unknown",
        };

        let class = env.load_class(JNIString::from(format!(
            "dev.diaxu.watplanner.parser.ClassKind${name}"
        )))?;

        if let Unknown(inner) = self {
            let field_sig = jni_sig!(sig = (JString));
            let value = inner.into_java(env)?;
            env.new_object(class, field_sig, &[value.as_jvalue()])
        } else {
            let field_sig = RuntimeFieldSignature::from_str(format!(
                "Ldev/diaxu/watplanner/parser/ClassKind${name};"
            ))?;
            env.get_static_field(class, jni_str!("INSTANCE"), field_sig.field_signature())?
                .l()
        }
    }
}

impl<'local> IntoJava<'local> for Professor {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("dev.diaxu.watplanner.parser.Professor"))?;

        let name = self.name.into_java(env)?;
        let hours = self.hours;

        env.new_object(
            class,
            jni_sig!(sig = (name: JString, hours: jshort)),
            &[name.as_jvalue(), hours.as_jvalue()],
        )
    }
}

impl<'local> IntoJava<'local> for Class {
    type JavaType = JObject<'local>;

    fn into_java(self, env: &mut Env<'local>) -> jni::errors::Result<Self::JavaType> {
        let class = env.load_class(jni_str!("dev.diaxu.watplanner.parser.Class"))?;

        let code = self.code.into_java(env)?;
        let kind = self.kind.into_java(env)?;
        let info = self.info.into_java(env)?;

        env.new_object(class, jni_sig!(sig = (code: JString, kind: "dev.diaxu.watplanner.parser.ClassKind", info: "java.util.ArrayList")), &[code.as_jvalue(), kind.as_jvalue(), info.as_jvalue()])
    }
}
