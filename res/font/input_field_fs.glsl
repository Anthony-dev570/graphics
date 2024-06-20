#version 330 core

out vec4 color;

in vec2 Uv;

uniform float radiusX, radiusY;

uniform vec3 u_color;

void main() {
    if (Uv.x <= radiusX || Uv.x >= 1.0 - radiusX || Uv.y <= radiusY || Uv.y >= 1.0 - radiusY) {
        color = vec4(0.3, 0.2, 0.4, 1) * vec4(u_color, 1);
    } else {
        color = vec4(0, 0, 0, 1);
    }
}