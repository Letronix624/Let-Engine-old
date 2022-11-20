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

ObjectDataids{
	Int ID[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1]
}
ObjectDataPositions{
	vec2 position[]
}
ObjectDataSizes{
	vec2 size[]
}
ObjectDataRotations{
	vec2 rotation[]
}