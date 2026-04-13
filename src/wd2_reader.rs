use std::io;
use std::io::Read;
use std::io::{Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};


/* ===========================================================
 *                          HEADER
 * ===========================================================
 *   This assumes that 'Multi Board Single File' is enabled
 *
 *   event_number     
 *   timestamp        
 *   samples          
 *   sampling_period
 *   channels^
 *
 *
 *   ^ channels only exists if there are more than one channels (WD2 "One file per Channel" setting
 *   is disabled)
*/

#[derive(Debug)]
pub struct HeaderWD2 {
    pub event_counter   : u32,
    pub timestamp       : u64,
    pub samples         : u32,
    pub sampling_period : u64,
    pub channels        : i32,
}

impl HeaderWD2 {
    /*
    fn validate_header<R: Read>(reader: &mut R) -> bool {
        // read the initial header in here including a channel
        let evt_cnt         = reader.read_u32::<LittleEndian>()?;
        let timestamp       = reader.read_u64::<LittleEndian>()?;
        let samples         = reader.read_u32::<LittleEndian>()?;
        let sampling_period = reader.read_u32::<LittleEndian>()?;
        let channels        = reader.read_u32::<LittleEndian>()?;

        // read a full dataset in based on the channels
        let mut buf = vec![0u8; 4*samples as usize * channels as usize];
        reader.read_exact(&mut buf)?;
        
        let evt_cnt_2         = reader.read_u32::<LittleEndian>()?;
        let timestamp_2       = reader.read_u64::<LittleEndian>()?;
        let samples_2         = reader.read_u32::<LittleEndian>()?;
        let sampling_period_2 = reader.read_u32::<LittleEndian>()?;
        let channels_2        = reader.read_u32::<LittleEndian>()?;

        if evt_cnt_2 != (evt_cnt + 1) 
            && samples == samples_2
                && sampling_period == sampling_period_2 {
            // true -> multi-channel 
            return channels 
        }
        else {
            // false -> single-channel
            return 1 
        }
    }
    */

    pub fn from_reader<R: Read>(reader: &mut R, channels: i32) -> io::Result<Self> {
        // validate the header ONCE, then never again when called
        let event_counter    = reader.read_u32::<LittleEndian>()?;
        let timestamp        = reader.read_u64::<LittleEndian>()?;
        let samples          = reader.read_u32::<LittleEndian>()?;
        let sampling_period  = reader.read_u64::<LittleEndian>()?;

        let channels = if channels > 1 {
            reader.read_i32::<LittleEndian>()?
        }
        else {
            channels
        };
        Ok( Self {
            event_counter,
            timestamp,
            samples,
            sampling_period,
            channels,
            }
        )
    }
}



/* ===========================================================
 *                         EVENT 
 * ===========================================================
*/

#[derive(Debug)]
pub struct EventWD2 {
    pub header : HeaderWD2,
    pub data   : Vec<f32>
}

impl EventWD2 {
    pub fn from_reader<R: Read>(reader: &mut R, channels: i32) -> io::Result<Self> {
        let header   = HeaderWD2::from_reader(reader, channels)?;
        let n        = header.samples as usize * channels as usize;
        
        // read all data into a buffer
        let mut buf = vec![0u8; n * 4]; // 32 bytes --> 8 * 4
        reader.read_exact(&mut buf)?;
        
        let data: Vec<f32> = buf
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        
        Ok(Self {header, data})
    }
}

/* ===========================================================
 *                        WD2Reader 
 * ===========================================================
*/



pub struct WD2Reader<R: Read> {
    reader   : R,
    channels : i32,
}



impl<R: Read + Seek> WD2Reader<R> {
    pub fn new(mut reader: R) -> io::Result<Self> {
        let channels = Self::detect_channels(&mut reader)?;
        // reset reader
        reader.seek(SeekFrom::Start(0))?;
        Ok(Self {reader, channels})
    }

