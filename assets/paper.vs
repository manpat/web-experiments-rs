attribute vec2 position;
attribute vec4 color;

uniform mat4 u_proj;
varying vec4 v_color;

void main() {
	vec4 world_pos = vec4(position, 0.0, 1.0);
	gl_Position = u_proj * world_pos;
	v_color = color;
}
