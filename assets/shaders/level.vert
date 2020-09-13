#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 UV;

out VS_OUTPUT {

    vec2 UV;
} OUT;

uniform mat4 view_projection;
uniform mat4 model;

void main() {

    gl_Position = view_projection * model * vec4(Position, 0.0, 1.0);
    OUT.UV = UV;
}