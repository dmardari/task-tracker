# Task Tracker

This repo contains an example of a task tracker with REST API and Prisma ORM in Rust language.

## Running

1. Prepare the environment variables
The following 4 variables need to be set up to run the task tracker: `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB` and `DATABASE_URL`. You can configure your own env variables or just use the example provided in the repo:
```bash
$ cp .env.example .env
```
2. Prepare the PostgreSQL instance
You can configure any instance of PostgreSQL to be used by the task tracker or just use the example `docker-compose.yml` file in the repo:
```bash
$ docker-compose up
``` 
3. Set up the database
DB schema must be applied to the DB. To do this just execute:
```bash
$ cargo prisma migrate dev
```
4. Launch the task tracker
```bash
$ cargo run --bin task-tracker
```
5. Use the task tracker
Use your REST client of choise (e.g. Postman). The following endpoints are available:

`/tasks` :
- `GET` : Lists all tasks
- `POST` : Create a task
  - ```json
    {
        "description": "a new task description"
    }
`/task/<uuid>` :
- `DELETE` : Delete a task

`/task/<uuid>/complete` :
- `PATCH` : Mark a task a completed (`completed: true`)