// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use core::fmt;
use std::error::Error;

use ballista::prelude::BallistaError as InnerBallistaError;
use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError as InnerDataFusionError;
use pyo3::{exceptions::PyException, PyErr};

#[derive(Debug)]
pub enum DataFusionError {
    ExecutionError(InnerDataFusionError),
    ArrowError(ArrowError),
    Common(String),
    PythonError(PyErr),
}

impl fmt::Display for DataFusionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataFusionError::ExecutionError(e) => write!(f, "DataFusion error: {:?}", e),
            DataFusionError::ArrowError(e) => write!(f, "Arrow error: {:?}", e),
            DataFusionError::PythonError(e) => write!(f, "Python error {:?}", e),
            DataFusionError::Common(e) => write!(f, "{}", e),
        }
    }
}

impl From<ArrowError> for DataFusionError {
    fn from(err: ArrowError) -> DataFusionError {
        DataFusionError::ArrowError(err)
    }
}

impl From<InnerDataFusionError> for DataFusionError {
    fn from(err: InnerDataFusionError) -> DataFusionError {
        DataFusionError::ExecutionError(err)
    }
}

impl From<PyErr> for DataFusionError {
    fn from(err: PyErr) -> DataFusionError {
        DataFusionError::PythonError(err)
    }
}

impl From<DataFusionError> for PyErr {
    fn from(err: DataFusionError) -> PyErr {
        match err {
            DataFusionError::PythonError(py_err) => py_err,
            _ => PyException::new_err(err.to_string()),
        }
    }
}

impl Error for DataFusionError {}

#[derive(Debug)]
pub enum BallistaError {
    DataFusionExecutionError(InnerDataFusionError),
    ExecutionError(InnerBallistaError),
    ArrowError(ArrowError),
    Common(String),
}

impl From<InnerDataFusionError> for BallistaError {
    fn from(err: InnerDataFusionError) -> BallistaError {
        BallistaError::DataFusionExecutionError(err)
    }
}

impl From<InnerBallistaError> for BallistaError {
    fn from(err: InnerBallistaError) -> BallistaError {
        BallistaError::ExecutionError(err)
    }
}

impl fmt::Display for BallistaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BallistaError::DataFusionExecutionError(e) => {
                write!(f, "DataFusion error: {:?}", e)
            }
            BallistaError::ExecutionError(e) => write!(f, "Ballista error: {:?}", e),
            BallistaError::ArrowError(e) => write!(f, "Arrow error: {:?}", e),
            BallistaError::Common(e) => write!(f, "{}", e),
        }
    }
}

impl From<BallistaError> for PyErr {
    fn from(err: BallistaError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
