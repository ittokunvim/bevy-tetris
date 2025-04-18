use bevy::prelude::*;

pub const BLOCK_MAP: [[usize; 10]; 24] = [
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0],
];
pub const I_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        1,2,3,4,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,0,2,0,
        0,0,3,0,
        0,0,4,0,
    ],
    [
        0,0,0,0,
        0,0,0,0,
        1,2,3,4,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        0,3,0,0,
        0,4,0,0,
    ],
];
pub const J_BLOCK: [[usize; 16]; 4] = [
    [
        1,0,0,0,
        2,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,2,0,
        0,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        3,4,0,0,
        0,0,0,0,
    ],
];
pub const L_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,1,0,
        4,3,2,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        4,0,0,0,
        0,0,0,0,
    ],
    [
        1,2,0,0,
        0,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const O_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
];
pub const S_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        0,1,2,0,
        3,4,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
    [
        0,2,1,0,
        3,4,0,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
];
pub const T_BLOCK: [[usize; 16]; 4] = [
    [
        0,1,0,0,
        2,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        2,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const Z_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        1,2,0,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        1,2,0,0,
        0,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const I_COLOR: Color = Color::srgb(0.53, 0.88, 0.99);
pub const J_COLOR: Color = Color::srgb(0.05, 0.72, 0.84);
pub const L_COLOR: Color = Color::srgb(1.00, 0.59, 0.42);
pub const O_COLOR: Color = Color::srgb(1.00, 0.78, 0.47);
pub const S_COLOR: Color = Color::srgb(0.31, 0.84, 0.75);
pub const T_COLOR: Color = Color::srgb(0.75, 0.60, 1.00);
pub const Z_COLOR: Color = Color::srgb(1.00, 0.46, 0.50);
