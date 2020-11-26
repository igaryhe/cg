# Report
## Tested Environment
- Compiler: `rustc` rustc 1.50.0-nightly (c919f490b 2020-11-17)
- Operating System: Windows 10 Pro 20H2

## Works
- Implemented off file loading. The `load_off` function will return a list of vertices and a list of indices
- Implemented uniform attributes loading from external file
- Implemented orthographic camera and perspective camera
- Implemented blinn-phong shading using "shader"
- Implemented line rendering and triangle rendering
- Implemented PNG rendering and GIF rendering
- Implemented the software rendering pipeline, could load custom "shader"(function pointer in our program)

## Explaination
The `load_off` function could load meshes from off files. It will load all the vertices into a vector(data structure), and load the corresponding indices into another vector. If the target primitive is triangle, it will just load the indices respectively into the list; if the target primitive is line, it will load the three edges of a triangle respectively into the list. When loading vertices, we calculate the normals. When traversing the indices, we calculate the face normal and added that value to the corresponding `VertexAttribute`. After the traverse, we normalize the normals for all the `VertexAttribute`. When performing flat shading, we recalculate the normal in the `rasterize_triangle` function, so that we keep the overall render pipeline unchanged.

We use `serde` to load `UniformAttribute` from an external file. This makes it easier for us when tuning the parameters. The `UniformAttribute` contains the light position and intensity, the material parameters of the mesh, the camera parameters, especially for the perspective camera, the transformation of the mesh in the world space. After loading these data, we calculate the corresponding matrices: model matrix, view matrix, projection matrix and normal matrix. These are used when performing the transformations. The difference between perspective camera and orthographic camera is that they have different projection matrices.

Because the calculation of the lighting happens in fragment shader and world space, and we need to perform all the required transformations in vertex shader, we need to pass a dedicate world space coordinate into fragment shader. The normal would only affect by rotation and scaling, so it's actually transposed inverse of the upper left corner(3x3) of the model matrix.

The blinn-phong shading is just the same as what we did in assignment 2/3, without the shadow calculation.

Rendering PNG file is quite straightforward, and rendering GIF is just like: in a pre-defined loop, each time changes the uniform attribute a little bit(the position and the rotation of the mesh), and render the scene. Added the rendered frame to a vector(data structure) and generate the GIF file from the vector.
