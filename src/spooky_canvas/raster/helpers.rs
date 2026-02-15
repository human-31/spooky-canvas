//! # test docs in spooky_canvas/raster/helpers.rs
//! 
//! ## spooky_canvas/raster/helpers.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/helpers.rs h3
//! 
//! - 10
//! - 31
//! - 06

// TODO: split into more descriptive modules

/// Calculates the edge value of a point and two vertices.
/// 
/// point = (x, y)
/// v1 = (x1, y1)
/// v2 = (x2, y2)
pub fn edge(point: (i32, i32), v1: (i32, i32), v2: (i32, i32)) -> i64 {
    let (x, y) = (point.0 as i64, point.1 as i64);
    let (x1, y1) = (v1.0 as i64, v1.1 as i64);
    let (x2, y2) = (v2.0 as i64, v2.1 as i64);

    (x - x1) * (y2 - y1) - (y - y1) * (x2 - x1)
}

/// Determines if a given point is inside a triangle given the edge values.
pub fn edges_in_triangle(edge_1: i64, edge_2: i64, edge_3: i64) -> bool {
    if edge_1 >= 0 && edge_2 >= 0 && edge_3 >= 0 {
        return true;
    }

    if edge_1 <= 0 && edge_2 <= 0 && edge_3 <= 0 {
        return true;
    }

    return false;
}

/// Determines if a given point is inside a triangle given its vertices.
/// 
/// point = (x, y)
/// v1 = (x1, y1)
/// v2 = (x2, y2)
/// v3 = (x3, y3)
pub fn point_in_triangle(
    point: (i32, i32),
    v1: (i32, i32),
    v2: (i32, i32),
    v3: (i32, i32),
) -> bool {
    let edge_1 = edge(point, v1, v2);
    let edge_2 = edge(point, v2, v3);
    let edge_3 = edge(point, v3, v1);

    edges_in_triangle(edge_1, edge_2, edge_3)
}