# Shiny-Rs

## How to write your first Shiny-rs App

Writing a Shiny-rs app is surprisingly simple! Most of the hard work will come
from creating plots, writing models, connecting with external APIs, etc.
This tutorial will cover two different methods of creating a Shiny-rs.

1. Using the default server and session.
2. Using a custom server and session.

_This tutorial assumes you already have Rust and Cargo installed._

### Using the default server

Rust's statically typed and safe nature makes it work quite differently from
R or Python. Having different environments that hold differing types of data is
not allowed. If you want to store session wide data with custom types please
read the **Using a custom server and session** section. The default server
allows for very basic interaction between the `input` and the `output`.
Anything more complex will require a custom server.

The first step will be to create a new project with:

```
cargo new my-shiny-app
cd my-shiny-app
```

Since this crate is still unstable and not ready to be published onto `crates.io`
you will need to add it as a Git dependency. On your `Cargo.toml` add the following lines
to the dependencies section:

```toml
shiny-rs = { git="https://github.com/andyquinterom/shiny-rs", branch="master" }
actix = "0.13"
actix-files = "0.6"
actix-web-actors = "4.1"
actix-web = "4.1"
tokio = "1.20"
```

This will add the dependencies necessary to run out app.

For this example we will create all the app logic in a single file (`main.rs`).
This is not recommended but serves the purpose for this example. The same
principles can be applied for setting up the app in multiple modules.

#### Creating the UI

Creating the UI can be done however you want! If you want to use R, it's ok.
If you want to use Python, that's also ok. If you want to write it in pure HTML,
that's also cool. You have all the freedom in world to use any front-end framework.
The only requirement is to use Shiny. You will need to include the dependency
on the HTML files either with a CDN or by directly exposing the files through
`actix-files`. In this tutorial we will use R for building the UI and
directly expose the files.

We will create a new directory on the root of the project called `static`.
This example is a single page application, so we will create a
new file called `index.R`. In this file we will write a small script
that renders our UI into HTML.

You can copy/paste the following or write your own:

```R
library(shiny)
library(devtools)
library(bslib)
library(htmltools)

# This function just get's the version of an R package you have installed.
# You could use Renv or anyother means of setting a static version if you need
# it.
get_package_version <- function(pkg) {
  ns <- .getNamespace(pkg)
  if (is.null(ns)) {
    utils::packageVersion(pkg)
  } else {
    as.package_version(ns$.__NAMESPACE__.$spec[["version"]])
  }
}

# The following functions add the HTML dependecies
# needed for `htmltools` to be able to properly
# render our UI. We could package this functions
# into an R package in the future.
jqueryDeps <- htmlDependency(
  "jquery",
  "3.6.0",
  src = "www/shared",
  package = "shiny",
  script = "jquery.min.js",
  all_files = FALSE
)

shinyDependencyCSS <- function(theme) {
  version <- get_package_version("shiny")

  if (!is_bs_theme(theme)) {
    return(htmlDependency(
      name = "shiny-css",
      version = version,
      src = "www/shared",
      package = "shiny",
      stylesheet = "shiny.min.css",
      all_files = FALSE
    ))
  }

  scss_home <- system_file("www/shared/shiny_scss", package = "shiny")
  scss_files <- file.path(scss_home, c("bootstrap.scss", "shiny.scss"))
  scss_files <- lapply(scss_files, sass::sass_file)

  bslib::bs_dependency(
    input = scss_files,
    theme = theme,
    name = "shiny-sass",
    version = version,
    cache_key_extra = version
  )
}

shinyDependencies <- function() {
  list(
    jqueryDeps,
    bslib::bs_dependency_defer(shinyDependencyCSS),
    htmlDependency(
      name = "shiny-javascript",
      version = get_package_version("shiny"),
      src = "www/shared",
      package = "shiny",
      script = "shiny.min.js",
      all_files = FALSE
    )
  )
}

ui <- fluidPage(
  # We must include the dependencies somewhere
  shinyDependencies(),
  title = "My Rust App",
  numericInput("x", "X", value = 0),
  numericInput("y", "Y", value = 0)
)

htmltools::save_html(ui, "static/index.html", libdir = "lib")
```

