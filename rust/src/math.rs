
pub fn xyrange(pow: f64, x: f64, y: f64) -> (f64, f64, f64, f64) {
    let nudge = 2.0_f64.powf(pow-1.0);
    (x-nudge, x+nudge, y+nudge, y-nudge)
}

#[derive(Clone)]
pub struct MapRange {
    scale: f64,
    toff: f64,
    foff: f64,
}
impl MapRange {
    /// gives a struct. use struct.map(num) to map num
    pub fn new(fs: f64, fe: f64, ts: f64, te: f64) -> MapRange {
        MapRange {
            scale: ((te-ts)/(fe-fs)),
            toff: (te+ts)/2.0,
            foff: (fe+fs)/2.0,
            }
    }
    
    #[inline(always)]
    pub fn map(&self, num: f64) -> f64 {
        (num-self.foff)*self.scale + self.toff
    }
    
}

#[derive(Clone)]
pub struct ChopRange { // generic version of thihs would be nice
    start: f64,
    end: f64,
}
impl ChopRange {
    pub fn new (start: f64, end: f64) -> ChopRange {
        ChopRange {
            start,
            end,
        }
    }

    #[inline(always)]
    pub fn map(&self, num: f64) -> f64 {
        if num < self.start {self.start} else if num > self.end {self.end} else {num}
    }
}

/// outputs a f64 in range [0, m) if m > 0, and (m, 0] if m < 0
/// the m > 0 case is chosen cuz of the remainder like behaviour
/// and m < 0 case is just the extension of the above case as i couldnt find any info on how it should be
/// note: (maybe) different difinitions could have been chosen for this
///       for eg, instead of floor(), we could round it to an integer closer to 0, etc
pub fn fmod(f: f64, m: f64) -> f64 {
    f-(f/m).floor()*m
}