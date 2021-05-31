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
    xfrom, xto := -2.0, 1.0
    yfrom, yto := 1.5, -1.5
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
                    rad = 0
                    // grn = itmap(i)
                    // grn = 256*math.Log(float64(i))/math.Log(float64(iterations-1))
                    grn = 256*math.Sqrt(float64(i)/float64(iterations))
                    blu = 0
                    break
                }
            }
            set(x, y, int(rad), int(grn), int(blu))
        }
        if y % height/100 == 0 { // progress indicator
            fmt.Printf("y %v done\n", y+1)
        }
    }
    dumpImg(img)
}
