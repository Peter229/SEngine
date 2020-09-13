#version 330 core

in VS_OUTPUT{

    vec2 UV;
} IN;

out vec4 Color;

uniform sampler2D inTexture;

void main() {

    vec4 texel = texture(inTexture, IN.UV);

    if (texel.w < 0.1) {
        discard;
    }

    Color = texel;
}