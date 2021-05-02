#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec4 fragPos;
layout(location = 1) in vec2 in_uv;

layout(set = 2, binding = 0) uniform BackgroundMaterial_color {
    vec4 color;
};


void main() {
    vec3 col = vec3(0.0);

    vec2 uv = in_uv - 0.5;

    float d = length(uv);
    float m = 0.10 / d;
    m *= 0.1;
    
    col += vec3(m);

    o_Target = vec4(col, 1.0);
}
