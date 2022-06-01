uniform sampler2D diffuseTexture;
uniform lowp vec4 diffuseColor;
varying lowp vec2 texCoord;

void main(){
  gl_FragColor = diffuseColor * texture2D(diffuseTexture, texCoord);
}
