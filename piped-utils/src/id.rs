#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(std::num::NonZeroU64);

        impl $name {
            pub fn new(id: u64) -> Self {
                Self(std::num::NonZeroU64::new(id).unwrap())
            }

            pub fn get(self) -> u64 {
                self.0.get()
            }
        }

        impl From<$name> for u64 {
            fn from(id: $name) -> Self {
                id.get()
            }
        }

        impl From<u64> for $name {
            fn from(id: u64) -> Self {
                Self::new(id)
            }
        }

        impl From<$name> for usize {
            fn from(id: $name) -> Self {
                id.get() as usize
            }
        }

        impl From<usize> for $name {
            fn from(id: usize) -> Self {
                Self::new(id as u64)
            }
        }

        impl From<$name> for std::num::NonZeroU64 {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl From<std::num::NonZeroU64> for $name {
            fn from(id: std::num::NonZeroU64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for std::num::NonZeroUsize {
            fn from(id: $name) -> Self {
                std::num::NonZeroUsize::new(id.get() as usize).unwrap()
            }
        }

        impl From<std::num::NonZeroUsize> for $name {
            fn from(id: std::num::NonZeroUsize) -> Self {
                Self::new(id.get() as u64)
            }
        }

        impl piped_utils::NodeIdCtor for $name {
            fn new(id: u64) -> Self {
                Self::new(id)
            }
        }
    };
}

pub trait NodeIdCtor {
    fn new(id: u64) -> Self;
}

pub struct NodeIdAllocator<T>
where
    T: NodeIdCtor,
{
    id: u64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NodeIdAllocator<T>
where
    T: NodeIdCtor,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn next(&mut self) -> T {
        self.id += 1;
        T::new(self.id)
    }
}

impl<T> Default for NodeIdAllocator<T>
where
    T: NodeIdCtor,
{
    fn default() -> Self {
        Self {
            id: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for NodeIdAllocator<T>
where
    T: NodeIdCtor,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: std::marker::PhantomData,
        }
    }
}
