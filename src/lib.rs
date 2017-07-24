#[macro_use] extern crate cpython;
extern crate numpy;
extern crate ndarray;
use numpy::{PyArray, PyArrayModule, IntoPyResult, IntoPyArray};
use ndarray::ArrayViewD;

use std::cell::RefCell;
use cpython::PyResult;
use simulation::Simulation as RustSimulation;

pub mod star;
pub mod spot;
pub mod profile;
pub mod bounds;
pub mod boundingshape;
pub mod point;
pub mod sun_ccfs;
pub mod fit_rv;
pub mod simulation;
pub mod planck;
pub mod compute_bisector;
pub mod linspace;
pub mod poly_fit_rv;

py_module_initializer!(rather, initrather, PyInit_rather, |py, m| {
    m.add_class::<Simulation>(py)?;
    Ok(())
});

py_class!(class Simulation |py| {

    data sim: RefCell<RustSimulation>;

    def __new__(_cls, filename: &str) -> PyResult<Simulation> {
        Simulation::create_instance(py, RefCell::new(RustSimulation::new(filename)))
    }

    def __repr__(&self) ->PyResult<String> {
        Ok(format!("{:?}", self.sim(py).borrow()))
    }

    def __str__(&self) ->PyResult<String> {
        Ok(format!("{:?}", self.sim(py).borrow()))
    }

    def observe_flux(&self, time: PyArray, wavelength_min: f64, wavelength_max: f64) -> PyResult<PyArray> {
        let np = PyArrayModule::import(py)?;
        let time: ArrayViewD<f64> = time.as_array().into_pyresult(py, "time must be an array of f64")?;
        let time_vec: Vec<f64> = time.iter().cloned().collect();
        let flux = self.sim(py).borrow_mut().observe_flux(&time_vec, wavelength_min, wavelength_max);
        Ok(flux.into_pyarray(py, &np))
    }

    def observe_rv(&self, time: PyArray, wavelength_min: f64, wavelength_max: f64) -> PyResult<(PyArray, PyArray)> {
        let np = PyArrayModule::import(py)?;
        let time: ArrayViewD<f64> = time.as_array().into_pyresult(py, "time must be an array of f64")?;
        let time_vec: Vec<f64> = time.iter().cloned().collect();
        let observations = self.sim(py).borrow_mut().observe_rv(&time_vec, wavelength_min, wavelength_max);

        let rv: Vec<f64> = observations.iter().map(|o| o.rv).collect();
        let mut bis_data = Vec::<f64>::with_capacity(rv.len() * observations[0].bisector.len());
        for bisector in observations.iter().map(|o| o.bisector.clone()) {
            bis_data.extend(bisector);
        }
        let bisectors = PyArray::new::<f64>(py, &np, &[rv.len(), observations[0].bisector.len()]);
        for (input, mut output) in bis_data.iter().zip(bisectors.as_slice_mut().unwrap()) {
            *output = *input;
        }

        Ok((rv.into_pyarray(py, &np), bisectors))
    }
});
