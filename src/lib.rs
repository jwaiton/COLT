use pyo3::prelude::*;
use pyo3::types::PyDict;

use std::fs::File;
use std::io::BufReader;

use numpy::PyArray1;

use crate::wd1_reader::WD1Reader;
use crate::wd2_reader::WD2Reader;

pub mod wd1_reader;
pub mod wd2_reader;

// enum for which wavedump version
enum ReaderInner {
    WD1(wd1_reader::WD1Reader<BufReader<File>>),
    WD2(wd2_reader::WD2Reader<BufReader<File>>),
}


#[pyclass]
struct EventReader {
    inner: ReaderInner, 
}


#[pymethods]
impl EventReader {
    #[new]
    fn new(path: &str, reader_type: &str) -> PyResult<Self> {
        let file   = File::open(path)?;
        let reader = BufReader::new(file);
        // ugly inner matcher with error handling
        let inner  = match reader_type {
            "WD1" => ReaderInner::WD1(WD1Reader::new(reader)),
            "WD2" => ReaderInner::WD2(WD2Reader::new(reader).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(e.to_string())
            })?),
            _     => return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("unknown reader type '{}', expected 'WD1' or 'WD2'", reader_type)
                    )),
        };
        Ok(Self { inner })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> Option<Py<PyAny>> {
 
        // translate into python dictionary
        let dict = PyDict::new(py);

        match &mut slf.inner {
            ReaderInner::WD1(reader) => {
                // pull out the next component from the rust iterator
                let event = reader.next()?.ok()?;
                dict.set_item("event_size",       event.header.event_size).unwrap();
                dict.set_item("board_id",         event.header.board_id).unwrap();
                dict.set_item("pattern",          event.header.pattern).unwrap();
                dict.set_item("board_channel",    event.header.board_channel).unwrap();
                dict.set_item("event_counter",    event.header.event_counter).unwrap();
                dict.set_item("timestamp",        event.header.timestamp).unwrap();

                let array = PyArray1::from_vec(py, event.data);
                dict.set_item("data", array).unwrap();
                       
            },
            ReaderInner::WD2(reader) => {
                let event = reader.next()?.ok()?;
                dict.set_item("event_counter",       event.header.event_counter).unwrap();
                dict.set_item("timestamp",           event.header.timestamp).unwrap();
                dict.set_item("samples",             event.header.samples).unwrap();
                dict.set_item("sampling_period",     event.header.sampling_period).unwrap();
                dict.set_item("channels",            event.header.channels).unwrap();

                let array = PyArray1::from_vec(py, event.data);
                dict.set_item("data", array).unwrap();
   

            },
        }

        
        // return the dictionary
        Some(dict.into())
    }
}

// Implement wd1 reader with COLT 
#[pymodule]
fn colt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EventReader>()?;
    Ok(())
}
