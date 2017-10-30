#version 400
#extension GL_ARB_gpu_shader_fp64 : require

smooth in vec2 pos;
out vec4 gl_FragColor;

uniform double xMin;
uniform double xMax;
uniform double yMin;
uniform double yMax;

uniform double height;
uniform double width;

uniform double mouseX;
uniform double mouseY;

uniform double maxi;
uniform double time;

// https://stackoverflow.com/questions/15095909/from-rgb-to-hsv-in-opengl-glsl
vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


dvec4 qMul(dvec4 a, dvec4 b) {
    double nw = a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z;
    double nx = a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y;
    double ny = a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x;
    double nz = a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w;
    return dvec4(nw, nx, ny, nz);
}

dvec3 julia_qm(dvec2 cin) {
    dvec4 z = dvec4(cin, mouseX, mouseY);
	dvec4 c = (dvec4(0.0, 0.0, mouseX, mouseY));
    // Setting `c` to 0 here is a derp.
    double istep;
    dvec4 start = c;
    while (length(z) < 2.0 && istep < maxi) {
        z = qMul(z, z) + c;
        istep += 1.0;
    }
    istep /= maxi;
    double dist = length(z - start); // This gives us something nicely continuous
    double hue = istep * 0.05 + time * 0.2; // uh, hue, calm down. not the whole thing.
    vec3 hsv = vec3(hue + smoothstep(0.0, 1.0, istep), 1.0, smoothstep(0.0, 1.0, dist));
    return hsv2rgb(hsv);
}

dvec3 qm(dvec2 cin) {
    dvec4 c = dvec4(cin, mouseX, mouseY);
	dvec4 z = dvec4(0.0, 0.0, 0.0, 0.0);
    double istep;
    dvec4 start = c;
    while (length(z) < 2.0 && istep < maxi) {
        z = qMul(z, z) + c;
        istep += 1.0;
    }
    double dist = length(z - start); // This gives us something nicely continuous
    istep /= maxi;
    double hue = istep * 0.05 + time * 0.2; // uh, hue, calm down. not the whole thing.
    vec3 hsv = vec3(hue + smoothstep(0.0, 1.0, istep), 1.0, smoothstep(0.0, 1.0, dist));
    return hsv2rgb(hsv);
}

void main() {
	dvec2 c = dvec2(
		xMin + (xMax - xMin) * (gl_FragCoord.x / width),
		yMax - (yMax - yMin) * (gl_FragCoord.y / height)
    );
    if (true) {
        gl_FragColor = vec4(qm(c), 1.0);
    } else {
        gl_FragColor = vec4(julia_qm(c), 1.0);
    }
}

