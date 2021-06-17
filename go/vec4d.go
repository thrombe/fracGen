package main
import (
	"math"
	"math/rand"
)

type vec4d struct {
    x, y, z, w float64
}

// methods

func (vec *vec4d) copy() *vec4d {
	return vector4d(vec.x, vec.y, vec.z, vec.w)
}

func (vec *vec4d) add(vec1 *vec4d) {
	vec.x += vec1.x
	vec.y += vec1.y
	vec.z += vec1.z
	vec.w += vec1.w
}

func (vec *vec4d) sub(vec1 *vec4d) {
	vec.x -= vec1.x
	vec.y -= vec1.y
	vec.z -= vec1.z
	vec.w -= vec1.w
}

func (vec *vec4d) scalar(t float64) {
	vec.x *= t
	vec.y *= t
	vec.z *= t
	vec.w *= t
}

func (vec *vec4d) lerp(vec1 *vec4d, t float64) {
	tinv := 1-t
	vec.x = vec.x*t + vec1.x*tinv
	vec.y = vec.y*t + vec1.y*tinv
	vec.z = vec.z*t + vec1.z*tinv
	vec.w = vec.w*t + vec1.w*tinv
}

func (vec *vec4d) apply(f func(float64) float64) {
	vec.x = f(vec.x)
	vec.y = f(vec.y)
	vec.z = f(vec.z)
	vec.w = f(vec.w)
}

func (vec *vec4d) dot(vec1 *vec4d) float64 {
	return vec.x*vec1.x + vec.y*vec1.y + vec.z*vec1.z + vec.w*vec1.w
}

func (vec *vec4d) size() float64 {
	return math.Sqrt(vec.x*vec.x + vec.y*vec.y + vec.z*vec.z + vec.w*vec.w)
}

func (vec *vec4d) unit() {
	vec.scalar(vec.size())
}

func (vec *vec4d) transform4d(mat [][]float64) {
	vec.x, vec.y, vec.z, vec.w = (mat[0][0]*vec.x + mat[0][1]*vec.y + mat[0][2]*vec.z + mat[0][3]*vec.w), (mat[1][0]*vec.x + mat[1][1]*vec.y + mat[1][2]*vec.z + mat[1][3]*vec.w), (mat[2][0]*vec.x + mat[2][1]*vec.y + mat[2][2]*vec.z + mat[2][3]*vec.w), (mat[3][0]*vec.x + mat[3][1]*vec.y + mat[3][2]*vec.z + mat[3][3]*vec.w)
}

func (vec *vec4d) transform2d(mat [][]float64) {
	vec.x, vec.y = (mat[0][0]*vec.x + mat[0][1]*vec.y), (mat[1][0]*vec.x + mat[1][1]*vec.y)
}

// pointer functions
// all these return a new vector instead of modifying previous

func vector4d(x, y, z, w float64) *vec4d {
    v := vec4d{}
    v.x, v.y, v.z, v.w = x, y, z, w
    return &v
}

func vector3d(x, y, z float64) *vec4d {
    v := vec4d{}
    v.x, v.y, v.z, v.w = x, y, z, 0
    return &v
}

func vector2d(x, y float64) *vec4d {
    v := vec4d{}
    v.x, v.y, v.z, v.w = x, y, 0, 0
    return &v
}

// returns random vector with each element between 0 to 1
func vecRand3d() *vec4d {
	v := vec4d{}
	v.x = rand.Float64()
	v.y = rand.Float64()
	v.z = rand.Float64()
	return &v
}

// returns random vector with each element between 0 to 1
func vecRand2d() *vec4d {
	v := vec4d{}
	v.x = rand.Float64()
	v.y = rand.Float64()
	return &v
}

func veclerp(vec1, vec2 *vec4d, t float64) *vec4d {
    return vecadd(vecscalar(vec1, t), vecscalar(vec2, 1-t))
}

func vecscalar(vec *vec4d, t float64) *vec4d {
	v := vec.copy()
	v.scalar(t)
	return v
}

func vecadd(vec1, vec2 *vec4d) *vec4d {
	v := vec1.copy()
	v.add(vec2)
	return v
}

func vecsub(vec1, vec2 *vec4d) *vec4d {
	v := vec1.copy()
	v.sub(vec2)
	return v
}

func vecapply(vec *vec4d, f func(float64) float64) *vec4d {
	v := vec.copy()
	v.apply(f)
	return v
}

func vecunit(vec *vec4d) *vec4d {
	v := vec.copy()
	v.unit()
	return v
}