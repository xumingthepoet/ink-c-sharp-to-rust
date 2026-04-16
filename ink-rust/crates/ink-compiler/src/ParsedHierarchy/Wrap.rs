// Source: ink-c-sharp/compiler/ParsedHierarchy/Wrap.cs

use ink_runtime::Glue::Glue as RuntimeGlue;
use ink_runtime::Tag::Tag as RuntimeTag;

#[derive(Clone, Debug)]
pub struct Wrap<T> {
    objToWrap: T,
}

#[derive(Clone, Debug)]
pub struct Glue {
    inner: Wrap<RuntimeGlue>,
}

#[derive(Clone, Debug)]
pub struct LegacyTag {
    inner: Wrap<RuntimeTag>,
}

impl<T> Wrap<T> {
    // C# signature: public Wrap (T objToWrap)
    pub fn new(objToWrap: T) -> Self {
        Self { objToWrap }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> T
    where
        T: Clone,
    {
        self.objToWrap.clone()
    }
}

impl Glue {
    pub fn new(glue: RuntimeGlue) -> Self {
        Self {
            inner: Wrap::new(glue),
        }
    }

    pub fn GenerateRuntimeObject(&self) -> RuntimeGlue {
        self.inner.GenerateRuntimeObject()
    }
}

impl LegacyTag {
    pub fn new(tag: RuntimeTag) -> Self {
        Self {
            inner: Wrap::new(tag),
        }
    }

    pub fn GenerateRuntimeObject(&self) -> RuntimeTag {
        self.inner.GenerateRuntimeObject()
    }
}

#[cfg(test)]
mod tests {
    use super::{Glue, LegacyTag};
    use ink_runtime::Glue::Glue as RuntimeGlue;
    use ink_runtime::Tag::Tag as RuntimeTag;

    #[test]
    fn wrap_returns_wrapped_runtime_object() {
        assert_eq!(
            Glue::new(RuntimeGlue::new())
                .GenerateRuntimeObject()
                .ToString(),
            "Glue"
        );
        assert_eq!(
            LegacyTag::new(RuntimeTag::new("x".to_string()))
                .GenerateRuntimeObject()
                .get_text(),
            "x"
        );
    }
}
