```sql
CREATE TABLE IF NOT EXISTS "Contacts"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "website" TEXT, 
    "display_phone" TEXT, 
    "uri_phone" TEXT,
    "email" TEXT
);



CREATE TABLE IF NOT EXISTS "Keywords"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "service_type_new" TEXT, 
    "quick_filter_new" TEXT CHECK ( "quick_filter_new" IN (
        'Financial', 'Food', 'Housing', 'Medical', 'Wellness'
    )),
    "service_type_id" TEXT,
    "service_groups" TEXT, 
    "service_group_types" TEXT, 
    "tags_new" TEXT, 
    "tag_groups" TEXT,
    "tag_ids" TEXT
);



CREATE TABLE IF NOT EXISTS "Log"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "updated_date" TEXT, 
    "last_updated_by" TEXT, 
    "date_added" TEXT,
    "created_by" TEXT, 
    "update_status" TEXT, 
    "date_of_next_update" TEXT
);



CREATE TABLE IF NOT EXISTS "Orgs"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "name" TEXT, 
    "description" TEXT, 
    "open_hours" TEXT,
    "org_type" TEXT CHECK( "org_type" IN (
        'Church', 
        'Community Center', 
        'For-Profit',
        'Government',
        'Non-Profit',
        'Outreach Ministry', 
        'School',
        'Technical Assistance Organization'        
    )),
    "status" TEXT CHECK( "status" IN (
        'Active', 'Closed', 'Temporary Closure'
    )),
    "parent_org_id" INTEGER NOT NULL,   
    "contact_id" INTEGER NOT NULL,
    "org_location_id" INTEGER NOT NULL, 
    "log_id" INTEGER NOT NULL, 
    "?_11" TEXT, 
    "?_12" TEXT,
    "location_id" INTEGER NOT NULL,
    FOREIGN KEY ("parent_org_id") REFERENCES "Orgs"(id),
    FOREIGN KEY ("contact_id") REFERENCES "Contacts"(id),
    FOREIGN KEY ("location_id") REFERENCES "Locations"(id),
    FOREIGN KEY ("log_id") REFERENCES "Log"(id),
    FOREIGN KEY ("location_id") REFERENCES "Locations"(id)
);


CREATE TABLE IF NOT EXISTS "Services"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "name" TEXT, 
    "org_id" INTEGER NOT NULL, 
    "location_id" INTEGER NOT NULL,
    "status" TEXT CHECK( "status" IN (
        'Active', 'Closed', 'Temporary Closure'
    )),
    "details" TEXT, 
    "open_hours" TEXT, 
    "is_online_only" TEXT CHECK( "is_online_only" IN (
        'Checked', ''
    )),
    "is_application_only" TEXT CHECK( "is_application_only" IN (
        'Checked', ''
    )),
    "eligibility" TEXT, 
    "cost_id" INTEGER NOT NULL, 
    "contact_id" INTEGER NOT NULL,
    "language_info" TEXT, 
    "log_id" INTEGER NOT NULL, 
    "keyword_id" INTEGER NOT NULL,
    FOREIGN KEY ("org_id") REFERENCES "Orgs"(id),
    FOREIGN KEY ("location_id") REFERENCES "Locations"(id),
    FOREIGN KEY ("contact_id") REFERENCES "Contacts"(id),
    FOREIGN KEY ("log_id") REFERENCES "Log"(id),
    FOREIGN KEY ("keyword_id") REFERENCES "Keywords"(id)
);
```