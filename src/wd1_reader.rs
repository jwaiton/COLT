use std::io;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

/* ===========================================================
 *                          HEADER
 * ===========================================================
*/

#[derive(Debug)]
pub struct Header {
    pub event_size    : i32,
    pub board_id      : i32,
    pub pattern       : i32,
    pub board_channel : i32,
    pub event_counter : i32,
    pub timestamp     : i32,
}

impl Header {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            event_size    : reader.read_i32::<LittleEndian>()?,
            board_id      : reader.read_i32::<LittleEndian>()?,
            pattern       : reader.read_i32::<LittleEndian>()?,
            board_channel : reader.read_i32::<LittleEndian>()?,
            event_counter : reader.read_i32::<LittleEndian>()?,
            timestamp     : reader.read_i32::<LittleEndian>()?,

        })
    }
}

/* ===========================================================
 *                         EVENT 
 * ===========================================================
*/


#[derive(Debug)]
pub struct Event {
    pub header : Header,
    pub data   : Vec<u16>
}

impl Event {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        // collect header, extract n_samples, create vector
        let header      = Header::from_reader(reader)?;
        // remove header 
        let n           = ((header.event_size - 24) / 2) as usize;

        // read all data into buffer
        let mut buf = vec![0u8; n * 2];
        reader.read_exact(&mut buf)?;
        
        // separate into u16 sied chunks, map and collect
        let samples: Vec<u16> = buf
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();

        Ok(Self {
            header: header,
            data: samples
        })
    }
}

/* ===========================================================
 *                        EVENT READER 
 * ===========================================================
*/

pub struct WD1Reader<R: Read> {
    reader  : R,
    errored : bool, // check if EOF or actually broken
}

impl<R: Read> WD1Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {reader, errored: false}
    }
}

impl<R: Read> Iterator for WD1Reader<R> {
   type Item = io::Result<Event>; 

   fn next(&mut self) -> Option<Self::Item> {
       if self.errored { return None; }
        match Event::from_reader(&mut self.reader) {
            Ok(event)  => Some(Ok(event)),
            Err(e)     => match e.kind() { // EOF --> return None, anything else return error
                                io::ErrorKind::UnexpectedEof => None,
                                _                            => {self.errored = true; Some(Err(e))}
            }
       }
   }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_header_reads_correctly(
            evt_size : i32, brd_id  : i32, ptrn   : i32,
            brd_ch   : i32, evt_cnt : i32, tmstmp : i32,
            ) {
            
            // generate vector with relevant information
            let mut data = Vec::new();
            data.extend_from_slice(&evt_size .to_le_bytes());
            data.extend_from_slice(&brd_id   .to_le_bytes());
            data.extend_from_slice(&ptrn     .to_le_bytes());
            data.extend_from_slice(&brd_ch   .to_le_bytes());
            data.extend_from_slice(&evt_cnt  .to_le_bytes());
            data.extend_from_slice(&tmstmp   .to_le_bytes());

            // generate Cursor to hold data
            let mut cursor = Cursor::new(data);
            // then read out into header
            let header     = Header::from_reader(&mut cursor).unwrap();

            // check
            prop_assert_eq!(header.event_size   , evt_size);
            prop_assert_eq!(header.board_id     , brd_id  );
            prop_assert_eq!(header.pattern      , ptrn    );
            prop_assert_eq!(header.board_channel, brd_ch  );
            prop_assert_eq!(header.event_counter, evt_cnt );
            prop_assert_eq!(header.timestamp    , tmstmp  );
        }
    }

    proptest! {
        #[test]
        fn test_event_reads_correctly(
            // event size must fit expected shape
            // 2 bigger than the header
            brd_id  : i32, ptrn   : i32,
            brd_ch   : i32, evt_cnt : i32, 
            tmstmp : i32,
            // ensure samples matches event size
            samples: Vec<u16>,
            ) {
            
            // collect event size explicitly from samples
            let evt_size = ((samples.len() * 2) + 24) as i32;
            // generate the data for comparison
            let mut data = Vec::new();
            data.extend_from_slice(&evt_size.to_le_bytes());
            data.extend_from_slice(&brd_id  .to_le_bytes());
            data.extend_from_slice(&ptrn    .to_le_bytes());
            data.extend_from_slice(&brd_ch  .to_le_bytes());
            data.extend_from_slice(&evt_cnt .to_le_bytes());
            data.extend_from_slice(&tmstmp  .to_le_bytes());
            for s in &samples {
                data.extend_from_slice(&s.to_le_bytes());
            }
            let mut cursor = Cursor::new(data);

            let event = Event::from_reader(&mut cursor).unwrap();

            prop_assert_eq!(event.data.len(), samples.len());
            prop_assert_eq!(event.data, samples);

        }


    }
    proptest! {
        #[test]
        fn test_wd1reader(
             // event size must fit expected shape
            // 2 bigger than the header
            brd_id  : i32, ptrn   : i32,
            brd_ch   : i32, evt_cnt : i32, 
            tmstmp : i32,
            // ensure samples matches event size
            samples: Vec<u16>,
            ) {
             // collect event size explicitly from samples
            let evt_size = ((samples.len() * 2) + 24) as i32;
            // generate the data for comparison
            let mut data = Vec::new();
            data.extend_from_slice(&evt_size.to_le_bytes());
            data.extend_from_slice(&brd_id  .to_le_bytes());
            data.extend_from_slice(&ptrn    .to_le_bytes());
            data.extend_from_slice(&brd_ch  .to_le_bytes());
            data.extend_from_slice(&evt_cnt .to_le_bytes());
            data.extend_from_slice(&tmstmp  .to_le_bytes());
            for s in &samples {
                data.extend_from_slice(&s.to_le_bytes());
            }
            let cursor = Cursor::new(data);

            let mut wd1reader = WD1Reader::new(cursor);
            let event         = wd1reader.next().unwrap()?;

            prop_assert_eq!(event.data.len(), samples.len());
            prop_assert_eq!(event.data, samples);


        }
    }

}
