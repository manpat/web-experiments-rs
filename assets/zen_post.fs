vec4 color = texture2D(u_color_tex, v_uv);
vec4 normal_rgba = texture2D(u_normal_tex, v_uv);

vec3 normal = normalize(normal_rgba.rgb * 2.0 - 1.0);

vec4 normal_transformed = /* u_normal_transform * */ vec4(normal, 0.0);

vec3 sun_direction = normalize(vec3(-0.5, -1.0, -0.5));

float ndotl = dot(normal_transformed.xyz, -sun_direction) + 0.1;
ndotl = clamp(ndotl, 0.4, 1.0);

vec4 ambient_color = vec4(0.0);

vec4 final_color = vec4(color.rgb, 1.0);
if (color.a > 0.1) {
	final_color.rgb *= ndotl;
}

final_color.rgb += ambient_color.rgb * ambient_color.a;

// gl_FragColor = vec4(normal_rgba.a * (normal_transformed.rgb * 0.5 + 0.5), 1.0);
gl_FragColor = final_color;