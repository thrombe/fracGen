package main
import (
    "image/color"
    "image/png"
    "os"
    "image"
    "fmt"
)

// image output stuff

func newImg(width, height int) (*image.RGBA, func(int, int, int, int, int)) {
    img := image.NewRGBA(image.Rectangle{image.Point{0, 0}, image.Point{width, height}})
    return img, func(x, y, r, g, b int) {
        // img.Set(x, y, color.RGBA{uint8(rad % 256), uint8(grn % 256), uint8(blu % 256), 0xff})
        img.Set(x, y, color.RGBA{uint8(r % 256), uint8(g % 256), uint8(b % 256), 0xff})
    }
}

func dumpImg(img *image.RGBA) {
    f, err := os.Create(fileName())
    if err != nil {panic(err)}
    png.Encode(f, img) // encode as png
}

func fileName() string {
    for i := 0;; i++ {
        name := fmt.Sprintf("/root/0Git/fracGen/images/image%v.png", i)
        _, err := os.Stat(name)
        if err != nil {return name}
    }
}

// pixel struct

type pixel struct {
    r, g, b float64
    x, y int
}

func (p *pixel) scalecolor(t float64) {
    p.r *= t
    p.g *= t
    p.b *= t
}

func pix(x, y int, r, g, b float64) *pixel {
    p := pixel{}
    p.x, p.y = x, y
    p.r, p.g, p.b = r, g, b
    return &p
}