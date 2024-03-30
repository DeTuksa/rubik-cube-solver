use rubik_cube_solver::RubikCube;

fn main() {
    let cube = RubikCube::read_from_input();
    cube.print();
}
