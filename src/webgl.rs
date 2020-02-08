use crate::Cell;

pub fn create_vertices(width: u32, height: u32) -> Vec<f32> {
    let mut vertices = Vec::new();
    let row_size = 2.0 / height as f32;
    let col_size = 2.0 / width as f32;
    for i in 0..height {
        let row_north = -((i as f32) * row_size - 1.0);
        let row_south = -(((i + 1) as f32) * row_size - 1.0);
        for j in 0..width {
            let column_east = ((j + 1) as f32) * col_size - 1.0;
            let column_west = (j as f32) * col_size - 1.0;
            vertices.push(column_west);
            vertices.push(row_north);
            vertices.push(column_east);
            vertices.push(row_north);
            vertices.push(column_east);
            vertices.push(row_south);
            
            vertices.push(column_west);
            vertices.push(row_north);
            vertices.push(column_west);
            vertices.push(row_south);
            vertices.push(column_east);
            vertices.push(row_south);
        }
    }
    vertices
}

pub fn create_colors(cells: &Vec<Cell>) -> Vec<u8> {
    let mut colors = Vec::new();
    for cell in cells.as_slice() {
        let c = *cell as u8;
        for _ in 0..6 {
            colors.push(c);
        }
    }
    colors
}