    fn detect_channels(reader: &mut R) -> io::Result<i32> {
        // read the initial header in here including a channel
        let event_counter   = reader.read_u32::<LittleEndian>()?;
        let _timestamp      = reader.read_u64::<LittleEndian>()?;
        let samples         = reader.read_u32::<LittleEndian>()?;
        let sampling_period = reader.read_u64::<LittleEndian>()?;
        let channels        = reader.read_i32::<LittleEndian>()?;

        // read a full dataset in based on the channels
        let mut buf = vec![0u8; 4*samples as usize * channels as usize];
        reader.read_exact(&mut buf)?;
        
        let event_counter_2   = reader.read_u32::<LittleEndian>()?;
        let _timestamp_2      = reader.read_u64::<LittleEndian>()?;
        let samples_2         = reader.read_u32::<LittleEndian>()?;
        let sampling_period_2 = reader.read_u64::<LittleEndian>()?;
        let channels_2        = reader.read_i32::<LittleEndian>()?;
        
        if event_counter_2 == event_counter + 1
            && samples == samples_2
                && sampling_period == sampling_period_2 
                    && channels_2 == channels {
            Ok(channels)
        }
        else {
            Ok(1)
        }

    }

}


impl<R: Read> Iterator for WD2Reader<R> {
    type Item = io::Result<EventWD2>;

    fn next(&mut self) -> Option<Self::Item> {
        match EventWD2::from_reader(&mut self.reader, self.channels) {
            Ok(event)                                          => Some(Ok(event)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e)                                             => Some(Err(e)),

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
           event_counter   : u32, 
           timestamp       : u64, 
           samples         : u32,
           sampling_period : u64, 
           channels        : i32,) {

           // generate vector with relevant information
           let mut data = Vec::new();
           data.extend_from_slice(&event_counter   .to_le_bytes());
           data.extend_from_slice(&timestamp       .to_le_bytes());
           data.extend_from_slice(&samples         .to_le_bytes());
           data.extend_from_slice(&sampling_period .to_le_bytes());
           if channels > 1 {
           data.extend_from_slice(&channels        .to_le_bytes());
           }

           // generate Cursor to hold data
           let mut cursor = Cursor::new(data);
           // then read out into header
           let header     = HeaderWD2::from_reader(&mut cursor, channels).unwrap();

            prop_assert_eq!(header.event_counter   , event_counter   ); 
            prop_assert_eq!(header.timestamp       , timestamp       ); 
            prop_assert_eq!(header.samples         , samples         ); 
            prop_assert_eq!(header.sampling_period , sampling_period ); 
            prop_assert_eq!(header.channels        , channels        ); 
        }

    }

    proptest! {
        #[test]
        fn test_event_reads_correctly(
            event_counter   : u32, 
            timestamp       : u64, 
            samples         in 10u32..1000u32, // applying limits here to avoid overflows
            sampling_period : u64, 
            channels        in 0i32..10i32,      // as above
            mut vals        in proptest::collection::vec(any::<f32>(), 1..10000usize) // defined to
                                                                                     // hold vals
            ) {
        
            // reshape vals to match desired size
            let n        = (samples * channels as u32) as usize;
            vals.resize(n, 0.0);

            // generate data vector
            let mut data = Vec::new();
            data.extend_from_slice(&event_counter   .to_le_bytes());
            data.extend_from_slice(&timestamp       .to_le_bytes());
            data.extend_from_slice(&samples         .to_le_bytes());
            data.extend_from_slice(&sampling_period .to_le_bytes());
            if channels > 1 {
                data.extend_from_slice(&channels    .to_le_bytes());
            }
            for s in &vals {
                data.extend_from_slice(&s.to_le_bytes());
            }
            // store everything
            let mut cursor = Cursor::new(data);

            // generate event
            let event      = EventWD2::from_reader(&mut cursor, channels).unwrap();

            prop_assert_eq!(event.data.len(), vals.len());
            prop_assert_eq!(event.data, vals);
        }
    }
   }
