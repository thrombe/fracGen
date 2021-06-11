package main
import (
	"math"
)

//returns a len(values), 1 matrix
func createVec(vals ...float64) [][]float64 {
    return matrix(len(vals), 1, vals...)
}

//returns the size of a n-dimentional vector
func vecSize(vec [][]float64) float64 {
    if len(vec[0]) > 1 {panic("vecSize not a vector")}
    vecDimensions := len(vec)
    var result float64
    for r := 0; r < vecDimensions; r++ {
        result += vec[r][0] * vec[r][0]
    }
    return math.Sqrt(result)
}

//returns unit vector in the same direction
func vecUnit(vec [][]float64) [][]float64 {
    size := vecSize(vec)
    rows := len(vec)
    // result := make([][]float64, rows)
    result := matrix(rows, 1)
    for r := 0; r < rows; r++ {
        result[r][0] = vec[r][0]/size
    }
    return result
}

//returns dot of 2 vectors
func vecDot(vec1, vec2 [][]float64) float64 {
    if len(vec1[0]) > 1 || len(vec2[0]) > 1 {panic("vecDot not a vector")}
    if len(vec1) != len(vec2) {panic("vecDot vectors of different dimentions")}
    vecDimensions := len(vec1)
    var result float64
    for r := 0; r < vecDimensions; r++ {
        result += vec1[r][0] * vec2[r][0]
    }
    return result
}

// apply a function to each element of a vector
func vecApply(vec [][]float64, f func(float64) float64) [][]float64 {
    length := len(vec)
    vals := make([]float64, length)
    for i := 0; i < length; i++ {
        vals[i] = f(vec[i][0])
    }
    return createVec(vals...)
}

// linear interpolation between 2 vectors
func vecLerp(vec1, vec2 [][]float64, t float64) [][]float64 {
    return matAdd(matScalar(vec1, t), matScalar(vec2, 1-t))
}
