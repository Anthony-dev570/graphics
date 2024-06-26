#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aUv;

uniform mat4 projection;

out vec2 Uv;

void main() {
    gl_Position = projection * vec4(aPos, 1);
    Uv = aUv;
}