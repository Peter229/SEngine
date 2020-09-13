#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 UV;

out VS_OUTPUT {

    vec2 UV;
} OUT;

uniform mat4 view_projection;
uniform mat4 model;
uniform int posInImage;
uniform int sizeOfImage;
uniform int flip;

void main() {

    gl_Position = view_projection * model * vec4(Position, 0.0, 1.0);
    vec2 tempUV = vec2(float(mod(int(UV.x) + flip, 2)), UV.y);
    tempUV = (tempUV / sizeOfImage);
    tempUV.x = tempUV.x + (((mod(posInImage, sizeOfImage)) / sizeOfImage));
    tempUV.y = tempUV.y + (floor(float(posInImage) / float(sizeOfImage)) / float(sizeOfImage));
    OUT.UV = tempUV;
}