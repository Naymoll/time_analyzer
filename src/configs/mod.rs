pub mod array_config;
pub mod matrix_config;
pub mod number_config;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Value {
    Int { min: i32, max: i32 },
    Float { min: f32, max: f32 },
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Step {
    None,
    Fixed(usize),
    Multiply(u8),
}
