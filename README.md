# Rust Actix-Web-App 


This is the first Rust web application I have created.  I considered Actix over Rocket but they seemed pretty similar in syntax and Actix seems to perform faster.

this actix example uses


in order to use this app you will need a .env file that contains the connection string to your postgres sql.

```DATABASE_URL=postgres://username:password@localhost/dbname```


you will also need the diesel cli with the postgres feature enabled in order to interact with migration changes.

1. Diesel - using postgres
2. actix-http for third party http request

<br/>
<br/>

### TODO
later on I may add web app features such as calling a third party caching service or maybe some middlware for logging.  I'm also interested in experimenting with adding web sockets.
