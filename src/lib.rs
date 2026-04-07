use pyo3::prelude::*;
use pyo3::types::PyDict;

use std::fs::File;
use std::io::BufReader;

use numpy::PyArray1;


pub mod wd1_reader;

#[pyclass]
struct EventReader {
    inner: crate::wd1_reader::EventReader<BufReader<File>>,
}

#[pymethods]
impl EventReader {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let file   = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self { inner: crate::wd1_reader::EventReader::new(reader)})
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> Option<Py<PyAny>> {
        // pull out the next component from the rust iterator
        let event = slf.inner.next()?.ok()?;
        
        // translate into python dictionary
        let dict = PyDict::new(py);

        dict.set_item("event_size",       event.header.event_size).unwrap();
        dict.set_item("board_id",         event.header.board_id).unwrap();
        dict.set_item("pattern",          event.header.pattern).unwrap();
        dict.set_item("board_channel",    event.header.board_channel).unwrap();
        dict.set_item("event_counter",    event.header.event_counter).unwrap();
        dict.set_item("timestamp",        event.header.timestamp).unwrap();

        let array = PyArray1::from_vec(py, event.data);
        dict.set_item("data", array).unwrap();
        
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
