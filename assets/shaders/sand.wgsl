struct Matter {
    info: u32,
    color: u32,
}

struct GridConfig {
    width: u32,
    height: u32,
    mouse_x: u32,
    mouse_y: u32,
    brush_radius: u32,
    brush_matter: u32,
    brush_color: u32,
    is_drawing: u32,
    time: u32, 
    run_physics: u32,
}

@group(0) @binding(0) var<storage, read> config: GridConfig;
@group(0) @binding(1) var<storage, read> grid_in: array<Matter>;
@group(0) @binding(2) var<storage, read_write> grid_out: array<Matter>;
@group(0) @binding(3) var out_texture: texture_storage_2d<rgba8unorm, write>;

fn random(seed: vec3<u32>) -> f32 {
    let n = dot(vec3<f32>(seed), vec3<f32>(12.9898, 78.233, 39.346));
    return fract(sin(n) * 43758.5453);
}

// Safely grabs a pixel, treating out-of-bounds as indestructible Rock (2)
fn get_matter(x: i32, y: i32) -> Matter {
    if (x < 0 || y < 0 || x >= i32(config.width) || y >= i32(config.height)) {
        return Matter(2u, 0x00000000u);
    }
    return grid_in[u32(y) * config.width + u32(x)];
}

// Returns true if 'src' can push 'dst' out of the way
fn can_displace(src: u32, dst: u32) -> bool {
    if (src == 1u && (dst == 0u || dst == 3u)) { return true; } // Sand sinks in Water and Empty
    if (src == 3u && dst == 0u) { return true; }                // Water sinks in Empty
    return false;
}

// STEP 1: Where does this cell WANT to go? (Evaluated safely by all threads)
fn get_target(x: i32, y: i32) -> vec2<i32> {
    let me = get_matter(x, y);
    let pos = vec2<i32>(x, y);
    
    if (me.info == 2u || me.info == 0u) { return pos; } // Rock/Empty don't move

    // Use time to alternate diagonal preferences so piles form evenly
    let bias_left = (config.time + u32(x) + u32(y)) % 2u == 0u;

    if (me.info == 1u || me.info == 3u) {
        let d = get_matter(x, y + 1);
        if (can_displace(me.info, d.info)) { return vec2<i32>(x, y + 1); }

        let dl = get_matter(x - 1, y + 1);
        let dr = get_matter(x + 1, y + 1);
        if (bias_left) {
            if (can_displace(me.info, dl.info)) { return vec2<i32>(x - 1, y + 1); }
            if (can_displace(me.info, dr.info)) { return vec2<i32>(x + 1, y + 1); }
        } else {
            if (can_displace(me.info, dr.info)) { return vec2<i32>(x + 1, y + 1); }
            if (can_displace(me.info, dl.info)) { return vec2<i32>(x - 1, y + 1); }
        }
    }
    
    // Water Spreading
    if (me.info == 3u) {
        let l = get_matter(x - 1, y);
        let r = get_matter(x + 1, y);
        if (bias_left) {
            if (can_displace(me.info, l.info)) { return vec2<i32>(x - 1, y); }
            if (can_displace(me.info, r.info)) { return vec2<i32>(x + 1, y); }
        } else {
            if (can_displace(me.info, r.info)) { return vec2<i32>(x + 1, y); }
            if (can_displace(me.info, l.info)) { return vec2<i32>(x - 1, y); }
        }
    }
    return pos;
}

// STEP 2: Who WINS the right to enter this cell?
fn get_winner(x: i32, y: i32) -> vec2<i32> {
    let pos = vec2<i32>(x, y);
    
    // Priority 1: Gravity (Straight down)
    if (all(get_target(x, y - 1) == pos)) { return vec2<i32>(x, y - 1); }

    // Priority 2: Diagonal Falling
    let bias_above = (config.time + u32(x) + u32(y - 1)) % 2u == 0u;
    if (bias_above) {
        if (all(get_target(x + 1, y - 1) == pos)) { return vec2<i32>(x + 1, y - 1); }
        if (all(get_target(x - 1, y - 1) == pos)) { return vec2<i32>(x - 1, y - 1); }
    } else {
        if (all(get_target(x - 1, y - 1) == pos)) { return vec2<i32>(x - 1, y - 1); }
        if (all(get_target(x + 1, y - 1) == pos)) { return vec2<i32>(x + 1, y - 1); }
    }

    // Priority 3: Horizontal Spreading (Water)
    let bias_sides = (config.time + u32(x) + u32(y)) % 2u == 0u;
    if (bias_sides) {
        if (all(get_target(x + 1, y) == pos)) { return vec2<i32>(x + 1, y); }
        if (all(get_target(x - 1, y) == pos)) { return vec2<i32>(x - 1, y); }
    } else {
        if (all(get_target(x - 1, y) == pos)) { return vec2<i32>(x - 1, y); }
        if (all(get_target(x + 1, y) == pos)) { return vec2<i32>(x + 1, y); }
    }

    return pos; // No one wants to enter, so the cell itself wins.
}

// STEP 3: The Handshake Execution
fn process_matter(x: i32, y: i32) -> Matter {
    let pos = vec2<i32>(x, y);
    let me = get_matter(x, y);

    // Am I receiving someone?
    let winner = get_winner(x, y);
    if (any(winner != pos)) {
        return get_matter(winner.x, winner.y); // I become the winner!
    }

    // Am I moving away?
    let t = get_target(x, y);
    if (any(t != pos)) {
        let target_winner = get_winner(t.x, t.y);
        if (all(target_winner == pos)) {
            return get_matter(t.x, t.y); // Target accepted me, I swap with them!
        }
    }

    return me; // Nothing happened, I stay the same.
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = i32(global_id.x);
    let y = i32(global_id.y);
    if (x >= i32(config.width) || y >= i32(config.height)) { return; }

    let idx = u32(y) * config.width + u32(x);
    let current = get_matter(x, y);

    // Draw to Screen
    let r = f32((current.color >> 24u) & 0xFFu) / 255.0;
    let g = f32((current.color >> 16u) & 0xFFu) / 255.0;
    let b = f32((current.color >> 8u) & 0xFFu) / 255.0;
    let a = f32(current.color & 0xFFu) / 255.0;
    textureStore(out_texture, vec2<i32>(x, y), vec4<f32>(r, g, b, a));

    // Mouse Draw Override
    if (config.is_drawing == 1u) {
        let dx = x - i32(config.mouse_x);
        let dy = y - i32(config.mouse_y);
        if (dx * dx + dy * dy <= i32(config.brush_radius * config.brush_radius)) {
            var final_color = config.brush_color;
            if (config.brush_matter != 0u) {
                let br = f32((config.brush_color >> 24u) & 0xFFu);
                let bg = f32((config.brush_color >> 16u) & 0xFFu);
                let bb = f32((config.brush_color >> 8u) & 0xFFu);
                let noise = (random(vec3<u32>(u32(x), u32(y), config.time)) - 0.5) * 40.0;
                final_color = (u32(clamp(br + noise, 0.0, 255.0)) << 24u) | 
                              (u32(clamp(bg + noise, 0.0, 255.0)) << 16u) | 
                              (u32(clamp(bb + noise, 0.0, 255.0)) << 8u) | 0xFFu;
            }
            grid_out[idx] = Matter(config.brush_matter, final_color);
            return; 
        }
    }

    if (config.run_physics == 1u) { grid_out[idx] = process_matter(x, y); } 
    else { grid_out[idx] = current; }
}