# Report
## Tested Environment
- Compiler: `rustc` 1.46.0
- Operating System: Windows 10 Pro 20H2

## Works
- 3D space representation of a parallelogram
- Intersection equation of ray-parallelogram
- Calculate the normal of a given parallelogram
- Calculate the origin and the ray direction of a perspective camera
- Shading a sphere using the Phong-Blinn model

## Explainations
- Similar to triangles, which we've discussed in class, the 3D representation of a parallelogram also uses an origin point plus two directional vector. The difference is that triangle requires the sum of two coefficients no more than one, however the parallelogram requires each of them no larger than one.
- The linear equation is just the same as the triangles, and the difference is just the constraints.
- The normal calculation is also the same as triangles. We should pay attention to the order of the cross product, since reversed order would point the normal in the opposite direction, which could not calculate the light.
- The perspective camera has a fixed origin point, and all we need to do is to calculate the center of each pixel and subtract that vector with the origin, and don't forget to normalize the result.
- In our example, the parallelogram looks smaller if we decrese the z value of the origin point, and it looks larger if we increase that value.
- The shading is the most interesting part. The shading equation is quite straightforward, the interesting thing is to tweak the coefficients. Kd defines the color of the light, and Ks defines the color of the specular light. I defines the intensity of the light, and p defines the arange of the specular light.