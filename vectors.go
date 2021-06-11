package main
import (
)

type vec3d struct {
    x, y, z float64
}

func vector3d(x, y, z float64) vec3d {
    v := vec3d{}
    v.x, v.y, v.z = x, y, z
    return v
}

func veclerp(vec1, vec2 vec3d, t float64) vec3d {
    return vecadd(vecscalar(vec1, t), vecscalar(vec2, 1-t))
}

func vecscalar(vec vec3d, t float64) vec3d {
    v := vec3d{}
    v.x = vec.x*t
    v.y = vec.y*t
    v.z = vec.z*t
    return v
}

func vecadd(vec1, vec2 vec3d) vec3d {
    v := vec3d{}
    v.x = vec1.x + vec2.x
    v.y = vec1.y + vec2.y
    v.z = vec1.z + vec2.z
    return v
}