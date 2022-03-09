#[derive(Debug)]

pub struct Details {
    pub name: String,
    pub m1:usize,
    pub m2:usize,
    pub m3:usize
   
}

impl Details {
    pub fn new(param1:String, param2: usize  ) ->Self {
        Self{
            name:param1,
            m1:param2,
            m2:90,
            m3:66
        }
    }
    pub fn total(&self) -> usize {
        let add = self.m1+self.m2+self.m3;
        add
    }
    pub fn compare(self, cmp: Details) -> (String,usize) {
        if self.m2 > cmp.m2 {
            (self.name,self.m2)
        } else {
            (cmp.name,cmp.m2)
        }
    }

}