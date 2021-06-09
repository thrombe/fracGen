package main
import (
	"math"
	"fmt"
)

func mandlebrot() {
    width := 2000
    height := 2000
    img, set := newImg(width, height)
    
    iterations := 1000
    // itmap := mapRange(0, float64(iterations), 0, 255)
    // itboard := make([]int, width*height)
    // xfrom, xto := -2.0, 1.0
    // yfrom, yto := 1.5, -1.5
    xfrom, xto, yfrom, yto := xyrange(-6, -0.74571890570893210, -0.11624642707064532)
    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)

    julia := complex(0, 0) // 1, 0 is on
    je := 0.25 + 0.0i

    var j complex128
    if julia == complex(1, 0) {j = 0} else {j = 1}    
    for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            c := complex(xmap(float64(x)), ymap(float64(y)))
            z := c
            rad, grn, blu := 0.0, 0.0, 0.0
            for i := 0; i < iterations; i++ {
                z = z*z + c*j + je*julia 
                if real(z)*real(z) + imag(z)*imag(z) > 4 {
                    // itboard[y*width + x] = i
                    // grn = itmap(i)
                    // grn = 256*math.Log(float64(i))/math.Log(float64(iterations-1))
                    // grn = 256*math.Sqrt(float64(i)/float64(iterations))
                    rad, grn, blu = colSch2(i, iterations)
                    break
                }
            }
            set(x, y, round(rad), round(grn), round(blu))
        }
        if y % round(float64(height)/100) == 0 { // progress indicator
            fmt.Printf("y %v done\n", float64(y)*100/float64(height))
        }
    }
    dumpImg(img)
}

func colSch1(i, iterations int) (float64, float64, float64) {
    var rad, grn, blu float64
    n := 69.0
    dov := round(float64(iterations)/n)
    dovmap := mapRange(0, float64(dov), 0, math.Pi/2)
    it := float64(i)
    it = dovmap(it)
    // it = math.Log(it)
    // it = math.Exp(-it)
    rad = 255*math.Cos(it-math.Pi*(0.5 + 0.166666667))
    grn = 255*math.Cos(it)
    blu = 255*math.Cos(it+math.Pi*(0.5 + 0.166666667))
    // rad, grn, blu = absVal(rad), absVal(grn), absVal(blu)
    if rad < 0 {rad = 0}
    if grn < 0 {grn = 0}
    if blu < 0 {blu = 0}
    shhh(dovmap)
    return rad, grn, blu
}

func colSch2(i, iterations int) (float64, float64, float64) {
    var rad, grn, blu float64
    n := 40.0
    dov := round(float64(iterations)/n)
    dovby3 := round(float64(iterations)/(n*3))
    dovmap := mapRange(0, float64(dov), 0, 1)
    it := float64(i%dov)
    it = dovmap(it)
    grn = 255*(1/(1+math.Exp(-3*it))-0.5)
    blu = 255*(1/(1+math.Exp(-3*dovmap(float64((i+dovby3)%dov))))-0.5)
    rad = 255*(1/(1+math.Exp(-3*dovmap(float64((i+dovby3*2)%dov))))-0.5)
    // rad, grn, blu = absVal(rad), absVal(grn), absVal(blu)
    if rad < 0 {rad = 0}
    if grn < 0 {grn = 0}
    if blu < 0 {blu = 0}
    shhh(dovmap)
    return rad, grn, blu
}
