mod utils;
mod data;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct TriangleSolver{
    unsolved:String,
    solved:Vec<i32>,
    count:i32,
}

#[wasm_bindgen]
impl TriangleSolver{

    //Gets an emtpy struct
    pub fn new()->TriangleSolver{
        TriangleSolver{
            unsolved:data::get_triangle_data(),
            solved:Vec::new(),
            count:0,
        }
    }

    //Read dataset

    pub fn count_triangles_horizontally(&mut self) -> i32{
        let mut count = 0;
        for sides_list in self.unsolved.lines(){
            let sides:Vec<i32> = sides_list
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect();
            let sides_cloned = sides.clone();

            if self.is_valid_triangle(sides) == true{
                self.solved.extend_from_slice(&sides_cloned.as_slice()[0..]);
                self.count+=1;
                count+=1;
            };
        }
        count
    }

    pub fn get_solved(&self) -> *const i32{
        self.solved.as_ptr()
    }

    pub fn get_size(&self) -> usize{
        self.solved.len()
    }

    fn is_valid_triangle(&self,sides:Vec<i32>)->bool{
        for i in 0..sides.len(){
            let l=&sides[..i];
            let r=&sides[i+1..];
            let merged = [l,r].concat();
    
            if sides[i] >= merged.iter().sum(){
                return false;
            }
        }
        return true
    }
}