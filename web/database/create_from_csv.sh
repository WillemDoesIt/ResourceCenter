# Refrences:
# - https://www.redhat.com/en/blog/process-script-inputs
# - https://www.cyberciti.biz/faq/bash-for-loop/
# - https://stackoverflow.com/questions/14947916/import-csv-to-sqlite

# To run this on windows first run `winget install Sqlite.sqlite` in powershell
# Then run `chmod +x create_from_csv.sh` in gitbash
# Make sure you can run `sqlite3 --version` first
# Then run `./create_from_csv.sh finished_csv\'s/ database.db` in gitbash

CSV_DIR="$1"
DB_FILE="$2"
SQL_FILE="create.sql"

function safe_runner() {
    eval "$1" > /dev/null 2>&1
    if [ $? -ne 0 ]
    then
        echo -e "\033[0;31m[  Error  ]\033[0m $2"
        eval "$1"
        exit 1
    else
        echo -e "\033[0;32m[   OK    ]\033[0m $2"
    fi
}

safe_runner "rm -f $DB_FILE $SQL_FILE" "Removing previous $DB_FILE and $SQL_FILE"
safe_runner "sqlite3 $DB_FILE \"PRAGMA journal_mode=WAL;\"" "Creating empty Sqlite DB"

# Loop through all CSVs in the directory
for file in "$CSV_DIR"/*.csv; do
  [ -e "$file" ] || continue
  table=$(basename "$file" .csv)

  safe_runner "sqlite3 $DB_FILE \".mode csv\" \".import $file $table\"" "   \`$table\` imported"

done

safe_runner "sqlite3 $DB_FILE \".dump\" > \"$SQL_FILE\"" "Export db as create.sql file \n"
