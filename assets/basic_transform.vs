attribute vec3 position;
uniform mat4 proj;

void main() {
	vec4 world_pos = vec4(position, 1.0);
	gl_Position = proj * world_pos;
}
