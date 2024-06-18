#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aUv;

out vec Uv;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(aPos, 0, 1);
    Uv = auv;
}