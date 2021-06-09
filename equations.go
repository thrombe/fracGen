package main

import (
	"fmt"
	"math"
)

func plotEq() {
    width := 2000
    height := 2000
    
    // xfrom, xto := 0.0, 4.0
    // yfrom, yto := 2.0, -2.0
    xfrom, xto, yfrom, yto := xyrange(4, 0, 0)

    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
	board := make([]float64, width*height)

	var max float64
    for y := 0; y < height; y++ { // calculating s for every pixel
        for x := 0; x < width; x++ {
			ex, wae := xmap(float64(x)), ymap(float64(y))
			// num := (ex-1)*(ex-2)*(ex-3) - wae
			// num := math.Tan(ex*ex+wae*wae) - 1
			num := math.Cos(ex*ex+wae*wae)-0.5*ex*wae
			// num := math.Sin(ex*ex+wae*wae)-math.Cos(ex*wae)
			// num := math.Sin(math.Exp(-20*ex)) - wae
			board[y*width + x] = absVal(num)
			if num > max {max = num}
        }
    }

	img, set := newImg(width, height)
	colmap := mapRange(max, 0, 0, 255) // setting board accouding to the color map
	for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
			val := board[y*width + x]
            grn = colmap(log(val)) // glowing curve + thiccccc black border
			if grn < 256 {grn = 0} else if grn > 511 {grn = 255} // eliminating the black sudden black to green change in last one
			// grn = -log(val/max)*256 // stripey
			// grn = -math.Sqrt(val/max)*255 // bit dimmer glow
			// grn = val*255 // 
            set(x, y, int(rad), int(grn), int(blu))
		}
	}

    dumpImg(img)
}

func findsols(x, ys, ye float64, height int) []float64 {
    dy := (ye-ys)/float64(height)
    s := equation()
    var sols []float64
    prev := s(x, ys-dy)
    var dec bool // true if trend is decreasing value of s(x, y)
    for y := ys; y <= ye; y += dy {
        curr := s(x, y)
        if curr < 0.00000000000001 {sols = append(sols, y)}
        if !dec && curr < prev {dec = true} // else if dec && curr > prev {dec = false}
        if dec && curr > prev { // once increasing, always triggers. fix plz
            preprev := s(x, y-dy*2)
            var ycurr float64
            if preprev < curr {prev, curr, ycurr = preprev, prev, y-dy} else {ycurr = y}
            dycurr := dy
            for i := 0; i < 100; i++ {
                dycurr = dycurr/2
                ycurr -= dycurr
                mid := s(x, ycurr)
                // fmt.Println(ycurr, dycurr, curr, mid, prev) //////////
                if mid < 0.00000000000001 {
                    sols = append(sols, ycurr)
                    curr = s(x, y)
                    break
                }
                if prev < curr {
                    curr = mid
                    if dycurr < 0 {dycurr = -dycurr}
                } else {
                    prev = mid
                    if dycurr > 0 {dycurr = -dycurr}
                }
            }
            curr = s(x, y)
            // return sols // debug
            dec = false
        }
        prev = curr
    }
    return sols
}

func equation() func(float64, float64) float64 {
    return func(x, y float64) float64 {
        // return absVal((x-1)*(x-2)*(x-3)-y)
        return absVal(x*x+y*y-5)
    }
}

func log(x float64) float64 { // just to ignore the import screams
    _ = fmt.Fprint
	return math.Log(x)
}
