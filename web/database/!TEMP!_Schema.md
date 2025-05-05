```sql
CREATE TABLE IF NOT EXISTS "Contacts"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "website" TEXT, 
    "uri_phone" TEXT,
    "email" TEXT
);


CREATE TABLE IF NOT EXISTS "Costs" (
    "id" INTEGER NOT NULL PRIMARY KEY,
    "cost" TEXT,
    "cost_type" TEXT CHECK ( "cost_type" IN (
        'FREE', 'Unlisted'
    )),
    "cost_details" TEXT
)


CREATE TABLE IF NOT EXISTS "Keywords"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "service_type" TEXT CHECK ( "service_type" IN (
        'Resource Finder', 'Food Assistance', 'Food Distribution Center',
        'Food Rescue', 'Free Meals', 'Basic Necessities', 'Childcare', 
        'Clothing Center', 'Educational Services', 'Health and Wellness',
        'Hygiene Assistance', 'Interpersonal Services', 'Language Classes',
        'Recovery Assistance', 'Recreational Activity', 'Youth Services',
        'Employment Assistance', 'Financial Assistance', 'Legal Assistance',
        'Technology Assistance', 'Transportation Assistance', 'Case Management',
        'Dental Services', 'Medical Equipment', 'Medical Services', 
        'Optometry Services', 'Pharmaceutical Assistance', 'Physical Therapy',
        'Eviction Mediation', 'Housing Assistance', 'Shelter',
        'Shelter', 'Shelter (Families)', 'Shelter (Individuals)',
        'Shelter (Men)', 'Shelter (Teens)', 'Shelter (Women)', 'Transitional Housing'
    )), 
    "quick_filter" TEXT CHECK ( "quick_filter_new" IN (
        'Financial', 'Food', 'Housing', 'Medical', 'Wellness'
    )), 
    "service_groups" TEXT CHECK ("service_groups" IN (
        'Anyone In Need', 'Adults: 31 - 54 Years', 'Children: 2 - 12 Years',
        'Infants: 0 Months - 1 Year', 'Legal Teens: 18 - 19 Years', 'Older Children: 7 - 10 Years',
        'Preschoolers: 3 - 4 Years', 'School-Aged Children: 5 - 12 Years', 'Seniors: 55 Years+',
        'Teens: 13 - 19 Years', 'Toddlers: 1 -2 Years', 'Tweens: 11 - 12 Years',
        'Young Adults: 20 - 30 Years', 'Young Children: 2 - 6 Years', 'Emancipated Minor',
        'Individuals', 'Single Father', 'Single Mother', 'Single Parent', 'Two-Parent Household',
        'With Children', 'Caregivers', 'Dependents', 'Fathers', 'Mothers', 'Parents', 'Spouses',
        'Lgbtqia+', 'Men', 'Non-Binary', 'Transgender', 'Women', 'All Disabilities',
        'Deaf or Hard of Hearing', 'Developmental Disability', 'Intellectual Disability',
        'Learning Disability', 'Limited Mobility', 'Physical Disability', 'Visual Impairment',
        'All Mental Health', 'Anxiety', 'Bipolar', 'Brain Injury', 'Depression',
        'Eating Disorder', 'Ptsd', 'Suicidal Thoughts', 'All Cancer Types', 'Breast Cancer',
        'Cervical Cancer', 'Colon Cancer', 'Hodgkin Lymphoma', 'Leukemia', 'Lung Cancer',
        'Lymphoma', 'Non-Hodgkin Lymphoma', 'Ovarian Cancer', 'Rectal Cancer',
        'Testicular Cancer', 'Alzheimer', 'Autism', 'Chronic Illness', 'Diabetes',
        'End Of Life Care', 'Genetic Disorder', 'Hiv/Aids', 'Hospitalized',
        'Neuromuscular Disease', 'On-Treatment', 'Post-Treatment', 'Pregnant',
        'Recently Diagnosed', 'Seizures', 'Terminal Illness', 'Immigrants', 'Refugees',
        'Undocumented', 'Adult Students', 'Dropouts', 'Students', 'Apartment Renters',
        'Eviction Notice', 'Home Owners', 'Home Renters', 'Homeless (In Shelter)',
        'Homeless (No Shelter)', 'Near Homeless', '30% Federal Poverty Level',
        '50% Federal Poverty Level', '70% Federal Poverty Level', 'Benefit Recipients',
        'Low-Income', 'Employed', 'Retired', 'Unable To Work', 'Unemployed', 'Fluent English',
        'Limited English', 'No English', 'Has Criminal History', 'In Jail', 'No Criminal History',
        'Released From Jail', 'Alcohol Dependency', 'Dual Diagnosis', 'Opioid Dependency',
        'Smoker', 'Substance Dependency', 'Abuse Or Neglect Survivors', 'Adult Cancer Survivors',
        'All Cancer Survivors', 'Burn Survivors', 'Domestic Violence Survivors',
        'Human Trafficking Survivors', 'Sexual Assault Survivors', 'Trauma Survivors',
        'Young Adult Cancer Survivors'
    )), 
    "service_group_types" TEXT CHECK ( "service_group_types" IN (
        'General', 'Age Group', 'Household', 'Roles', 'Gender And Identity',
        'Disability', 'Mental Health', 'Cancer', 'Health', 'Citizenship',
        'Education', 'Housing', 'Income', 'Employment', 'Language', 
        'Justice System', 'Substance Dependency', 'Survivors'
    )), 
    "tags" TEXT CHECK ( "tags" IN (
        'Activities', 'Community', 'Community Gathering',
        'Family Events', 'Group Function', 'Small Group',
        'Summer Camp', 'Car Seats', 'Cleaning Supplies', 
        'Clothing', 'Feminine Products', 'Food',
        'Groceries', 'Household Items', 'Hygiene', 
        'Laundry', 'Meals', 'Diapers', 'Formula', 
        'Infant Care Items', 'Childcare (infant: 0 months - 1 year)',
        'Childcare (preschooler: 3 - 4 years)', 
        'Childcare (school-aged: 5 - 12 years)',
        'Childcare (toddler: 1 - 2 years)', 'ECEAP', 'Preschool', 
        'Elementary School', 'High School', 'Kindergarten', 'Middle School',
        'Coordinated Entry', 'Eviction', 'Housing', 'Mediation',
        'Rent', 'Shelter', 'Temporary Housing', 'Utilities', 
        'Ankle Brace', 'Arm Brace', 'Back Brace', 'Clinic', 'Contact Lenses',
        'Contraceptives', 'Crutches', 'Dental', 'End of Life Care',
        'Glasses', 'In-Home Medical Care', 'Leg Brace',
        'Medical', 'Medical Bill', 'Medical Equipment', 'Medical Transportation',
        'Medication', 'Neck Brace', 'Prescription', 'Vision', 'Wheelchair', 
        'Child Support', 'Criminal Case', 'Criminal History', 'Custody', 
        'Domestic Violence', 'Legal', 'Restraining Order', 'Green Card', 
        'Immigration', 'Naturalization', 'Budgeting Assistance', 
        'Bus Pass', 'Computer', 'Debt Assistance', 'Financial', 
        'Free', 'Fuel Voucher', 'Reduced Cost', 'BFET', 'Employment', 
        'Job', 'Computer Lab', 'Education', 'English Class', 
        'Language Class', 'Legal Navigation Class', 'Parenting Class', 
        'Technology Class', 'Counseling', 'Emotional Wellbeing', 
        'Mental-Health', 'Suicide Prevention', 'EBT', 'Gov Program', 
        'Grant', 'SSI', 'SSI-Disability', 'TANF', 'Case Management', 
        'Home Visit', 'One-on-One', 'PC', 'PRIME', 'Alcoholics Anonymous', 
        'Narcotics Addicts Anonymous', 'Recovery Group', 'Support Group'
    )),
    "tag_groups" TEXT CHECK ("tag_groups" IN (
        'Community & Recreation', 'Basic Needs', 'Infant Care', 
        'Childcare', 'School', 'Housing', 'Medical', 'Legal', 
        'Immigration', 'Financial', 'Employment', 'Education', 
        'Mental-Health', 'Gov Programs', 'Case Management', 'Support & Recovery'
    ))
);




CREATE TABLE IF NOT EXISTS "Locations"(
    "id" INTEGER NOT NULL PRIMARY KEY,
    "street" TEXT, 
    "unit " TEXT, 
    "city" TEXT,
    "state" TEXT CHECK ( "state" IN (
        'AL', 'AK', 'AZ', 'AR', 'CA', 'CO', 'CT', 'DE', 'FL', 'GA',
        'HI', 'ID', 'IL', 'IN', 'IA', 'KS', 'KY', 'LA', 'ME', 'MD',
        'MA', 'MI', 'MN', 'MS', 'MO', 'MT', 'NE', 'NV', 'NH', 'NJ',
        'NM', 'NY', 'NC', 'ND', 'OH', 'OK', 'OR', 'PA', 'RI', 'SC',
        'SD', 'TN', 'TX', 'UT', 'VT', 'VA', 'WA', 'WV', 'WI', 'WY'
    )), 
    "zipcode" INTEGER, 
    "latitude" REAL, 
    "longitude" REAL,
    "url_address" TEXT, 
    "log_id" INTEGER NOT NULL,
    FOREIGN KEY ("log_id") REFERENCES "Log"(id)
);




CREATE TABLE IF NOT EXISTS "Log"(
    "id" INTEGER NOT NULL PRIMARY KEY, 
    "last_updated_date" TEXT, 
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
    "log_id" INTEGER NOT NULL, 
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
