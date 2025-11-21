# Command for logging music you listened to
def mlog [] {
  print "Command for logging music you listened to.\n\nTry \"help mlog\""
}

module commands {
  def completition-releases [] {
    let releases = (musiklog release list | from json | each {|| get name })
    {
      options: {
        case_sensitive: false,
        completiton_algorithm: fuzzy,
        sort: false
      },
      completions: $releases
    }
  }

  def completition-artists [] {
    let artists = (musiklog artist list | from json | each {|| get name })
    {
      options: {
        case_sensitive: false,
        completiton_algorithm: fuzzy,
        sort: false
      },
      completions: $artists
    }
  }

  # List all releases or for a certain artist
  export def "mlog release" [
      artist?: string@completition-artists # Only list for this artist
    ] {
    if $artist != null {
      musiklog release list --artist $artist | from json | each {|| { name: $in.name, year: $in.year }}
    } else {
      musiklog release list | from json
    }
  }

  # Add new release
  export def "mlog release add" [
      artist: string@completition-artists # Artist for release
      name: string # Name of the release
      year: number # Release year of the release
    ] {
    musiklog release add $artist $name $year
  }

  # List all artists
  export def "mlog artist" [] {
    musiklog artist list | from json
  }

  # Add new artist
  export def "mlog artist add" [
    name: string # Name of the artist
    ] {
    musiklog artist add $name
  }

  # List all logs
  export def "mlog log" [] {
    musiklog log list | from json
  }

  # Add a new log for a release
  export def "mlog log add" [
      release: string@completition-releases # Name of the release
      date?: string # Date of listen, or default to today
    ] {
    if $date != null {
      musiklog log add $release --date $date
    } else {
      musiklog log add $release
    }
  }
}

use commands *
