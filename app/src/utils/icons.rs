// All SVG icons centralized here. Each returns &'static str for use in view! macro.
// Use with: view! { <span inner_html=icon::LOGO /> }

pub const LOGO: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><polygon points="8,1 15,5 15,11 8,15 1,11 1,5"/></svg>"#;

pub const GRID: &str = r#"<svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2 2h3v3H2zM8 2h3v3H8zM2 8h3v3H2zM8 8h3v3H8z"/></svg>"#;

pub const LIST: &str = r#"<svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2 3.5h9M2 6.5h9M2 9.5h9"/></svg>"#;

pub const UPLOAD: &str = r#"<svg width="22" height="22" viewBox="0 0 22 22" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M11 3.5v10M6.5 9.5L11 5l4.5 4.5"/><path d="M3.5 15v2a2 2 0 002 2h11a2 2 0 002-2v-2"/></svg>"#;

pub const CLOSE: &str = r#"<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M3 3l8 8M11 3l-8 8"/></svg>"#;

pub const CHEVRON_LEFT: &str = r#"<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M8.5 3L4 7l4.5 4"/></svg>"#;

pub const CHEVRON_RIGHT: &str = r#"<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M5.5 3L10 7l-4.5 4"/></svg>"#;

pub const PLUS: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M8 3v10M3 8h10"/></svg>"#;

pub const FOLDER: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M2 4.5v8a1 1 0 001 1h10a1 1 0 001-1V6a1 1 0 00-1-1H8.5L7 3.5H3a1 1 0 00-1 1z"/></svg>"#;

pub const PLAY: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><polygon points="5,2 13,8 5,14"/></svg>"#;