Now that we have our UI written, let's render it. In our shell
we can run the script with: 

```sh
Rscript static/index.R
```

This will create an `index.html` file and a directory `static/lib` with
all our JS/CSS dependencies.

#### Writing our server

The default server and session take in a couple of functions.

- `initialize`: a  function that runs when a session starts.
- `update`: a function that runs when an input updates.
- `tick`: a function that runs on every session tick.
- `hb_interval`: how often should the session tick.
- `client_timeout`: after how much should the client timeout.

Before defining out functions we must setup out modules
in out `main.rs`.

At the top of the file we can import the following:

```rs
use shiny_rs::session::{ ShinyServer, ShinySession };
use actix_web::{ web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder };
use actix_files::NamedFile;
use actix_web_actors::ws;
use std::time::Duration;
```

This imports the necessary `actix` dependencies and the default `shiny-rs`
server and session.

Next we will create an endpoint that returns out front-end static content.
We will define an async function that returns our `index.html` file.

```rs
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}
```

Now we will actually build our server logic. First, we will define our
functions, heartbeat interval and client timeout duration.

```rs
// These functions take in a ShinyServer and a ShinySession.
// They will run on different moments of our session.
fn initialize(shiny: &mut ShinyServer, session: &mut ShinySession) {
}
fn update(shiny: &mut ShinyServer, session: &mut ShinySession) {
}
fn tick(shiny: &mut ShinyServer, session: &mut ShinySession) {
}

// These intervals use the `from_secs` function. However,
// you could lower the tick rate or increase it. It's all
// on you!
const HB_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
```

Now we will define our websockets server with:

```rs
async fn server(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ShinyServer::new(initialize, update, tick, HB_INTERVAL, CLIENT_TIMEOUT), &req, stream)
}
```

Here we create a new ShinyServer taking in our functions and our intervals.

To make it all work together we will define the `main` function with the following:

```rs
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/lib", "./static/lib").show_files_listing())
            .service(web::resource("/websocket/").route(web::get().to(server)))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))? // Change the port and IP accordingly
    .run()
    .await
}
```

Now we can run out app with `cargo run`. The server should start up on the binding
address and port we defined on the last step.

##### Defining logic

Now that we ran our app successfully. Let's make it actually do something.
In this example, we took in an input `x` and `y`, both of numeric type.
We will make the server send a notification to the user with the sum
of both numbers.

Before doing anything, let's add the `changed` macro, the `ui` module
and the `generate_id` function from `shiny-rs` at the top of our `main.rs`
file.

```rs
use shiny_rs::{ changed, ui, session::generate_id };
```

Inside our `update` function we will and the following:

```rs
fn update(shiny: &mut ShinyServer, session: &mut ShinySession) {
    // Will only run if x or y change
    if changed!(shiny, ("x:shiny.number", "y:shiny.number")) {
        // Will show a notification with a random id, and the sum of x and y as
        // two floating point numbers.
        // We use the unwrap_or(0.0) in case the input can't be parsed
        // into an f64 value, it will return 0.
        let x = shiny.input.get_f64("x:shiny.number").unwrap_or(0.0);
        let y = shiny.input.get_f64("y:shiny.number").unwrap_or(0.0);
        // the `ui::args!` let's us define the arguments for the UI.
        ui::show_notification(session, ui::args!({
            "id": generate_id(),
            "html": format!("The sum is: {}", x + y),
            "type": "message"
        }))
    }
}
```

This will display a notification with the sum of x and y as floating
point values every time any of the two inputs change.

We can also add a welcome notification by changing our initialize
function:

```rs
fn initialize(shiny: &mut ShinyServer, session: &mut ShinySession) {
    ui::show_notification(session, ui::args!({
        "id": "welcome",
        "html": "<b>Welcome</b>",
        "type": "message"
    }));
}
```

We can test out app with `cargo run`.

That's all you need to get started!

### Using custom server

Still being written... For now, you can checkout the official
example hosted on
[https://experiment.indexmic.com](https://experiment.indexmic.com) or
on the official repo
[https://github.com/andyquinterom/shiny-rs-example](https://github.com/andyquinterom/shiny-rs-example).
