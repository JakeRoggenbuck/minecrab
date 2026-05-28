#version 330 core

in vec2 fragTexCoord;

// Produce a fragment color.
out vec4 fragColor;

uniform sampler2D tex;

void main() {
    fragColor = texture(tex, fragTexCoord);
}
