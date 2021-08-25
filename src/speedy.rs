// Copyright 2021 CoD Technologies Corp.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! speedy implementation.

use crate::Decimal;
use speedy::{Writer, Reader};

#[cfg_attr(docsrs, doc(cfg(feature = "speedy")))]
impl<C: speedy::Context> speedy::Writable<C> for Decimal {
    #[inline]
    fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
        writer.write_bytes(&self.int_val.to_le_bytes())?;
        self.scale.write_to(writer)?;
        self.negative.write_to(writer)
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Ok(19)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "speedy")))]
impl<'a, C: speedy::Context> speedy::Readable<'a, C> for Decimal {
    #[inline]
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let mut bs = [0u8; 16];
        reader.read_bytes(&mut bs)?;
        let int_val = u128::from_le_bytes(bs);
        let scale = i16::read_from(reader)?;
        let negative = bool::read_from(reader)?;
        Ok(unsafe { Decimal::from_parts_unchecked(int_val, scale, negative) })
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        19
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use speedy::{Writable, Readable};

    #[test]
    fn test_speedy() {
        let mut buf = [0u8; 256];
        let dec = "123.456".parse::<Decimal>().unwrap();

        dec.write_to_buffer(&mut buf).unwrap();
        let dec2 = Decimal::read_from_buffer(&buf).unwrap();
        assert_eq!(dec, dec2);
    }
}
