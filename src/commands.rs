/// Creates a new project directory, src subfolder and jango.toml
///
pub fn new_project(project_name: &str) -> std::io::Result<()> {
    use std::env;
    use std::fs::{self, DirBuilder, File};
    use std::io::Write;

    //try to create the project directory, check if dir of name already exists
    let mut root = env::current_dir()?;
    root.push(project_name);
    println!("Trying to create '{}' project under '{}' ...", project_name, root.display());

    if let Err(e) = DirBuilder::new().create(root.clone()){
        println!("A directory with that name already exists."); 
        return Err(e);
    };

    
    //create jargo.toml
    let tomlpath = root.clone();
    tomlpath.push("jargo");
    tomlpath.set_extension("toml");

    //create src dir
    let srcpath = root.clone();
    srcpath.push("src");

    //because the directory is guaranteed to not already contain a similarly named folder
    //(we just created it), and we can't handle the obscure errors that may still cause it to fail,
    // it is okay to cascade the ? here instead of matching the result it in place.
    DirBuilder::new().create(srcpath)?;


    //create Main.java
    let mainpath = root.clone();
    mainpath.push("src");
    mainpath.push("Main");
    mainpath.set_extension("java");

    let mut f = File::create(root)?;
    f.write_all(
br#"class Main{
    public static void main (String[] args){
        System.out.println("Hello world");
    }
}
"#)?;





    Ok(())
}