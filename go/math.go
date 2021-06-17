package main
import (
	"math"
)

// returns modulus of the no. |n|
func absVal(n float64) float64 {
    if n >= 0 {return n} else {return -n}
}

//rounds to nearest int and retrun int
func round(i float64) int {
    return int(math.Round(i))
    //llim := int(i)
    //if i-float64(llim) >= 0.5 {return llim+1} else {return llim}
}

// linearly map 1 range to another
func mapRange(fs, fe, ts, te float64) func(float64) float64 {
    scale := ((te-ts)/(fe-fs))
    toff := (te+ts)/2
    foff := (fe+fs)/2
    return func(num float64) float64 {
        // return (num + (ts+te)/2 - (fs+fe)/2)*((te-ts)/(fe-fs))
        return (num-foff)*scale + toff
    }
}

// chop a range and return the extreme vals if outside range
func chopRange(s, e float64) func(float64) float64 {
    return func(num float64) float64 {
        if num < s {return s} else if num > e {return e} else {return num}
    }
}

// returns a square range around x, y. x and y range as powers of 2
func xyrange(pow, x, y float64) (float64, float64, float64, float64) {
    nudge := math.Exp2(pow-1)
    return x-nudge, x+nudge, y+nudge, y-nudge // y+nudge first cuz the y is flipped in the computer things or something
}

// returns minimum value
func min(vals ...float64) float64 {
    minn := vals[0]
    for _, val := range vals {
        if minn > val {minn = val}
    }
    return minn
}

// returns maximim value
func max(vals ...float64) float64 {
    maxx := vals[0]
    for _, val := range vals {
        if maxx < val {maxx = val}
    }
    return maxx
}

/*
//returns a func that returns if a point lies in a triangle (2d)
func inTriangle(vertices [][][]float64) func([][]float64) bool {
    v1v2 := matSub(vertices[1], vertices[0])
    v1v3 := matSub(vertices[2], vertices[0])
    v2v3 := matSub(vertices[2], vertices[1])
    return func(point [][]float64) bool {
        v1p := matSub(point, vertices[0])
        v2p := matSub(point, vertices[1])
        ori := (vecCross(v1v2, v1p)[2][0] > 0)
        if (vecCross(v1v3, v1p)[2][0] < 0) != ori {return false}
        if (vecCross(v2v3, v2p)[2][0] > 0) != ori {return false}
        return true
    }
}*/
