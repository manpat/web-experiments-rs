attribute vec2 position;

uniform mat4 u_proj;
varying vec2 v_pos;

void main() {
	vec4 world_pos = vec4(position, 0.0, 1.0);
	gl_Position = u_proj * world_pos;

	v_pos = position;
}
