---
description: Outlining for the database format
tags:
 - 
---

# Questions
* Make sample query
to print contact info and address
```SQL
SELECT *
From Services AS S
JOIN Contact AS C ON C.id = S.contact_id
JOIN Address AS A ON A.id = S.address_id
```

# Todos:
- [x] Schema defined
- [X] Modify csvs to schema
	- [X] Orgs
	- [X] Services
	- [X] Locations
	- [X] Cost
	- [X] Keywords
	- [X] Contacts
- [X] Set types manually in `create.sql`

# create.sh
I have created a bash script that converts a collection of CSV's like that in `finished_csv\'s`:

```bash
$ ./create.sh finished_csv\'s/ database.db
[   OK    ] Removing previous database.db and create.sql
[   OK    ] Creating empty Sqlite DB
[   OK    ]    `OrgContacts` imported
[   OK    ]    `OrgLocation` imported
[   OK    ]    `OrgLog` imported
[   OK    ]    `Orgs` imported
[   OK    ] Export db as create.sql file

$ ls
 create.sh   create.sql   database.db  "finished_csv's"  "temp_downloaded_airtable_csv's"
```

The only issue is that the .db file that it creates doesn't have the correct schema, instead everything is TEXT type.
I believe the insert commands won't have to be modified at all though, so the quickest solution is rather than modifying the bash script further, to instead manually modify the schema's defined in the `create.sql` and recompile the `.db`


# create.sql

https://www.sqlitetutorial.net/sqlite-primary-key/
https://www.sqlitetutorial.net/sqlite-foreign-key/
https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite

!! An issue we are currently facing with this is that after setting up the types and keys in the create.sql, some mistakes must have been made because the services and orgs tables, yk the most important ones, just aren't loading in there data due to errors.

# Tables

## Org
| Column        | Type   | Notes                                                                  |
| ------------- | ------ | ---------------------------------------------------------------------- |
| id            | Int    | PK                                                                     |
| name          | String |                                                                        |
| description   | String |                                                                        |
| open_hours    | String |                                                                        |
| org_type      | Enum   | Non-Profit/Government/Church/Community Center/Outreach Ministry/School |
| status        | Enum   | Active/Inactive/Temporary Closure                                      |
| parent_org_id | Int    | SRK* from Organizations                                                |
| contact_id    | Int    | FK from Contacts                                                       |
| location_id   | Int    | FK from Location                                                       |
| log_id        | Int    | FK from Log                                                            |
|               |        |                                                                        |
*SRK means self referencing key*

## Servies
| Column              | Type    | Notes                             |
| ------------------- | ------- | --------------------------------- |
| id                  | Int     | PK                                |
| name                | String  |                                   |
| org_id              | Int     | FK from Organizations             |
| location_id         | Int     | FK from Locations                 |
| status              | Enum    | Active/Inactive/Temporary Closure |
| details             | String  |                                   |
| open_hours          | String  |                                   |
| is_online_only      | Boolean |                                   |
| is_application_only | Boolean |                                   |
| eligibility         | Text    |                                   |
| cost_id             | Int     | FK from Cost                      |
| contact_id          | Int     | FK from contact                   |
| language_info       | Text    |                                   |
| log_id              | int     |                                   |
| keyword_id          | int     |                                   |

## Cost
this is just so we don't lose some important stuff but won't be really used much for this project.
| Column             | Type | Notes |
| ------------------ | ---- | ----- |
| id                 | Int  | PK    |
| OLD - Service Cost | Text |       |
| cost_type          | Text |       |
| set_cost           | Text |       |



## Log
| Column              | Type   | Notes                             |
| ------------------- | ------ | --------------------------------- |
| id                  | Int    | PK                                |
| last_updated_date   | Epoch  |                                   |
| last_updated_by     | String | Maybe later make employee table?  |
| date_added          | Epoch  |                                   |
| created_by          | String | Active/Inactive/Temporary Closure |
| update_status       | Enum   |                                   |
| date_of_next_update | Epoch  |                                   |
*Epoch time being a standard referring to the number of milliseconds sense 1970 Jan 1st*

## Contact
| Column        | Type   | Notes                     |
| ------------- | ------ | ------------------------- |
| id            | Int    | PK                        |
| website       | URL    | ex. `https://website.com` |
| display_phone | String | +1 (123)-456-7890         |
| url_phone     | URI    | ex. `tel://1234567890`    |
| email         | String | "name@website.com"        |

## Keywords
| Column     | Type | Notes                     |
| ---------- | ---- | ------------------------- |
| id         | Int  | PK                        |
| word       | URL  | ex. `https://website.com` |
| service_id | Int  | FK from Service           |
| org_id     | Int  | FK from Org               |

* need to cross check ids
* currently does not have latitude and longitude.
## Location
| Column      | Type   | Notes |
| ----------- | ------ | ----- |
| id          | Int    | PK    |
| street      | String |       |
| unit        | String |       |
| city        | String |       |
| state       | Enum   |       |
| zip         | Int    |       |
| latitude    | Double |       |
| longitude   | Double |       |
| google_maps | URL    |       |

