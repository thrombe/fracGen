package main
import (
)

func polynomialDist() {
    width := 2000
    height := 2000
    img, set := newImg(width, height)
    
    xfrom, xto := 0.0, 4.0
    yfrom, yto := 2.0, -2.0
	
    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
	board := make([]float64, width*height)

	var max float64
    for y := 0; y < height; y++ { // calculating s for every pixel
        for x := 0; x < width; x++ {
			ex, wae := xmap(float64(x)), ymap(float64(y))
			num := (ex-1)*(ex-2)*(ex-3) - wae
			board[y*width + x] = num
			if num > max {max = num}
        }
    }

	colmap := mapRange(-max, 0, 0, 255) // setting board accouding to the color map
	for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
            grn = colmap(-absVal(board[y*width + height]))
            set(x, y, int(rad), int(grn), int(blu))
		}
	}

    dumpImg(img)
}
