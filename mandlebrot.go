package main
import (
	"math"
	"fmt"
    "math/rand"
)

func mandlebrot() {
    width := 2000
    height := 2000
    
    samples := 4 // random samples per pixel
    iterations := 1000 // max iterations per sample
    xfrom, xto, yfrom, yto := xyrange(-6, -0.74571890570893210, -0.11624642707064532)
    // j := 0.25 + 0.0i
    eq := func(c, z complex128) complex128 {return z*z + c}
    out := make(chan *pixel, 50) // store 50 calculated trajectories
    workers := make(chan struct{}, 16) // no of goroutines calculating trajectories in parallel
    
    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
    pixwidth := (xto-xfrom)/float64(width)

    // there should be no need to change the two following functions
    singlesample := func() {
        img, set := newImg(width, height)
        for y := 0; y < height; y++ {
            for x := 0; x < width; x++ {
                c := complex(xmap(float64(x)), ymap(float64(y)))
                z := c
                rad, grn, blu := 0.0, 0.0, 0.0
                for i := 0; i < iterations; i++ {
                    z = eq(c, z)
                    if real(z)*real(z) + imag(z)*imag(z) > 4 {
                            rad, grn, blu = colSch1(i, iterations)
                        break
                    }
                }
                set(x, y, round(rad), round(grn), round(blu))
            }
            if y % round(float64(height)/100) == 0 { // progress indicator
                fmt.Printf("%v done\n", float64(y)*100/float64(height))
            }
        }
        dumpImg(img)    
    }
    multisample := func() { // this is slow when doing just 1 sample per pixel
        getpix := func(x, y int, out chan *pixel, workers chan struct{}) {
            c := complex(xmap(float64(x)), ymap(float64(y)))
            p := pix(x, y, 0, 0, 0)
            for s := 0; s < samples; s++ {
                z := c
                rc := complex((rand.Float64()-0.5)*pixwidth, (rand.Float64()-0.5)*pixwidth)
                z += rc
                for i := 0; i < iterations; i++ {
                    z = eq(c, z)
                    if real(z)*real(z) + imag(z)*imag(z) > 4 {
                        rad, grn, blu := colSch1(i, iterations)
                        p.r += rad
                        p.g += grn
                        p.b += blu
                        break
                    }
                }
            }
            p.scalecolor(1/float64(samples))
            out <- p
            <- workers
        }

        go func(out chan *pixel, workers chan struct{}) { // creating more workers as needed
            for y := 0; y < height; y++ {
                for x := 0; x < width; x++ {
                    workers <- struct{}{}
                    go getpix(x, y, out, workers)
                }
            }
        }(out, workers)

        img, set := newImg(width, height)
        lim := width*height
        mod := round(float64(lim)/100)
        for i := 0; i < lim; i++ { // filling image
            p := <- out
            set(p.x, p.y, round(p.r), round(p.g), round(p.b))
            if i % mod == 0 { // progress indicator
                fmt.Printf("%v percent done\n", float64(i)*100/float64(lim))
            }
        }
        dumpImg(img)
    }
    if samples == 1 {singlesample()} else {multisample()}
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
