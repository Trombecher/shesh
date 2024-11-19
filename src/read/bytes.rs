use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

pub type Index = u32;

pub struct Span<T> {
    pub value: T,
    pub range: Range<Index>,
}

impl<T: PartialEq> PartialEq for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.range == other.range
    }
}

impl<T: Clone> Clone for Span<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            range: self.range.clone(),
        }
    }
}

impl<T: Debug> Debug for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("value", &self.value)
            .field("range", &self.range)
            .finish()
    }
}

pub struct Cursor<'a> {
    start: *const u8,
    next: *const u8,
    end: *const u8,
    _marker: PhantomData<&'a str>,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(slice: &'a str) -> Self {
        let start = slice.as_ptr();
        
        Self {
            start,
            next: start,
            end: unsafe { start.add(slice.len()) },
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.next == self.end {
            None
        } else {
            Some(unsafe { *self.next })
        }
    }

    #[inline]
    pub fn advance(&mut self) {
        self.next = unsafe { self.next.add( (self.next != self.end) as usize) };
    }

    #[inline]
    pub fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }
    
    #[inline]
    pub fn index(&self) -> Index {
        (self.next as usize - self.start as usize) as Index
    }
    
    #[inline]
    pub fn pointer(&self) -> *const u8 {
        self.next
    }
}