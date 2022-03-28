use yew::prelude::*;
use gloo_timers::callback::Interval;
use yew::html::Scope;
use rand::Rng;

enum Msg {
    Start,
    Stop,
    Step,
    Tick,
}

struct Page {
    running: bool,
    cells: Vec<Cell>,
    cell_rows: usize,
    cell_cols: usize,
    _interval: Interval,
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
struct Cell {
    state: State,
}

impl Cell {
    fn random(&mut self) {
    } 

    fn is_living(self) -> bool {
        self.state == State::Alive
    }

    fn new() -> Self {
        if rand::thread_rng().gen_bool(50.0) {
            Self { state: State::Alive }
        } else {
            Self { state: State::Dead }
        }
    }
    
    fn die(&mut self) {
        self.state = State::Dead;
    }

    fn to_life(&mut self) {
        self.state = State::Alive;
    }

    fn toggle(&mut self) {
        if self.is_living() {
            self.state = State::Dead;
        } else {
            self.state = State::Alive;
        }
    }

    fn check_neighbors(neighbors: &[Self]) -> usize {
        neighbors.iter().filter(|i| i.is_living()).count()
    } 

    fn overpopulated(neighbors: &[Self]) -> bool {
        Self::check_neighbors(neighbors) > 3
    }

    fn alone(neighbors: &[Self]) -> bool {
        Self::check_neighbors(neighbors) == 0
    }

    fn can_spawn(neighbors: &[Self]) -> bool {
        Self::check_neighbors(neighbors) == 3
    }
}

impl Page {
    fn render(&self, index: usize, cell: &Cell, _link: &Scope<Self>) -> Html {
        let cell_state = match cell.is_living() {
            true => "alive",
            _ => "dead",
        }; 
        html! {
            <div key={index} class={classes!("cell", cell_state)}></div>
        }
    }

    fn step(&mut self) {
        let mut to_die = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cell_cols {
            for col in 0..self.cell_rows {
                let neighbors = self.neighbors(row as isize, col as isize);
                let current_i = self.xy_index(row as isize, col as isize);

                if self.cells[current_i].is_living() {
                    if Cell::alone(&neighbors) || Cell::overpopulated(&neighbors) {
                        to_die.push(current_i);
                    }
                } else if Cell::can_spawn(&neighbors) {
                    to_live.push(current_i);
                }
            }
        }
        to_die
            .iter()
            .for_each(|i| self.cells[*i].die());
        to_live
            .iter()
            .for_each(|i| self.cells[*i].to_life());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cell; 8] {
        [
            self.cells[self.xy_index(row + 1, col)],
            self.cells[self.xy_index(row + 1, col + 1)],
            self.cells[self.xy_index(row + 1, col - 1)],
            self.cells[self.xy_index(row, col + 1)],
            self.cells[self.xy_index(row, col - 1)],
            self.cells[self.xy_index(row - 1, col)],
            self.cells[self.xy_index(row - 1, col + 1)],
            self.cells[self.xy_index(row - 1, col - 1)],
        ]
    }

    fn xy_index(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cell_cols as isize);
        let col = wrap(col, self.cell_rows as isize);
        row * self.cell_cols + col
    }
}

fn wrap(pos: isize, range: isize) -> usize {
    let res = if pos < 0 {
        pos + range
    } else if pos >= range {
        pos - range
    } else {
        pos
    };
    res as usize
}


impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let (cell_cols, cell_rows) = (50, 50);

        Self {
            running: false,
            cells: vec![Cell::new(); cell_cols * cell_rows],
            cell_rows,
            cell_cols,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.running = true;
                true
            }
            Msg::Stop => {
                self.running = false;
                false 
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Tick => {
                if self.running {
                    self.step();
                    true
                } else {
                    false 
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = 
            self.cells
            .chunks(self.cell_cols)
            .enumerate()
            .map(|(col, cells)| {
                let offset = col * self.cell_cols;
                let cell_grid = 
                    cells
                    .iter()
                    .enumerate()
                    .map(|(row, cell)| self.render(offset + row, cell, ctx.link()));
                html! {
                    <div key={col} class="grid-row">
                        { for cell_grid }
                    </div>
                }
            });
        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Page>();
}


