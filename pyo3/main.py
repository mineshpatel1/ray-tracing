from ray_tracer import MaterialProps, Sphere, render_scene

diffuse = MaterialProps('Diffuse', colour=(1.0, 0.3, 0.4))
metal = MaterialProps('Metal', colour=(0.1, 0.6, 1.0), fuzz=0.3)
glass = MaterialProps('Glass', refractive_idx=1.5)

sphere1 = Sphere((0, 1.0, 0), 1, glass)
sphere2 = Sphere((-4, 1.0, 0), 1, diffuse)
sphere3 = Sphere((4, 1.0, 0), 1, metal)

out = render_scene(
	[sphere1, sphere2, sphere3],
	400,
	100,
	50,
	v_fov=20,
	look_from=(13, 2, 3),
)
with open('images/py_interface_v1.ppm', 'w') as f:
    f.write(out)

