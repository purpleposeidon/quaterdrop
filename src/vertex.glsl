#version 400

in vec2 p;
smooth out vec2 pos;

void main() {
    pos = p;
	gl_Position = vec4(p, 0.0, 1.0);
}
