use js_sys::Error;
use wasm_bindgen::prelude::*;

use pglt_workspace::workspace::{
    self, ChangeFileParams, CloseFileParams, GetCompletionsParams, GetFileContentParams,
    OpenFileParams, PullDiagnosticsParams, UpdateSettingsParams,
};

mod utils;

pub use crate::utils::DiagnosticPrinter;
use crate::utils::{into_error, set_panic_hook};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));

#[wasm_bindgen]
pub struct Workspace {
    inner: Box<dyn workspace::Workspace>,
}

#[wasm_bindgen]
impl Workspace {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Workspace {
        Workspace {
            inner: workspace::server(),
        }
    }

    #[wasm_bindgen(js_name = getCompletions)]
    pub fn get_completions(
        &self,
        params: IGetCompletionsParams,
    ) -> Result<ICompletionResult, Error> {
        let params: GetCompletionsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.get_completions(params).map_err(into_error)?;
        to_value(&result)
            .map(ICompletionResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = updateSettings)]
    pub fn update_settings(&self, params: IUpdateSettingsParams) -> Result<(), Error> {
        let params: UpdateSettingsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.update_settings(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = openFile)]
    pub fn open_file(&self, params: IOpenFileParams) -> Result<(), Error> {
        let params: OpenFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.open_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = getFileContent)]
    pub fn get_file_content(&self, params: IGetFileContentParams) -> Result<String, Error> {
        let params: GetFileContentParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.get_file_content(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = changeFile)]
    pub fn change_file(&self, params: IChangeFileParams) -> Result<(), Error> {
        let params: ChangeFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.change_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = closeFile)]
    pub fn close_file(&self, params: ICloseFileParams) -> Result<(), Error> {
        let params: CloseFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.close_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = pullDiagnostics)]
    pub fn pull_diagnostics(
        &self,
        params: IPullDiagnosticsParams,
    ) -> Result<IPullDiagnosticsResult, Error> {
        let params: PullDiagnosticsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.pull_diagnostics(params).map_err(into_error)?;
        to_value(&result)
            .map(IPullDiagnosticsResult::from)
            .map_err(into_error)
    }
}

fn to_value<T: serde::ser::Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true))
}
