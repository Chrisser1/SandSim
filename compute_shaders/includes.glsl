/*
Specialization constants
*/
layout(constant_id = 0) const int canvas_size_x = 1;
layout(constant_id = 1) const int canvas_size_y = 1;
layout(constant_id = 2) const uint empty_matter = 1;
layout(constant_id = 3) const uint state_empty = 1;
layout(constant_id = 4) const uint state_powder = 1;
layout(constant_id = 5) const uint state_liquid = 1;
layout(constant_id = 6) const uint state_solid = 1;
layout(constant_id = 7) const uint state_solid_gravity = 1;
layout(constant_id = 8) const uint state_gas = 1;
layout(constant_id = 9) const uint state_energy = 1;
layout(constant_id = 10) const uint state_object = 1;

layout(local_size_x_id = 11, local_size_y_id = 12, local_size_z = 1) in;

/*
Buffers
*/
layout(set = 0, binding = 0) restrict buffer MatterInBuffer { uint matter_in[]; };
layout(set = 0, binding = 1) restrict writeonly buffer MatterOutBuffer { uint matter_out[]; };
layout(set = 0, binding = 2, rgba8) restrict uniform writeonly image2D canvas_img;
layout(set = 0, binding = 3) restrict writeonly buffer QueryMatterBuffer { uint query_matter[]; };

layout(push_constant) uniform PushConstants {
    uint sim_step;
    uint move_step;
    vec2 draw_pos_start;
    vec2 draw_pos_end;
    float draw_radius;
    uint draw_matter;
    ivec2 query_pos;
} push_constants;

#include "dirs.glsl"
#include "matter.glsl"

/*
Utility functions to be used in the various kernels:
*/

ivec2 get_current_sim_pos() {
    return ivec2(gl_GlobalInvocationID.xy);
}

int get_index(ivec2 pos) {
    return pos.y * canvas_size_y + pos.x;
}

bool is_at_border_top(ivec2 pos) {
    return pos.y == canvas_size_y - 1;
}

bool is_at_border_bottom(ivec2 pos) {
    return pos.y == 0;
}

bool is_at_border_right(ivec2 pos) {
    return pos.x == canvas_size_x - 1;
}

bool is_at_border_left(ivec2 pos) {
    return pos.x == 0;
}

bool is_inside_sim_canvas(ivec2 pos) {
    return pos.x >= 0 && pos.x < canvas_size_x &&
    pos.y >= 0 && pos.y < canvas_size_y;
}

Matter read_matter(ivec2 pos) {
    return new_matter(matter_in[get_index(pos)]);
}

uint matter_to_uint(Matter matter) {
    return ((matter.color << uint(8)) | matter.matter);
}

void write_query_matter(Matter matter) {
    query_matter[0] = matter_to_uint(matter);
}

void write_matter(ivec2 pos, Matter matter) {
    matter_out[get_index(pos)] = matter_to_uint(matter);
}

void write_matter_input(ivec2 pos, Matter matter) {
    matter_in[get_index(pos)] = matter_to_uint(matter);
}

void write_image_color(ivec2 pos, vec4 color) {
    imageStore(canvas_img, pos, color);
}

ivec2 get_pos_at_dir(ivec2 pos, int dir) {
    return pos + OFFSETS[dir];
}

// | 0 1 2 |
// | 7 x 3 |
// | 6 5 4 |
Matter get_neighbor(ivec2 pos, int dir) {
    ivec2 neighbor_pos = get_pos_at_dir(pos, dir);
    if (is_inside_sim_canvas(neighbor_pos)) {
        return read_matter(neighbor_pos);
    } else {
        return new_matter(empty_matter);
    }
}

bool is_empty(Matter matter) {
    return matter.matter == 0;
}

bool is_gravity(Matter m) {
    return m.matter == 1 || m.matter == 3;
}

bool falls_on_empty(Matter from, Matter to) {
    return is_gravity(from) && is_empty(to);
}

bool slides_on_empty(Matter from_diagonal, Matter to_diagonal, Matter from_down, Matter side) {
    return is_gravity(from_diagonal) && !is_empty(from_down) && is_empty(to_diagonal) && is_empty(side);
}

// /// From could move to one direction to empty only
// bool moves_on_empty_certainly(Matter from, Matter to, Matter opposite, Matter down) {
//     return push_constants.dispersion_step < from.dispersion &&
//     ((is_liquid(from) && !is_empty(down)) || is_gas(from)) &&
//     is_empty(to) && !is_empty(opposite);
// }

// /// From could move to one direction to liquid only
// bool moves_on_swap_certainly(Matter from, Matter to, Matter opposite) {
//     return push_constants.dispersion_step < from.dispersion &&
//     (is_liquid(from) || is_gas(from)) && (is_liquid(to) || is_gas(to) || is_energy(to)) &&
//     !(is_liquid(opposite) && opposite.weight < from.weight) &&
//     to.weight < from.weight;
// }

// /// From could move to both direction to empty, but takes a change at one direction
// bool moves_on_empty_maybe(Matter from, Matter to, Matter opposite, Matter down, float p) {
//     return p < 0.5 && push_constants.dispersion_step < from.dispersion &&
//     ((is_liquid(from) && !is_empty(down)) || is_gas(from)) &&
//     is_empty(to) && is_empty(opposite);
// }

vec4 matter_color_to_vec4(uint color) {
    return  vec4(float((color >> uint(16)) & uint(255)) / 255.0,
        float((color >> uint(8)) & uint(255)) / 255.0,
        float(color & uint(255)) / 255.0,
        1.0);
}

