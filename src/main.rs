use yew::prelude::*;
use rand::Rng;

enum Msg {
    RenderGrid
}

struct Cells {
    grid: Vec<Vec<i32>>,
}

impl Cells {
    fn init(cols: usize, rows: usize) -> Cells {
        let mut grid = Cells::new_grid(cols, rows);
        for i in 0..cols {
            for j in 0..rows {
                grid[i][j] = rand::thread_rng().gen_range(0..2);
            }
        }
        Cells {
            grid,
        }
    }

    fn new_grid(cols: usize, rows: usize) -> Vec<Vec<i32>> {
       let mut matrix: Vec<Vec<i32>> = Vec::new();
       matrix.resize(cols, vec![]);
       for i in 0..matrix.len() {
           matrix[i] = vec![0; rows]; 
       }
       matrix
    }
}

impl Component for Cells {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Cells::init(10, 10)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RenderGrid => {
                self.play();
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <div class="grid">
                <p>{ 0 }</p>
            </div>
        )
    }
}

fn main() {
    let cells = Cells::init(10, 10); 
    println!("{:#?}", cells.grid);
}
