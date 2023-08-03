#[macro_export]
macro_rules! impl_comp_as_ref {
    [$($t:ident),*] => {
        $(
            impl AsRef<$t> for Component {
                fn as_ref(&self) -> &$t {
                    if let Component::$t(inner) = self {
                        inner
                    } else {
                        panic!("Wrong cast, this component ({:?}) is not of correct type ({})", self, stringify!($t))
                    }
                }
            }
            impl AsMut<$t> for Component {
                fn as_mut(&mut self) -> &mut $t {
                    if let Component::$t(inner) = self {
                        inner
                    } else {
                        panic!("Wrong cast, this component ({:?}) is not of correct type ({})", self, stringify!($t))
                    }
                }
            }
        )*
    };
}