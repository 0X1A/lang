use std::fmt;
use std::iter::{Enumerate, Map};
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;
use std::str::{CharIndices, Chars};

use crate::error::LangError;

use nom::{
    error::ErrorKind, error::ParseError, AsBytes, Compare, CompareResult, Err, FindSubstring,
    IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Span<T> {
    pub input: T,
    pub offset: usize,
    pub line: u32,
    pub column: u32,
}

impl<T> fmt::Debug for Span<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span {{ offset: {}, line: {}, column: {} }}",
            self.offset, self.line, self.column
        )
    }
}

impl<T: fmt::Display> fmt::Display for Span<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span {{ offset: {}, line: {}, column: {} input: {} }}",
            self.offset, self.line, self.column, self.input
        )
    }
}

impl<T> Span<T> {
    pub fn new(input: T, offset: usize, line: u32, column: u32) -> Span<T> {
        Span {
            input,
            offset,
            line,
            column,
        }
    }
}

impl<T: InputLength> InputLength for Span<T> {
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

macro_rules! gen_input_iter_impl {
    ($input_type:ty, $element:ty, $iterator:ty, $iterator_element:ty) => {
        impl<'a> InputIter for Span<$input_type> {
            type Item = $element;
            type Iter = $iterator;
            type IterElem = $iterator_element;

            fn iter_indices(&self) -> Self::Iter {
                self.input.iter_indices()
            }

            fn iter_elements(&self) -> Self::IterElem {
                self.input.iter_elements()
            }

            fn position<P>(&self, predicate: P) -> Option<usize>
            where
                P: Fn(Self::Item) -> bool,
            {
                self.input.position(predicate)
            }

            fn slice_index(&self, count: usize) -> Option<usize> {
                self.input.slice_index(count)
            }
        }
    };
}

gen_input_iter_impl!(
    &'a [u8],
    u8,
    Enumerate<Self::IterElem>,
    Map<Iter<'a, Self::Item>, fn(&u8) -> u8>
);

gen_input_iter_impl!(&'a str, char, CharIndices<'a>, Chars<'a>);

impl<T: FindSubstring<T>> FindSubstring<T> for Span<T> {
    fn find_substring(&self, substr: T) -> Option<usize> {
        self.input.find_substring(substr)
    }
}

impl<T: Offset> Offset for Span<T> {
    fn offset(&self, second: &Self) -> usize {
        let first = self.offset;
        let second = second.offset;

        second - first
    }
}

impl<T: AsBytes> AsBytes for Span<T> {
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

macro_rules! gen_slice_impl {
    ($input_type:ty, $range_type:ty, $can_return_self:expr) => {
        impl<'a> Slice<$range_type> for Span<$input_type> {
            fn slice(&self, range_type: $range_type) -> Self {
                if $can_return_self(&range_type) {
                    return self.clone();
                }
                let next_input = self.input.slice(range_type);
                let offset = self.input.offset(&next_input);
                if offset == 0 {
                    return Span::new(next_input, self.offset, self.line, self.column);
                }

                let slice = self.input.slice(..offset);
                let next_offset = self.offset + offset;

                let bytes = slice.as_bytes();
                let number_of_lines = bytecount::count(bytes, b'\n') as u32;
                let next_line = self.line + number_of_lines;

                Span::new(next_input, next_offset, next_line, 0)
            }
        }
    };
}

gen_slice_impl!(&'a str, Range<usize>, |_| false);
gen_slice_impl!(&'a str, RangeTo<usize>, |_| false);
gen_slice_impl!(&'a str, RangeFrom<usize>, |range: &RangeFrom<usize>| range
    .start
    == 0);
gen_slice_impl!(&'a str, RangeFull, |_| true);

gen_slice_impl!(&'a [u8], Range<usize>, |_| false);
gen_slice_impl!(&'a [u8], RangeTo<usize>, |_| false);
gen_slice_impl!(&'a [u8], RangeFrom<usize>, |range: &RangeFrom<usize>| range
    .start
    == 0);
gen_slice_impl!(&'a [u8], RangeFull, |_| true);

impl<T> InputTake for Span<T>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

macro_rules! gen_compare_impl {
    ($input_type:ty, $cmp_to_type:ty) => {
        impl<'a, 'b> Compare<$cmp_to_type> for Span<$input_type> {
            #[inline]
            fn compare(&self, t: $cmp_to_type) -> CompareResult {
                self.input.compare(t)
            }

            fn compare_no_case(&self, t: $cmp_to_type) -> CompareResult {
                self.input.compare_no_case(t)
            }
        }
    };
}

gen_compare_impl!(&'b str, &'a str);
gen_compare_impl!(&'b [u8], &'a [u8]);

pub fn position<T: fmt::Display>(s: T) -> IResult<T, T, LangError>
where
    T: InputIter + InputTake,
{
    nom::bytes::complete::take(0usize)(s)
}

impl<T> InputTakeAtPosition for Span<T>
where
    T: InputTakeAtPosition + InputLength + InputIter,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>> + Clone,
{
    type Item = <T as InputIter>::Item;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::Size(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::Size(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            result => result,
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.input.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}
