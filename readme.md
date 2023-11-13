# 🎠 Trotter
Trotter is an *experimental* crate that aims to make
interacting with gemini servers fun and easy.

There's also [Fluffer](https://docs.rs/fluffer), a crate for writing gemini server apps.

## 😊 Requests
Trotter seeks to be a feature-complete gemini client
library, while also being easy to use.

### Trot
If you're only here to send a quick request, refer to the
ergonomic [`trot`] and [`trot_in`] methods.

``` rust
#[tokio::main]
async fn main() {
    trotter::trot("geminiprotocol.net") // gemini:// prefix and root slash can be elided
        .await
        .unwrap();

    trotter::trot_in("localhost/input", "notice me!") // gemini:// prefix and root slash can be elided
        .await
        .unwrap();
}
```

### Actors
For more-detailed requests, Trotter personifies the entity
requesting information as an [`Actor`].

You can use the builder pattern to easily attach a user
agent and client certificate to the actor.

Once you've built an [`Actor`], you can call [`Actor::get`]
to send a request with it.

``` rust
use trotter::{Actor, UserAgent};

#[tokio::main]
async fn main() {
    let owo = Actor::default()
        .user_agent(UserAgent::Indexer)
        .cert_file("id/owo.crt")
        .key_file("id/owo.key");
        
    owo.get("localhost")
        .await
        .unwrap()
}
```

### 🤖 User-agents
Did you know there's a version of the `robots.txt` standard
for gemini? ([robots.txt for Gemini](https://geminiprotocol.net/docs/companion/robots.gmi))

Trotter has robots functionality built-in. Once you set your
user-agent, you will receive a `RobotDenied` error if you
try to access a page you are disallowed from.

I strongly suggest you do this if you're using Trotter for a
project that depends on other peoples' content.

## 🎁 Responses
Once you receive a structured [`Response`], you can either
weed through it yourself, or rely on the helper functions it
implements to preform common operations.

## Tips
### Certificates
If you have access to a posix shell with `openssl`
installed, you can define the following functions to easily
generate and inspect x509 certificates.

``` sh
certgen() { [ -f "${1:?usage: certgen [name]}.key" ] || [ -f "$1.crt" ] || ( openssl req -new -subj "/CN=$1" -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -days 3650 -nodes -out "$1.crt" -keyout "$1.key" && printf '📜 Cert generated\n' ) ;}
certinfo(){ openssl x509 -noout -text < "${1:?usage: certinfo [file]}" ;}
```

## Todo
For now, I want this to be a helpful tool for automating
gemini requests. But ultimately, I would like for it to be
robust enough to write a complete client with.

- [X] Write response to file
- [X] Get response as gemtext
- [X] robots.txt support
- [X] Custom errors
- [X] Cli binary 👀
- [ ] Server certificates
