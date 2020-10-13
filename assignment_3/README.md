# Report
## Tested Environment
- Compiler: `rustc` 1.46.0
- Operating System: Windows 10 Pro 20H2

## Works
- Ex.1: Implemented fov and perspective camera
- Ex.2: Implemented shadow rays
- Ex.3: Implemented reflection & refraction
- Ex.4: Implemented depth of field
- Ex.5: Implemented the animation

## Explaination
The calculation of the field of view is like calculate the length of the base edge of an isosceles triangle with a given top angle. The perspective camera is basically where the origin point is and what is the direction of the ray. The origin point is the position of the camera, and the direction is the normalized vector from the camera position to the pixel position in the focal length plane.

To calculate the shadow ray, we first need to fill the `intersect` function of each object. We could easily copy code from assignment 2 to complete these functions. Then we use `intersect` to complete the `find_nearest_object` function. The function traverses all the objects in the scene and finds if it intersects with the ray. If it intersects, return the corresponding object and the intersection position. With this function, we could cast shadow rays to see if the ray could reach to the light source.

Reflection is quite simple, as we already have the equation from the lecture. We could easily calculate the reflection ray and call another `shoot_ray` recursively. To limit the bounces, we need to decrease the `max_bounce` in each recursion. Refraction is the same, yet we need to calculate for its refraction ray. We could derive the calculation from Snell's Law equation.

To implement depth of field, instead of casting a ray from the position of the camera, we are going to cast rays from a "focal lens" with a small radius. We generate rays from random positions within the circle and we average the result color of the several rays to calculate the final color. Thus, objects on the focal plane are clear and others are blurred.

The animation is quite straightforward. Simply generate several images with a changing property and compress them into a gif.

You could find the results in the "img" folder.