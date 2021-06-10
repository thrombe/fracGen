package main

import (
	"fmt"
	"math"
)

func equation() func(float64, float64) float64 {
    return func(x, y float64) float64 {
        // val := (x-1)*(x-2)*(x-3)-y
  		// val := math.Tan(x*x+y*y) - 1
		val := math.Cos(x*x+y*y)-0.5*x*y
		// val := math.Sin(x*x+y*y)-math.Cos(x*y)
		// val := math.Sin(math.Exp(-20*x)) - y
        // val := x*x+y*y-5
        return absVal(val)
    }
}

func plotEq() {
    width := 2000
    height := 2000
    xfrom, xto, yfrom, yto := xyrange(4, 0, 0)

    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
	board := make([]float64, width*height)
	s := equation()

	var max float64
    for y := 0; y < height; y++ { // calculating s for every pixel
        for x := 0; x < width; x++ {
			ex, wae := xmap(float64(x)), ymap(float64(y))
			num := s(ex, wae)
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
            grn = colmap(math.Log(val)) // glowing curve + thiccccc black border
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
    accuracy := 0.00000000000001
    searchdepth := 1000
    var sols []float64
    var trigger bool // this bool is used as a one time trigger for each possible solution
    
    s := equation()
    var prev, curr float64
    prev = s(x, ys-dy)
    for y := ys; y <= ye; y += dy {
        curr = s(x, y)
        if curr < accuracy {sols = append(sols, y)} // check if guess is correct
        if !trigger && curr < prev {trigger = true} // there is possibly a solution ahead cuz trend is decreasing s()
        if trigger && curr > prev { // if the trend changed from decreasing s() to increasing, we passed a solution
            preprev := s(x, y-dy*2)
            var ycurr float64 // used to keep track of the jumping y value while searching for solution
            dycurr := dy // same as ycurr
            if preprev < curr {
                prev, curr, ycurr = preprev, prev, y-dy
            } else {
                ycurr = y
            }
            for i := 0; i < searchdepth; i++ { // jumping around to find the solution
                dycurr = dycurr/2
                ycurr -= dycurr
                mid := s(x, ycurr)
                // fmt.Println(ycurr, dycurr, curr, mid, prev) //////////
                if mid < accuracy { // check if new guess is correct enough
                    sols = append(sols, ycurr)
                    break
                }
                if prev < curr { // deciding if solution is above the guess or below
                    curr = mid
                    if dycurr < 0 {dycurr = -dycurr}
                } else {
                    prev = mid
                    if dycurr > 0 {dycurr = -dycurr}
                }
            }
            curr = s(x, y) // resetting curr for next loop
            trigger = false
        }
        prev = curr
    }
    fmt.Println(sols)
    return sols
}
