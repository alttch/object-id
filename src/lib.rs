#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "README.md" ) ) ]
#[derive(Default, Clone)]
pub struct UniqueId {
    inner: Box<u8>,
}

impl core::fmt::Debug for UniqueId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}
impl core::hash::Hash for UniqueId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_usize().hash(state);
    }
}

impl PartialEq for UniqueId {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.inner, &*other.inner)
    }
}

impl PartialOrd for UniqueId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_usize().cmp(&other.as_usize()))
    }
}

impl Ord for UniqueId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_usize().cmp(&other.as_usize())
    }
}

impl Eq for UniqueId {}

impl UniqueId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_usize(&self) -> usize {
        &*self.inner as *const u8 as usize
    }
}

#[cfg(test)]
mod test {
    use crate::UniqueId;

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct Test {
        id: UniqueId,
    }
    #[test]
    fn test_stack() {
        fn generate() -> (Test, usize) {
            let t = Test { id: <_>::default() };
            let n = t.id.as_usize();
            (t, n)
        }
        let (t, n) = generate();
        assert_eq!(t.id.as_usize(), n);
    }
    #[test]
    fn test_clone_eq() {
        let t = Test { id: <_>::default() };
        let t2 = t.clone();
        assert_ne!(t.id, t2.id);
        assert_ne!(t.id.as_usize(), t2.id.as_usize());
        assert_ne!(t, t2);
        assert_eq!(t, t);
        assert_eq!(t.id, t.id);
    }
    #[test]
    fn test_heap_movement() {
        let t = Test { id: <_>::default() };
        let n = t.id.as_usize();
        let mut x = vec![t];
        assert_eq!(x[0].id.as_usize(), n);
        let t_back = x.pop().unwrap();
        assert_eq!(t_back.id.as_usize(), n);
    }
}