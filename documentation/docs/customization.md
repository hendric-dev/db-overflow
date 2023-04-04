---
title: Customization
---

# Customization

The tool is able to autodiscover the database tables and generate data based on the column types. \
Some use cases may require to customize this behaviour. The `config` command can be used to generate a configuration
file from an existing table:

```sh
db-overflow config --table users --output users.schema.json
```

The generated `users.schema.json` may look like this:

```json
{
  "name": "users",
  "columns": [
    {
      "name": "id",
      "data_type": "Uuid"
    },
    {
      "name": "name",
      "data_type": "Text"
    },
    {
      "name": "age",
      "data_type": "Integer"
    },
    {
      "name": "email",
      "data_type": "Text"
    },
    {
      "name": "created",
      "data_type": "Timestamp"
    },
  ]
}
```

The config can be customized as needed. E.g. to give all users the same age:

```json
{
  "name": "age",
  "data_type": "Integer",
  "value": "28"
},
```

This prevents generating a random number and instead uses the defined value.

Don't forget to pass the config to the run command: `db-overflow run -c users.schema.json`
