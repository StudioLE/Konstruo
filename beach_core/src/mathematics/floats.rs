/// <https://stackoverflow.com/a/6400477/247218>
pub fn modulo(a: f32, b: f32) -> f32 {
    a - b * (a / b).floor()
}
