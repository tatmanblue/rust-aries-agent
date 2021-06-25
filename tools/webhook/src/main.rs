use std::env;
use tide::*;

static mut COUNTER: i32 = 0;

//
// a simple listener that will listen at a given address and dump the body of what
// is received
//
#[async_std::main]
async fn main()  -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if 2 != args.len() {
        panic!("only one argument is allowed:  url.  Example: `http://localhost:5001'");
    }

    let mut app = tide::new();
    app.at("/:topic/:subtopic").post(|mut req: Request<()>| async move {
        let mut print_count: i32;
        unsafe {
            COUNTER = COUNTER + 1;
            print_count = COUNTER;
        }
        let topic = req.param("topic").unwrap_or("").to_string();
        let subtopic = req.param("subtopic").unwrap_or("").to_string();
        let data: String = req.body_string().await?;
        println!("- {} ----------------------------------------------------------------------- {} ---",
                 print_count, print_count);
        println!("/{}/{}\r\n{}\r\n", topic, subtopic, data);
        Ok("ok")
    });

    println!("listening on {:?}", args[1]);

    app.listen(args[1].to_string()).await?;

    Ok(())
}
