package main
import (
    "math"
)

//returns a matrix with entered values or empty matrix if no values entered
func matrix(rows, cols int, vals ...float64) [][]float64 {
    if len(vals) != rows*cols {
        if len(vals) == 0 {vals = make([]float64, rows*cols)} else {panic("matrix() not enough values")}
    }
    mat := make([][]float64, rows)
    for r := 0; r < rows; r++ {
        mat[r] = vals[r*cols : (r+1)*cols]
    }
    return mat
}

//multiply matrices of arbitary sizes(legal only ofc)
func matMul(mat1, mat2 [][]float64) [][]float64 {
    m1rows, m1cols, m2rows, m2cols := len(mat1), len(mat1[0]), len(mat2), len(mat2[0])
    if m1cols != m2rows {panic("matMul shape error")}
    result := matrix(m1rows, m2cols)
    for r := 0; r < m1rows; r++ {
        for c := 0; c < m2cols; c++ {
            for item := 0; item < m1cols; item++ {
                result[r][c] += mat1[r][item]*mat2[item][c]
            }
        }
    }
    return result
}

//multiply any no. of matrices (in order)
func nMatMul(mats...[][]float64) [][]float64 {
    if len(mats) < 2 {panic("not enogh matrices in nMatMul")}
    var result [][]float64
    result = mats[0]
    for _, mat := range mats[1:] {
        result = matMul(result, mat)
    }
    return result
}

//returns the addition of two similarly shaped matrices
func matAdd(mat1, mat2 [][]float64) [][]float64 {
    m1rows, m1cols, m2rows, m2cols := len(mat1), len(mat1[0]), len(mat2), len(mat2[0])
    if !(m1rows == m2rows && m1cols == m2cols) {panic("matAdd shape error")}
    result := matrix(m1rows, m1cols)
    for r := 0; r < m1rows; r++ {
        for c := 0; c < m1cols; c++ {
            result[r][c] = mat1[r][c] + mat2[r][c]
        }
    }
    return result
}

//multiply any no. of matrices (in order)
func nMatAdd(mats...[][]float64) [][]float64 {
    if len(mats) < 2 {panic("not enogh matrices in nMatMul")}
    var result [][]float64
    result = mats[0]
    for _, mat := range mats[1:] {
        result = matAdd(result, mat)
    }
    return result
}
/*
//returns the addition of multiple similarly shaped matrices
func matAdd2(mats...[][]float64) [][]float64 {
    if len(mats) < 2 {panic("not enogh matrices in matAdd")}
    m1rows, m1cols := len(mats[0]), len(mats[0][0])
    result := make([][]float64, m1rows)
    copy(result, mats[0])
    for _, mat := range mats[1:] {
        if !(m1rows == len(mat) && m1cols == len(mat[0])) {panic("matAdd shape error")}
        for r := 0; r < m1rows; r++ {
            for c := 0; c < m1cols; c++ {
                result[r][c] += mat[r][c]
            }
        }
    }
    return result
}*/

//returns the subtraction of the second matrix from first
func matSub(mat1, mat2 [][]float64) [][]float64 {
    m1rows, m1cols, m2rows, m2cols := len(mat1), len(mat1[0]), len(mat2), len(mat2[0])
    if !(m1rows == m2rows && m1cols == m2cols) {panic("matSub shape error")}
    result := matrix(m1rows, m1cols)
    for r := 0; r < m1rows; r++ {
        for c := 0; c < m1cols; c++ {
            result[r][c] = mat1[r][c] - mat2[r][c]
        }
    }
    return result
}

//multiply a scalar to a matrix
func matScalar(mat [][]float64, scale float64) [][]float64 {
    mrows, mcols := len(mat), len(mat[0])
    result := matrix(mrows, mcols)
    for r := 0; r < mrows; r++ {
        for c := 0; c < mcols; c++ {
            result[r][c] = mat[r][c]*scale
        }
    }
    return result
}

//remember to input y and x index resp.
func subMat(mat [][]float64, y, x int) [][]float64 {
    mrows := len(mat)
    submat := matrix(mrows-1, mrows-1)
    h, k := 0, 0
    for r := 0; r < mrows; r++ {
        if r == y {continue}
        k++
        for c := 0; c < mrows; c++ {
            if c == x {continue}
            h++
            submat[k][h] = mat[r][c]
        }
    }
    return submat
    //return matScalar(submat, math.Pow(-1, float64(x+y)))
}

//returns determinant of a square matrix
func matDet(mat [][]float64) float64 {
    mrows, mcols := len(mat), len(mat[0])
    if mrows != mcols {panic("matDet non square matrix")}
    if mrows == 2 {return mat[0][0]*mat[1][1] - mat[0][1]*mat[1][0]}
    var result float64
    for c := 0; c < mrows; c++ { // to be more efficient here, we can search for the row/col with most zeroes
        result += mat[0][c] * math.Pow(-1, float64(c)) * matDet(subMat(mat, 0, c))
    }
    return result
}

//returns transpose of a matrix
func matTranspose(mat [][]float64) [][]float64 {
    mrows, mcols := len(mat), len(mat[0])
    result := matrix(mcols, mrows)
    for r := 0; r < mrows; r++ {
        for c := 0; c < mcols; c++ {
            result[r][c] = mat[c][r]
        }
    }
    return result
}

// returns a 2 by 2 rotation matrix
func rotMat(theta float64) [][]float64 {
    return matrix(2, 2,
        math.Cos(theta), -math.Sin(theta),
        math.Sin(theta), math.Cos(theta),
        )
}