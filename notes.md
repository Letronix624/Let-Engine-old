multiple object vertex shader buffer idea

ObjectData {
	int ID
	vec2 position[]
	vec2 size[]
	float rotation[]
}

gl_Position = vec4(rotated)


index = 1
positionx = index
positiony = positionx
sizex = positiony
sizey = sizex
rot = sizey


new idea


idea 1.
vertex buffer position vec2
vertex buffer object index uint[]

storage buffer {
	vec2 position[]
	vec2 size[]
	float rotation[]
	vec4 color[]
} object

idea 2.
vertex buffer position vec2
vertex buffer object position vec2
vertex buffer object size vec2
vertex buffer object rotation float
vertex buffer object color vec4