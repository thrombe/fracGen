package main

import (
    "fmt"
    "time"
    // "math"
    // "github.com/pkg/profile"
)

func shhh(vals ...interface{}) {
    for _, val := range vals {
        _ = val
    }
}

func main() {
  	// defer profile.Start(profile.MemProfile).Stop()
	// defer profile.Start().Stop()

    start := time.Now()
    defer func() {fmt.Println(time.Now().Sub(start))}()
    mandlebrot()
    // chaosFrac(5, 0.5, 5000000, 10000, 1)
	// genImg()
	// plotEq()
    // buddhabrot()
    // findsols(0.25, -5, 5, 1000)
}

func genImg() {
    width := 1000
    height := 1000
    img, set := newImg(width, height)
    // Set color for each pixel.
    var t float64 = 1
    dt := 1/float64(height)
    for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            rad := 229*(1-t) + t*148
            grn := 240*(1-t) + t*191
            blu := 255*(1-t) + t*255
            // rad = int(math.Round(mapp(float64(rad))))
            // grn = int(math.Round(mapp(float64(grn))))
            // blu = int(math.Round(mapp(float64(blu))))
            set(x, y, int(rad), int(grn), int(blu))
        }
        t -= dt
    }
    dumpImg(img)
}