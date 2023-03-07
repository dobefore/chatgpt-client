add auth verification:copy user related code from anki-sync-server-rs
For the free plan of OpenAI's GPT-3 API, there is a limit of 4,000 tokens per month. This limit applies to all API requests made using the free plan, including requests made by the ChatGPT web application.
drawn from the above,no need to enable user verifying,as available tokens are few.

add tls support.
1. try to use self-signed certificate. use rustls crate.copy code ffrom anki-sync-server-rs.It is mixed with tow many feature annotations,so give up,resort to actix-web example repo.

compress the repo and send to cloud linux. comopile this project,download mkcert and generate the cert and keys ,copy them into the executable folder and  create env vars for them.

kill old v2ray and launch it with new client config.

2. use let's encript

## auth veriying
