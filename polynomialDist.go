package main
import (
	"math"
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
			board[y*width + x] =num
			if num > max {max = num}
        }
    }

	colmap := mapRange(-max, 0, 0, 255) // setting board accouding to the color map
	for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
			val := absVal(board[y*width + x])
            grn = colmap(-log(val)) // glowing curve + thiccccc black border
			// grn = -log(val/max)*256 // stripey
			// grn = -math.Sqrt(val/max)*255 // bit dimmer glow
			// grn = val*255 // 
            set(x, y, int(rad), int(grn), int(blu))
		}
	}

    dumpImg(img)
}

func log(x float64) float64 { // just to ignore the import screams
	return math.Log(x)
}