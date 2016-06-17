#version 140

in vec3 position;
in float order;
out float progress;

uniform mat4 matrix;
uniform mat4 view;

void main() {
  progress = order;

  gl_Position = view * matrix * vec4(position, 1.0);
}
