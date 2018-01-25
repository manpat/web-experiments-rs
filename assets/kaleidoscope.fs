vec2 aspect = vec2(u_aspect, 1.0);

float ang = clamp(atan(v_pos.y, v_pos.x * u_aspect) / 3.14159 * 0.5 + 0.5, 0.0, 1.0);
ang = mod(ang + 0.25, 1.0);

float dist = length(v_pos * aspect);

float quantisation = 24.0;

float seg_ang = mod(ang * u_sections * 2.0, 2.0);
if(seg_ang < 0.0)
	seg_ang += 2.0;

if(seg_ang >= 1.0)
	seg_ang = 2.0 - seg_ang;

if(dist >= 1.0)
	dist = 2.0 - dist;

float resamp_ang = seg_ang / u_sections * 3.14159 + u_time / 3.0;
vec2 dir = vec2(cos(resamp_ang), sin(resamp_ang));

vec2 uv = dir * dist * 0.5 + 0.5;
uv = floor(uv * quantisation * aspect) / quantisation / aspect;

// uv = v_pos * aspect * 0.5 + 0.5;

vec4 col = texture2D(u_tex, uv);

gl_FragColor = vec4(col.rgb, 1.0);
// gl_FragColor = vec4(uv, seg_ang, 1.0);