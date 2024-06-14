#[macro_export]
macro_rules! atomic_macro {
    (
        pub struct $atomic:ident {
            $($atomic_var:ident: $atomic_ty:ty),*
        }

        $(
            $av:ident
        ),*

        $(
            @$av2:ident
        ),*

    ) => {
        paste::paste! {
            pub struct [<$atomic Inner>] {
                $($atomic_var: $atomic_ty),*
            }

            pub type $atomic = crate::atomic::Atomic<[<$atomic Inner>]>;

            impl $atomic {
                $(
                    pub fn [<get_ $av>](&self) -> std::sync::MutexGuard<[<$atomic Inner>]> {
                        self.0.lock().unwrap()
                    }
                )*
                $(
                    pub fn [<take_ $av2>](&self) -> [<$atomic Inner>]{
                        self.0.lock().unwrap().clone()
                    }
                )*
            }
        }
    };
}