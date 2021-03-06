package main

import (
	"fmt"
	"math"
	"math/rand"
)

func equation() func(float64, float64) float64 {
    return func(x, y float64) float64 {
        // val := (x-1)*(x-2)*(x-3)-y
  		// val := math.Tan(x*x+y*y) - 1
		// val := math.Cos(x*x+y*y)-0.5*x*y
		// val := math.Sin(x*x+y*y)-math.Cos(x*y)
		// val := math.Sin(math.Exp(-20*x)) - y
        // val := x*x+y*y-5
        // val := math.Cos(x*x+y*y)-x*y/4
        // val := math.Cos(x*x+y*y)-x*y/2+x*x+y*y-16 // too dark
        // val := math.Cos(x*x+y*y)-x*y/2-x*x-y*y+16 // too dark
        val := min(absVal(math.Cos(x*x+y*y)-x*y/4), absVal(math.Cos(x*x+y*y)-x*y/2+x*x+y*y-16), absVal(math.Cos(x*x+y*y)-x*y/2-x*x-y*y+16))
        return absVal(val)
    }
}

func plotEq() {
    width := 2000
    height := 2000
    xfrom, xto, yfrom, yto := xyrange(3.5, 0, 0)

    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
	board := make([]float64, width*height)
	s := equation()
	antialiasing := false

    if antialiasing { // anti-aliasing (only useful if the coloring is such that the lines are thin (maybe))
        se := equation()
        pixwidth := (xto-xfrom)/float64(width)
        tries := 50
        s = func(x, y float64) float64 {
            var val float64
            for i := 0; i < tries; i++ {
                val += se(x+(rand.Float64()-0.5)*pixwidth, y+(rand.Float64()-0.5)*pixwidth)
            }
            return val/float64(tries)
        }
    }

	var max float64
    for y := 0; y < height; y++ { // calculating s for every pixel
        for x := 0; x < width; x++ {
			ex, wae := xmap(float64(x)), ymap(float64(y))
			num := s(ex, wae)
			board[y*width + x] = absVal(num)
			if num > max {max = num}
        }
    }

    // setting up color stuff
	colmap := mapRange(max, 0, 0, 1)
    colfunc := func(x float64) float64 {
        val := colmap(math.Log(x)*4) // glowing curve + thiccccc black border
        if val <= 1 {val = 0} else if val < 2 {val -= 1} else if val >= 2 {val = 1} // eliminating the black sudden black to green change in last one
        // val := -log(val/max) // stripey
		// val := -math.Sqrt(val/max) // bit dimmer glow
        return val // must return values 0 <= val <= 1 cuz lerp
    }
    plotcolor := vector3d(0, 255, 0)
    bgcolor := vector3d(0, 0, 0)

    img, set := newImg(width, height) // setting board according to the color map
	for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            val := colfunc(board[y*width + x])
            color := veclerp(plotcolor, bgcolor, val)
            set(x, y, int(color.x), int(color.y), int(color.z))
		}
	}
    dumpImg(img)
}

/*
findsols explain:
    . ycurr, dycurr -> both these are used to store y and dy resp.
          so that we can change them and not worry about actual y and dy
    . curr and prev -> stores current and previous guess
    . sols -> stores found solutions in a list
    . searchdepth -> how many times should i oscillate about the solution (max)
    . accuracy -> how small should s() become for it to be considered a solution
          (since s() = 0 would be a perfect solution)
    . preprev -> stores the value of previous to prev (y-2*dy).
          it is used to decide if the solution lies between curr and prev or prev and preprev.
    . trigger -> this bool was added just to prevent the algo from checking till max search depth
          every time the value of s() increases from one guess to next.
          its set to true if the the guesses value of s() starts decreasing as we guess more values
    . ys and ye -> y start and y end. ie the y bounds
*/
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
