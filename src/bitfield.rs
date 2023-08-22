#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitfieldUnit<Storage> {
    storage: Storage,
}

impl<Storage> BitfieldUnit<Storage> {
    #[inline]

    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl<Storage> BitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;

        let byte = self.storage.as_ref()[byte_index];

        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;

        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);

        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());

        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };

                val |= 1 << index;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);

        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());

        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;

            let val_bit_is_set = val & mask == mask;

            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };

            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct ArrayField<T>(std::marker::PhantomData<T>, [T; 0]);

impl<T> ArrayField<T> {
    #[inline]

    pub const fn new() -> Self {
        Self(std::marker::PhantomData, [])
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]

    pub const unsafe fn as_slice(&self, len: usize) -> &[T] {
        std::slice::from_raw_parts(self.as_ptr(), len)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]

    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> std::fmt::Debug for ArrayField<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("ArrayField")
    }
}

#[repr(C)]
pub struct UnionField<T>(std::marker::PhantomData<T>);

impl<T> UnionField<T> {
    #[inline]

    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]

    pub unsafe fn as_ref(&self) -> &T {
        std::mem::transmute(self)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]

    pub unsafe fn as_mut(&mut self) -> &mut T {
        std::mem::transmute(self)
    }
}

impl<T> std::default::Default for UnionField<T> {
    #[inline]

    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::clone::Clone for UnionField<T> {
    #[inline]
    #[allow(clippy::incorrect_clone_impl_on_copy_type)]

    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> std::marker::Copy for UnionField<T> {}

impl<T> std::fmt::Debug for UnionField<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("UnionField")
    }
}

impl<T> std::hash::Hash for UnionField<T> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> std::cmp::PartialEq for UnionField<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> std::cmp::Eq for UnionField<T> {}
