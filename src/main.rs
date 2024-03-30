use rubik_cube_solver::RubikCube;

fn main() {
    let cube = RubikCube::new();
    println!("<<<<<<<<<<<<Front Face>>>>>>>>>>>");
    cube.faces[0].print_face();
    println!("<<<<<<<<<<<<Back Face>>>>>>>>>>>");
    cube.faces[1].print_face();
    println!("<<<<<<<<<<<<Up Face>>>>>>>>>>>");
    cube.faces[2].print_face();
    println!("<<<<<<<<<<<<Down Face>>>>>>>>>>>");
    cube.faces[3].print_face();
    println!("<<<<<<<<<<<<Left Face>>>>>>>>>>>");
    cube.faces[4].print_face();
    println!("<<<<<<<<<<<<Right Face>>>>>>>>>>>");
    cube.faces[5].print_face();
}
