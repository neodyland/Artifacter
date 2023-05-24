use std::str::FromStr;

use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData};
use gen::{generate as rs_generate, ImageFormat, Lang, ScoreCounter};
use once_cell::sync::OnceCell;
use pyo3::{
    prelude::{pyfunction, PyAny, PyResult, Python},
    pymodule,
    types::{PyBytes, PyList, PyModule},
    wrap_pyfunction, ToPyObject,
};

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

/// Load
#[pyfunction]
fn load(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        let enka = tokio::task::spawn_blocking(move || Some(EnkaNetwork::new().ok()))
            .await
            .map_err(|_| pyo3::exceptions::PyException::new_err("EnkaNetwork load failed"))?;
        let enka = enka.ok_or(pyo3::exceptions::PyException::new_err(
            "EnkaNetwork load failed",
        ))?;
        let enka = enka.ok_or(pyo3::exceptions::PyException::new_err(
            "EnkaNetwork load failed",
        ))?;
        let icon_data = enka.icon_data().await;
        let e = ENKA.set(enka);
        if e.is_err() {
            return Err(pyo3::exceptions::PyException::new_err(
                "EnkaNetwork already loaded",
            ));
        }
        let e = ICON_DATA.set(icon_data);
        if e.is_err() {
            return Err(pyo3::exceptions::PyException::new_err(
                "IconData already loaded",
            ));
        }
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// get characters
#[pyfunction]
fn get_characters(py: Python, uid: i32) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let enka = ENKA.get().ok_or(pyo3::exceptions::PyException::new_err(
            "EnkaNetwork not loaded",
        ))?;
        let (user, _) = enka
            .simple(uid)
            .await
            .map_err(|e| pyo3::exceptions::PyException::new_err(e))?;
        let characters = user.profile().show_character_list().to_owned();
        Ok(Python::with_gil(|py| {
            let list = PyList::empty(py);
            for c in characters {
                let a = list.append(c.0);
                if a.is_err() {
                    return py.None();
                }
            }
            list.to_object(py)
        }))
    })
}

/// generater
#[pyfunction]
fn generate(
    py: Python,
    uid: i32,
    cid: u32,
    lang: String,
    format: String,
    counter: String,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let lang = Lang::from(lang.as_str());
        let enka = ENKA.get().ok_or(pyo3::exceptions::PyException::new_err(
            "EnkaNetwork not loaded",
        ))?;
        let icon_data = ICON_DATA
            .get()
            .ok_or(pyo3::exceptions::PyException::new_err(
                "IconData not loaded",
            ))?;
        let (user, _) = enka
            .simple(uid)
            .await
            .map_err(|e| pyo3::exceptions::PyException::new_err(e))?;
        let character = user.character(CharacterId(cid));
        if character.is_none() {
            return Err(pyo3::exceptions::PyException::new_err(
                "Character not found",
            ));
        }
        let character = character.unwrap();
        let counter = ScoreCounter::from_str(counter.as_str())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e))?;
        let format = ImageFormat::from_str(format.as_str())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e))?;
        let result = rs_generate(
            character.to_owned(),
            enka,
            &lang,
            icon_data,
            counter,
            format,
        )
        .await;
        if result.is_none() {
            return Err(pyo3::exceptions::PyException::new_err("Generate failed"));
        }
        let result = result.unwrap();
        Ok(Python::with_gil(|py| {
            let bytes = PyBytes::new(py, &result);
            bytes.to_object(py)
        }))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn artifacter_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(get_characters, m)?)?;
    Ok(())
}
