use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use rand::Rng;

pub const CHUNK_SIZE: usize = 8; // будет 32, но пока это не финальное решение
pub const CHUNK_SIZE_H: usize = CHUNK_SIZE; // размер чанка по горизонтали x, z
pub const CHUNK_SIZE_V: usize = CHUNK_SIZE * 2; // размер чанка по вертикали y

fn generate_data() -> [[[u8; CHUNK_SIZE_V]; CHUNK_SIZE_H]; CHUNK_SIZE_H] {
    let mut data: [[[u8; CHUNK_SIZE_V]; CHUNK_SIZE_H]; CHUNK_SIZE_H] =
        [[[0; CHUNK_SIZE_V]; CHUNK_SIZE_H]; CHUNK_SIZE_H];

    let mut rng = rand::rng();

    for x in 0..CHUNK_SIZE_H {
        for z in 0..CHUNK_SIZE_H {
            for y in 0..CHUNK_SIZE_V {
                if rng.random_range(0..2) % 2 == 1 {
                    data[x][z][y] = 1u8;
                }
            }
        }
    }

    data
}

pub fn generate() -> Mesh {
    let data = generate_data();

    let mut vecs = Vec::<[f32; 3]>::new();
    let mut uv0s = Vec::<[f32; 2]>::new();
    let mut nrls = Vec::<[f32; 3]>::new();
    // let mut inds = Vec::<u32>::new();
    let mut side_count = 0;

    for x in 0..CHUNK_SIZE_H {
        let xf = x as f32;
        for z in 0..CHUNK_SIZE_H {
            let zf = z as f32;
            for y in 0..CHUNK_SIZE_V {
                if data[x][z][y] == 1u8 {
                    let yf = y as f32;

                    if y == CHUNK_SIZE_V - 1 || data[x][z][y + 1] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [0.0 + xf, 1.0 + yf, 0.0 + zf],
                            [1.0 + xf, 1.0 + yf, 0.0 + zf],
                            [1.0 + xf, 1.0 + yf, 1.0 + zf],
                            [0.0 + xf, 1.0 + yf, 1.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[0.0, 1.0, 0.0]; 4]);
                    }

                    if y == 0 || data[x][z][y - 1] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [0.0 + xf, 0.0 + yf, 1.0 + zf],
                            [1.0 + xf, 0.0 + yf, 1.0 + zf],
                            [1.0 + xf, 0.0 + yf, 0.0 + zf],
                            [0.0 + xf, 0.0 + yf, 0.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[0.0, -1.0, 0.0]; 4]);
                    }

                    if x == CHUNK_SIZE_H - 1 || data[x + 1][z][y] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [1.0 + xf, 0.0 + yf, 0.0 + zf],
                            [1.0 + xf, 0.0 + yf, 1.0 + zf],
                            [1.0 + xf, 1.0 + yf, 1.0 + zf],
                            [1.0 + xf, 1.0 + yf, 0.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[1.0, 0.0, 0.0]; 4]);
                    }

                    if x == 0 || data[x - 1][z][y] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [0.0 + xf, 0.0 + yf, 0.0 + zf],
                            [0.0 + xf, 1.0 + yf, 0.0 + zf],
                            [0.0 + xf, 1.0 + yf, 1.0 + zf],
                            [0.0 + xf, 0.0 + yf, 1.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[-1.0, 0.0, 0.0]; 4]);
                    }

                    if z == CHUNK_SIZE_H - 1 || data[x][z + 1][y] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [1.0 + xf, 0.0 + yf, 1.0 + zf],
                            [0.0 + xf, 0.0 + yf, 1.0 + zf],
                            [0.0 + xf, 1.0 + yf, 1.0 + zf],
                            [1.0 + xf, 1.0 + yf, 1.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[0.0, 0.0, 1.0]; 4]);
                    }

                    if z == 0 || data[x][z - 1][y] == 0 {
                        side_count += 1;

                        vecs.extend([
                            [1.0 + xf, 0.0 + yf, 0.0 + zf],
                            [1.0 + xf, 1.0 + yf, 0.0 + zf],
                            [0.0 + xf, 1.0 + yf, 0.0 + zf],
                            [0.0 + xf, 0.0 + yf, 0.0 + zf],
                        ]);

                        uv0s.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
                        nrls.extend([[0.0, 0.0, -1.0]; 4]);
                    }
                }
            }
        }
    }
    let inds = (0..side_count)
        .flat_map(|i| [i * 4, i * 4 + 3, i * 4 + 1, i * 4 + 1, i * 4 + 3, i * 4 + 2])
        .collect();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vecs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv0s)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrls)
    .with_inserted_indices(Indices::U32(inds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        generate();
    }
}
