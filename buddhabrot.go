package main
import (
	"math/rand"
)

func buddhabrot() {
    // the bright green one - 1000*1000 img, 20 iterations, 10^8 trajectories 5mins no color mapping
    // the high quality ones - 5k*5k image, 20000 iterations, 10^8 trajectories no color mapping 1h 0min
    width := 1000
    height := 1000
    
    iterations := 20
    trajectories := 100_000_000
    xfrom, xto := -2.0, 2.0
    yfrom, yto := 2.0, -2.0
    
    board := make([]int, width*height)
    xmap := mapRange(xfrom, xto, 0, float64(width))
    ymap := mapRange(yfrom, yto, 0, float64(height))
    
    out := make(chan []int, 50) // store 50 calculated trajectories
    workers := make(chan struct{}, 15) // no of goroutines calculating trajectories in parallel
    go func() { // creating more workers as needed in parallel
        for i := 0; i < trajectories; i++ {
            workers <- struct{}{}
            go validTrajectory(iterations, width, height, xmap, ymap, out, workers)
        }
    }()
    
    // it wont cost too much to make more images at different qualities from the stuff calculated above.
    // so just add more boards here and create images from them later.
    // maybe make it automated. so like a function that takes in the input of qualities of pixels and outputs images
    for i := 0; i < trajectories; i++ { // marking trajectories on board
        for _, k := range <- out {
            board[k] += 1
        }
    }

    // var max int
    // for _, col := range board { // finding the max brightness so as to map it in 0 to 256
    //     if max < col {max = col}
    // }
    // colmap := mapRange(0, float64(max), 0, 256)
    
    // img, set := newImg(width, height)
    // for y := 0; y < height; y++ { // creating image from board
    //     for x := 0; x < width; x++ {
    //         rad, grn, blu := 0, 0, 0
    //         grn = round(colmap(float64(board[y*width + x])))
    //         set(x, y, rad, grn, blu)
    //     }
    // }
    // dumpImg(img)

    // // images without colormap are usually very dark when thr pixel to trajectory ratio is low.
    // // for eg: 10^8 trajectories and 1000*1000 is the only one(that i found) that looks better this way
    // img2, set2 := newImg(width, height) // image without color mapping i.e. with %256
    // for y := 0; y < height; y++ { // creating image from board
    //     for x := 0; x < width; x++ {
    //         rad, grn, blu := 0, 0, 0
    //         grn = board[y*width + x]
    //         set2(x, y, rad, grn, blu)
    //     }
    // }
    // dumpImg(img2)

    img3, set3 := newImg(width, height) // image without color mapping i.e. with %256
    for y := 0; y < height; y++ { // creating image from board
        for x := 0; x < width; x++ {
            rad, grn, blu := 0, 0, 0
            col := board[y*width + x]
            // if col < 256 {grn = col} else if col < 512 {grn, blu = 255, col-255} else if col < 768 {grn, blu, rad = 255, 255, col-511} else {grn, blu, rad = 255, 255, 255}
            // if col < 256 {grn = col} else if col < 512 {grn, rad = 255, col-255} else if col < 768 {grn, rad, blu = 255, 255, col-511} else {grn, rad, blu = 255, 255, 255}
            if col < 256 {grn = col} else if col < 512 {grn, rad = 180, col-255} else if col < 768 {grn, rad, blu = 180, 140, col-511} else {grn, rad, blu = 255, 255, 255}
            set3(x, y, rad, grn, blu)
        }
    }
    dumpImg(img3)
}

// helper for buddhabrot
func validTrajectory(iterations, width, height int, xmap, ymap func(float64) float64, out chan []int, workers chan struct{}) {
    var traj []int
    z := complex((rand.Float64()*4-2), (rand.Float64()*4-2)) // a random complex no. in the mandlebrot range (shouldve been a circle but whatever)
    c := z
    var index int
    for i := 0; i < iterations; i++ {
        index = round(xmap(real(z))) + round(ymap(imag(z)))*width
        if index > 0 && index < width*height-1 {traj = append(traj, index)} // rounding from complex to image coords
        if real(z)*real(z) + imag(z)*imag(z) > 4 {
            out <- traj//[: len(traj)-1] // cuz last one is out of the circle of radius 2 (but sometimes it kinda looks nice with those pixels lit tho)
            <- workers
            return
        }
        z = z*z + c
    }
    validTrajectory(iterations, width, height, xmap, ymap, out, workers) // if the random point was inside mandlebrot, then choose another
}
