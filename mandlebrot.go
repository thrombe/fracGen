package main
import (
	"math"
	"fmt"
    "math/rand"
)

func mandlebrot() {
    width := 2000
    height := 2000
    
    iterations := 1000
    xfrom, xto, yfrom, yto := xyrange(-6, -0.74571890570893210, -0.11624642707064532)
    julia := complex(0, 0) // 1, 0 is on
    je := 0.25 + 0.0i
    
    var j complex128
    if julia == complex(1, 0) {j = 0} else {j = 1}
    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
    pixwidth := (xto-xfrom)/float64(width)

    samples := 1
    f := func(c complex128) (float64, float64, float64) {
        var r, g, b float64
        for s := 0; s < samples; s++ {
            z := c
            rc := complex((rand.Float64()-0.5)*pixwidth, (rand.Float64()-0.5)*pixwidth)
            z += rc
            for i := 0; i < iterations; i++ {
                z = z*z + c*j + je*julia
                if real(z)*real(z) + imag(z)*imag(z) > 4 {
                    rad, grn, blu := colSch1(i, iterations)
                    r += rad
                    g += grn
                    b += blu
                    break
                }
            }
        }
        return r/float64(samples), g/float64(samples), b/float64(samples)
    }

    img, set := newImg(width, height)
    for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            c := complex(xmap(float64(x)), ymap(float64(y)))
            rad, grn, blu := f(c)
            set(x, y, round(rad), round(grn), round(blu))
        }
        if y % round(float64(height)/100) == 0 { // progress indicator
            fmt.Printf("%v percent done\n", float64(y)*100/float64(height))
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
    // grn = 256*math.Log(float64(i))/math.Log(float64(iterations-1))
    // grn = 256*math.Sqrt(float64(i)/float64(iterations))
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
    return rad, grn, blu
}
