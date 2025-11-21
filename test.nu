let dbname = "test.db"

def "main init-db" [--force] {
  if ($force) {
    rm -f $dbname
  } else if ($dbname | path exists) {
    print $"($dbname) exists"
    exit 1
  }

  "CREATE TABLE IF NOT EXISTS artist(name TEXT PRIMARY KEY, WITHOUR ROWID)" | sqlite3 $dbname
  "CREATE TABLE IF NOT EXISTS release(id INTEGER PRIMARY KEY, name TEXT NOT NULL, artistname STRING NOT NULL, year INTEGER NOT NULL, FOREIGN KEY(artistname) REFERENCES artist(name))" | sqlite3 $dbname
  "CREATE TABLE IF NOT EXISTS log(id INTEGER PRIMARY KEY, release_id INTEGER NOT NULL, date TEXT, FOREIGN KEY(release_id) REFERENCES release(id))" | sqlite3 $dbname
}

def "main fill" [] {
  let values = {
    "Kelly Moran": ["Don''t Trust Mirrors"],
    "The Beatles": ["Rubber Soul", "The White Album"],
    "Stereolab": ["Peng!", "Dots and Loops"]
  } | transpose key val

  for v in $values {
    $"INSERT INTO artist\(name\) VALUES \('($v.key)'\)" | sqlite3 $dbname
    $v.val | each {|album| $"INSERT INTO release\(name, artistname, year\) VALUES \('($album)','($v.key)',2025\)" | sqlite3 $dbname }
  }
}
