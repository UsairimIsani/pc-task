#[derive(Debug)]
pub struct ThreeArr {
    data: Vec<f32>,
    strides: Vec<usize>,
    dim: u32,
}

impl ThreeArr {
    pub fn new(dim: u32, length: usize) -> Self {
        let strides = (0..dim).fold(Vec::new(), |mut state, i| {
            state.push(length.pow(i));
            state
        });

        Self {
            data: vec![0.0; length.pow(dim)],
            strides,
            dim,
        }
    }

    pub fn get(&self, axis: Vec<usize>) -> Option<&f32> {
        let index = axis
            .iter()
            .zip(self.strides.iter())
            .fold(0, |mut ind, (a, b)| {
                ind += *a * *b;
                ind
            });

        self.data.get(index)
    }

    pub fn transpose(&mut self, axes: Vec<usize>) {
        self.strides = axes.iter().fold(vec![], |mut state, axis| {
            state.push(self.strides[*axis]);
            state
        });
    }

    pub fn swap_axis(&mut self, a_axis: usize, b_axis: usize) {
        self.strides.swap(a_axis, b_axis);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_naive_impl() {
        let mut darr = vec![vec![vec![0; 2]; 2]; 2];

        darr[0][0][0] = 1;
        darr[0][0][1] = 2;
        darr[0][1][0] = 3;
        darr[0][1][1] = 4;
        darr[1][0][0] = 5;
        darr[1][0][1] = 6;
        darr[1][1][0] = 7;
        darr[1][1][1] = 8;

        println!("{:#?}", darr[0][1][0]);

        let mut new_darr = vec![];

        for x_val in darr.into_iter() {
            let mut b: Vec<Vec<i32>> = vec![vec![]; 2];
            for y in x_val {
                for (z_index, z_val) in y.into_iter().enumerate().rev() {
                    b[z_index].push(z_val);
                }
            }
            new_darr.push(b);
        }

        println!("{:#?}", new_darr[0][1][0]);
    }

    #[test]
    fn test_data_structure() {
        let mut a = ThreeArr::new(3, 2);
        println!("{:#?}", a.strides);
        for i in 1..=(2 as u32).pow(3) as usize {
            a.data[i - 1] = i as f32;
        }
        println!("{:#?}", a.data);

        let b = a.get(vec![0, 1, 0]);

        println!("{:#?}", b);
        a.transpose(vec![2, 0, 1]);

        println!("{:#?}", a.data);
        println!("{:#?}", a.strides);
        let b = a.get(vec![0, 0, 1]);
        println!("{:#?}", b);
    }

    #[test]
    fn test_swap_axis() {
        let mut a = ThreeArr::new(3, 2);
        println!("{:#?}", a.strides);
        for i in 0..(2 as u32).pow(3) as usize {
            a.data[i] = i as f32;
        }
        println!("{:#?}", a.data);

        let b = a.get(vec![0, 1, 0]);
        println!("{:#?}", b);

        a.swap_axis(1, 2);

        println!("{:#?}", a.data);
        println!("{:#?}", a.strides);

        let b = a.get(vec![0, 1, 0]);
        println!("{:#?}", b);
    }

    #[test]
    fn test_swap_axis_60() {
        let dim = 3;
        let length = 60;
        let mut a = ThreeArr::new(dim, length);
        println!("{:#?}", a.strides);
        for i in 0..(length as u32).pow(dim) as usize {
            a.data[i] = i as f32;
        }

        let b = a.get(vec![0, 1, 0]);
        println!("{:#?}", b);

        a.swap_axis(1, 2);
        println!("{:#?}", a.strides);

        let b = a.get(vec![0, 1, 0]);
        println!("{:#?}", b);
    }
}
