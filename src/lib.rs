mod ops;
mod vec;

pub use vec::*;

#[cfg(test)]
mod tests {
    use super::*;

    type Vec2 = vec::Vector2<f32>;
}
