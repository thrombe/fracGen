
pub fn xyrange(pow: f64, x: f64, y: f64) -> (f64, f64, f64, f64) {
    let nudge = 2.0_f64.powf(pow-1.0);
    (x-nudge, x+nudge, y+nudge, y-nudge)
}

pub struct MapRange {
    scale: f64,
    toff: f64,
    foff: f64,
}

// gives a struct. use struct.map(num) to map num
pub fn map_range(fs: f64, fe: f64, ts: f64, te: f64) -> MapRange {
    MapRange {
        scale: ((te-ts)/(fe-fs)),
        toff: (te+ts)/2.0,
        foff: (fe+fs)/2.0,
        }
}

impl MapRange {

    #[inline(always)]
    pub fn map(&self, num: f64) -> f64 {
        (num-self.foff)*self.scale + self.toff
    }

}