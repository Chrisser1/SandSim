struct Pixel {
    matter_info: u32,
    color: u32,
}

struct GridConfig {
    width: u32,
    height: u32,
}

@group(0) @binding(0) var<storage, read> config: GridConfig;
@group(0) @binding(1) var<storage, read> grid_in: array<Pixel>;
@group(0) @binding(2) var<storage, read_write> grid_out: array<Pixel>;
@group(0) @binding(3) var out_texture: texture_storage_2d<rgba8unorm, write>;

fn get_idx(x: u32, y: u32) -> u32 {
    return y * config.width + x;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= config.width || y >= config.height) {
        return;
    }

    let idx = get_idx(x, y);
    let pixel = grid_in[idx];

    // Write color to texture
    let r = f32((pixel.color >> 24u) & 0xFFu) / 255.0;
    let g = f32((pixel.color >> 16u) & 0xFFu) / 255.0;
    let b = f32((pixel.color >> 8u) & 0xFFu) / 255.0;
    let a = f32(pixel.color & 0xFFu) / 255.0;
    textureStore(out_texture, vec2<i32>(i32(x), i32(y)), vec4<f32>(r, g, b, a));

    // By default, the pixel stays the exact same
    var next_pixel = pixel;

    // Rule 1: If I am empty, does sand fall into me?
    if (pixel.matter_info == 0u && y > 0u) {
        let above_idx = get_idx(x, y - 1u);
        let above_pixel = grid_in[above_idx];
        
        if (above_pixel.matter_info == 1u) {
            next_pixel = above_pixel; // Pull the sand down into my spot!
        }
    } 
    // Rule 2: If I am sand, do I fall down?
    else if (pixel.matter_info == 1u && y < config.height - 1u) {
        let below_idx = get_idx(x, y + 1u);
        let below_pixel = grid_in[below_idx];
        
        if (below_pixel.matter_info == 0u) {
            next_pixel = Pixel(0u, 0u); // Empty my old spot because I fell!
        }
    }

    // Every thread only ever writes to its OWN index. Zero race conditions!
    grid_out[idx] = next_pixel;
}