# La Poste Releve Web
I have created these application in order to  visualize all my expenses.

"La Banque Postale" has history of your banking activities of few weeks.

So once in a while, I'm downloading my banking statements as CSV.

Instead of going through all those CSVs in order to find some informations, this application will:
* build a database (sqlite)
* run a HTTP server serving a React app
* display some stats


# Techs
It uses:
* Rust / Typescript
* React (react-chartjs-2)
* sqlite

# Build

* To build the database, go to the `cli` folder and run

`cargo run -- --db`

* To run the <ins>**API**</ins> server, go to the `cli` folder and run

`cargo run -- --http // You need to build the db first`

* To run the web application, go to the `webapp` folder and run
  
`npm install //Just the first time` 

and then 

`npm run dev`


A script `build.sh` at the project root will build all projects and copy all necessary files in a `dist` folder with a config file (`config.toml`)pointing to some test data.

```
dist
├── config.toml
├── data
│   ├── input01.csv
│   ├── input02.csv
│   └── input03.csv
├── lpr-rs
└── www
    ├── index.html
    └── main.bundle.js
```

If you use `build.sh` to build the projects,

* To build the database, go to the `dist` folder and run

`lpr-rs --db`

* To run the web application, go to the `cli` folder and run

`pr-rs --http`


The app will be accessible at `http://localhost:3030/`



