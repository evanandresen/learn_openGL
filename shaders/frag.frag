#version 330 core

out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D brick;
uniform sampler2D face;

uniform float mix_lvl;

void main() {
        // Sample both textures
    vec4 pngColor = texture(face, TexCoord); // RGBA (with alpha)
    vec3 jpgColor = texture(brick, TexCoord).rgb; // RGB

    // Mix the PNG with the JPG, where the transparent parts of PNG let JPG through
    vec3 finalColor = mix(jpgColor, pngColor.rgb, pngColor.a * mix_lvl);

    // Output the final color with the alpha from the PNG texture
    FragColor = vec4(finalColor, 1.0); // Set alpha to 1.0, so the final output is fully opaque
    
    //FragColor = mix(texture(brick, TexCoord),
                    // texture(face, TexCoord), mix_lvl);
}