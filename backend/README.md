# `rs-diesel-sqlite`

Diesel's `Getting Started` guide using SQLite instead of Taskgresql

## Usage

```
$ echo "DATABASE_URL=file:test.db" > .env
$ diesel migration run

$ cargo run --bin show_tasks

$ cargo run --bin write_task
# write your task

$ cargo run --bin publish_task 1

$ cargo run --bin show_tasks
# your task will be printed here

# Delete task with given title
$ cargo run --bin delete_task "title of task to delete"

$ cargo run --bin show_tasks
# observe that no tasks are shown
```
