package main

import (
	"math"
	"math/rand"
)

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
    
    for y := 0; y < height; y++ { // setting the background pixels
        for x := 0; x < width; x++ {
            rad, grn, blu := 0.0, 0.0, 0.0
            set(x, y, int(rad), int(grn), int(blu))
        }
    }
    
    var lastVer int // keeping track of the previously chosen vertex
    
    vertices := make([]*vec4d, sides) // generating the polygon
    center := vector2d(float64(width)/2, float64(height)/2)
    vertices[0] = vector2d(float64(width)/2, 0)
    rot := rotMat(2*math.Pi/float64(sides))
    for i := 1; i < sides; i++ {
        vertices[i] = vecsub(vertices[i-1], center)
        vertices[i].transform2d(rot)
        vertices[i].add(center)
    }

    cursor := vector2d(float64(width)/2, float64(height)/2)
    t := 1-jumpRatio
    // bgcolor := vector3d(0, 0, 0)
    plotcolor := vector3d(0, 255, 0)
    for i := 0; i < iterations; i++ {
        ver := rand.Intn(sides)
        if rules[0] == 1 && lastVer == ver {
            iterations += 1
            continue
        }
        lastVer = ver
        cursor.lerp(vertices[ver], t)
    
        // cholors := vector3d(0, 0, 0)
        // cholors[ver%3][0] = 255
        // cholors.y = 255
        set(int(cursor.x), int(cursor.y), int(plotcolor.x), int(plotcolor.y), int(plotcolor.z))
    }
    dumpImg(img)
}

func sirpinskiTri() {
    chaosFrac(3, 0.5, 1000000, 2000, 0)
}

func demonFrac() {
    chaosFrac(5, 0.5, 1000000, 2000, 1)
}