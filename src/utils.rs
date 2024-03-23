use esp32c3_hal::Rtc;

#[derive(Copy, Clone, Debug)]
pub enum Bit {
    High,
    Low,
}

fn reverse_bits(mut num: u8) -> u8 {
    let mut result = 0;
    for _ in 0..8 {
        result = (result << 1) | (num & 1);
        num >>= 1;
    }
    result
}

pub fn to_hex(matrix: &[[Bit; 8]; 8]) -> [u8; 8] {
    let mut result: [u8; 8] = [0x00; 8];
    for i in 0..8 {
        let mut value: u8 = 0;
        for j in 0..8 {
            value = (value << 1)
                + match matrix[i][j] {
                    Bit::High => 1,
                    Bit::Low => 0,
                }
        }
        result[i] = reverse_bits(value);
    }
    result
}

// pub fn test() {
//     let mut matrix: [[Bit; 8]; 8] = [[Bit::Low; 8]; 8];
//     // println!("{:?}", &matrix);
//     matrix[3][3] = Bit::High;
//     matrix[0][7] = Bit::High;
//     let val = to_hex(&matrix);
//     // println!("{:?}", val);
// }

// pub struct Random<'a> {
//     rtc:&'a Rtc<'static>
// }

// impl  Random<'a> {
//     pub fn new(rtc: &Rtc<'static>)->Self{
//         Random{
//             rtc
//         }
//     }
// }

pub fn find_emtpy_count(matrix:&[[Bit; 8]; 8])->u16{
    let mut count = 0;
    for row in matrix {
            for cell in row{
                if let Bit::Low = cell {
                    count+=1;
                }
              
            }
    }
    count
}

pub fn find_emtpy_offset(matrix:&[[Bit; 8]; 8])->(usize,usize){
    for x in 0..8{
        for y in 0..8{
            if let Bit::Low = matrix[x][y] {
                return (x,y);
            }
        }
    }
    (0,0)
}

pub struct Random<'a> {
    rtc: &'a Rtc<'static>,
}

impl<'a> Random<'a> {
    pub fn new(rtc: &'a Rtc<'static>) -> Self {
        Random {
            rtc,
        }
    }
    // 伪随机
    pub fn get_rand(&self,num:u64)->u64{
        if self.rtc.get_time_us()%2==0{
            (self.rtc.get_time_us()+self.rtc.get_time_ms()-num) %num 
        }else if self.rtc.get_time_us()%3==0{
            (self.rtc.get_time_us()+ num +self.rtc.get_time_ms()*num) %num 
        }else{
            (self.rtc.get_time_raw()*num+self.rtc.get_time_ms()+num) %num 
        }
    }

}