#version 460

layout(local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

layout(set = 0, binding = 0, rgba8) uniform writeonly image2D Data;

layout(push_constant) uniform Params {
    highp float factor_x;
    highp float factor_y;
    highp float offset_x;
    highp float offset_y;
    highp uint amnt_of_lines;
} params;

vec3 hsv_to_rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

// From colors.rs
float colors(float n) {
    {{ colors }}
}

// From shadows.rs
float shadows(float n) {
    {{ shadows }}
}

int from_decimal(float n) {
    return int(mod(n, 1) * 255);
}

void write_data(vec4 res) {
    imageStore(Data, ivec2(gl_GlobalInvocationID.xy), res);
}

struct Complex {
    vec2 data; // data.x => real & data.y => imaginary
};

Complex add(Complex n1, Complex n2) {
    return Complex(
        n1.data + n2.data
    );
}

Complex mult(Complex n1, Complex n2) {
    return Complex(
        vec2(
            n1.data.x * n2.data.x - n1.data.y * n2.data.y,
            n1.data.y * n2.data.x + n1.data.x * n2.data.y
        )
    );
}

void main() {

    highp ivec2 global_dimensions = ivec2({{ width }}, params.amnt_of_lines); // global size
    highp ivec2 local_dimensions = ivec2(imageSize(Data)); // local size

    // Basically 1-dimensionalizes the image
    highp uint global_count = gl_GlobalInvocationID.y * uint(local_dimensions.x) + gl_GlobalInvocationID.x;
    highp uint global_y = global_count / uint(global_dimensions.x);
    highp uint global_x = global_count - global_y * uint(global_dimensions.x);

    highp vec2 cords = vec2(global_x, global_y) / vec2(global_dimensions);

    /*
    write_data(vec4(cords.x, cords.y, 0, 255));
    return;
    */

    Complex c = Complex(vec2({{ c_init }}));
    Complex z = Complex(cords * vec2(params.factor_x, params.factor_y) + vec2(params.offset_x, params.offset_y));

    {{ julia_changes }}

    float z_output = 0.0;
    bool travel_distance = {{ travel_distance }};

    int maxi = {{ max_i }};
    float added = 1.0 / maxi;

    vec3 res;
    Complex previous_z = z;

    int i;
    for (i = 0; i < maxi; i += 1) {

        if (length(z.data) > 2.0) {
            break;
        }

        {{ formula }}

        if (travel_distance) {
            z_output += distance(z.data, previous_z.data);
            previous_z = z;
        } else {
            z_output += 1;
        }
    }

    vec4 out_pixel;

    if (z_output == 0.0) {
        out_pixel = vec4( {{ background }} );
    } else if (z_output == maxi) {
        out_pixel = vec4( {{ foreground }} );
    } else {

        out_pixel = vec4(
            hsv_to_rgb(
                vec3(
                    mod(
                        colors(
                            z_output * {{ rate_of_color_change }}
                        )
                    / 360.0, 1.0),
                    1.0,
                    shadows(z_output)
                )
            ),
            1.0
        );
    }

    write_data(out_pixel);
}
