use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct UploadStore {
    uploading: RwSignal<bool>,
    progress: RwSignal<u8>,
    dragover: RwSignal<bool>,
    on_upload: RwSignal<bool>,
}

impl UploadStore {
    pub fn new() -> Self {
        Self {
            uploading: RwSignal::new(false),
            progress: RwSignal::new(0),
            dragover: RwSignal::new(false),
            on_upload: RwSignal::new(false),
        }
    }

    pub fn uploading(&self) -> RwSignal<bool> {
        self.uploading
    }

    pub fn progress(&self) -> RwSignal<u8> {
        self.progress
    }

    pub fn dragover(&self) -> RwSignal<bool> {
        self.dragover
    }

    pub fn on_upload(&self) -> RwSignal<bool> {
        self.on_upload
    }

    pub fn set_uploading(&self, v: bool) {
        self.uploading.set(v);
    }

    pub fn set_progress(&self, v: u8) {
        self.progress.set(v);
    }

    pub fn set_dragover(&self, v: bool) {
        self.dragover.set(v);
    }

    pub fn set_on_upload(&self, v: bool) {
        self.on_upload.set(v);
    }
}
