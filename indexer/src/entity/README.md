# Schema

SQL generated from entities by SeaORM

```sql
CREATE TABLE "debugger_info"
(
    "id"             integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "debugger"       text    NOT NULL,
    "version"        text    NOT NULL,
    "workspace_root" text    NOT NULL,
    "package_name"   text    NOT NULL,
    "target"         text    NOT NULL,
    "arguments"      text    NOT NULL,
    "exit_code"      integer
);
CREATE TABLE "file"
(
    "id"         integer NOT NULL PRIMARY KEY,
    "path"       text    NOT NULL,
    "crate_name" text    NOT NULL,
    "modified"   text    NOT NULL
);
CREATE TABLE "breakpoint"
(
    "id"              integer NOT NULL PRIMARY KEY,
    "file_id"         integer NOT NULL,
    "loc_line"        integer NOT NULL,
    "loc_column"      integer,
    "breakpoint_type" text    NOT NULL,
    "capture"         text    NOT NULL,
    "hit_count"       bigint  NOT NULL,
    FOREIGN KEY ("file_id") REFERENCES "file" ("id")
);
CREATE TABLE "event"
(
    "id"              integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "breakpoint_id"   integer NOT NULL,
    "thread_id"       bigint  NOT NULL,
    "frame_id"        bigint  NOT NULL,
    "parent_frame_id" bigint,
    "stack_pointer"   bigint,
    "function_name"   text,
    "event_type"      text(1) NOT NULL,
    "timestamp"       text    NOT NULL,
    "data"            text    NOT NULL,
    "pretty"          text    NOT NULL,
    "is_error"        boolean NOT NULL,
    FOREIGN KEY ("breakpoint_id") REFERENCES "breakpoint" ("id")
);
CREATE INDEX "idx-event-frame_id" ON "event" ("frame_id");
CREATE TABLE "function"
(
    "function_name" text NOT NULL PRIMARY KEY
);
CREATE TABLE "type_info"
(
    "type_name"  text NOT NULL PRIMARY KEY,
    "attributes" text
);
```