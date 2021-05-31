package main

import (
	"fmt"
	"math"
	"math/rand"
)
// needs img.go

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
            if col < 256 {grn = col} else if col < 512 {grn, rad = 255, col-255} else if col < 768 {grn, rad, blu = 255, 255, col-511} else {grn, rad, blu = 255, 255, 255}
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


func polynomialDist() {
    width := 2000
    height := 2000
    img, set := newImg(width, height)
    
    xfrom, xto := 0.0, 4.0
    yfrom, yto := 2.0, -2.0
    xmap := mapRange(0, float64(width), xfrom, xto)
    ymap := mapRange(0, float64(height), yfrom, yto)
    colmap := mapRange(-4, 0, 0, 255)

    for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
            ex, wae := xmap(float64(x)), ymap(float64(y))
            grn = colmap(-absVal((ex-1)*(ex-2)*(ex-3) - wae))
            set(x, y, int(rad), int(grn), int(blu))
        }
        if y % height/100 == 0 { // progress indicator
            fmt.Printf("y %v done\n", y+1)
        }
    }
    dumpImg(img)
}

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

// regular polygon chaos fractal with random point choosing (jump ratio in decimal plz)
// choose from these rules- 1) same vertex cant be chosen twice
func chaosFrac(sides int, jumpRatio float64, iterations int, imgSize int, rules ...int) { // chaos game (look on wiki)
    // we take 3 points (far apart). take another random point. guess a random no. from [0, 3) (integer)
    // if 0, we jump midway the chosen random point and 1st point of the 3. similar for 1 and 2
    // and magically appears sirpinski's triangle
    // also works for more points than 3
    width := imgSize
    height := imgSize
    img, set := newImg(width, height)
    
    cursor := vector(float64(width)/2, float64(height)/2)

    for y := 0; y < height; y++ { // setting the background pixels
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
            set(x, y, int(rad), int(grn), int(blu))
        }
    }
    
    var lastVer int
    
    vertices := make([][][]float64, sides) // generating the polygon
    center := vector(float64(width)/2, float64(height)/2)
    vertices[0] = vector(float64(width)/2, 0)
    rot := rotMat(2*math.Pi/float64(sides))
    for i := 1; i < sides; i++ {
        vertices[i] = matAdd(matMul(rot, matSub(vertices[i-1], center)), center)
    }
    for i := 0; i < iterations; i++ {
        ver := rand.Intn(sides)
        if rules[0] == 1 && lastVer == ver {
            iterations += 1
            continue
        }
        lastVer = ver
        cursor = matAdd(matScalar(cursor, 1-jumpRatio), matScalar(vertices[ver], jumpRatio))
    
        cholors := vector(0, 0, 0)
        // cholors[ver%3][0] = 255
        cholors[1][0] = 255
        set(int(cursor[0][0]), int(cursor[1][0]), int(cholors[0][0]), int(cholors[1][0]), int(cholors[2][0]))
    }
    dumpImg(img)
}

func sirpinskiTri() {
    chaosFrac(3, 0.5, 1000000, 2000, 0)
}

func demonFrac() {
    chaosFrac(5, 0.5, 1000000, 2000, 1)
}