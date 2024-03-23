#version 330
out vec4 FragColor;

struct UiMaterial {
    sampler2D surface;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

uniform vec3 viewPos;
uniform UiMaterial material;
    
void main() {    
    FragColor = texture(material.surface, TexCoord);
}