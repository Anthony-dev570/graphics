#version 330 core

in vec2 Uv;

out vec4 color;

void main() {
    color = vec4(Uv, 0, 1);
}