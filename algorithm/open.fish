#!/usr/bin/env fish

function usage
    echo "Usage: fish open.fish <contest-id>" >&2
    echo "Example: fish open.fish abc023" >&2
    echo "         fish open.fish adt_all_20260917_1" >&2
end

if test (count $argv) -ne 1
    usage
    exit 2
end

set --local contest (string lower -- $argv[1])

if not string match --quiet --regex '^[a-z0-9][a-z0-9_-]*$' -- $contest
    echo "open.fish: invalid contest ID: $argv[1]" >&2
    usage
    exit 2
end

set --local contest_directory

if string match --quiet --regex '^adt_[a-z0-9]+_[0-9]{8}_[0-9]+$' -- $contest
    set --local parts (string split '_' -- $contest)
    set --local category $parts[2]
    set --local date $parts[3]
    set --local year (string sub --start 1 --length 4 -- $date)
    set --local month (string sub --start 5 --length 2 -- $date)
    set contest_directory "contests/adt/$category/$year/$month/$contest"
else if string match --quiet --regex '^(abc|arc|agc|ahc)[0-9]{3}$|^awc[0-9]{4}$' -- $contest
    set --local series (string replace --regex '[0-9]+$' '' -- $contest)
    set --local digits (string replace --regex '^[a-z]+' '' -- $contest)
    set --local number (string replace --regex '^0+' '' -- $digits)
    if test -z "$number"
        set number 0
    end

    if test "$number" -eq 0
        echo "open.fish: the contest number must be at least 1" >&2
        exit 2
    end

    set --local range_start (math "floor($number / 100) * 100")
    set --local range_end (math "$range_start + 99")

    if test "$series" = awc
        set range (printf '%04d-%04d' $range_start $range_end)
    else
        set range (printf '%03d-%03d' $range_start $range_end)
    end

    set contest_directory "contests/$series/$range/$contest"
else
    set contest_directory "contests/othres/$contest"
end

set --local repository_root (path resolve (status dirname))
set contest_directory "$repository_root/$contest_directory"

cd "$repository_root"; or exit 1
cargo compete new "$contest"; or exit $status

if not test -d "$contest_directory"
    echo "open.fish: cargo-compete did not create $contest_directory" >&2
    exit 1
end

cd "$contest_directory"; or exit 1
code .; or exit $status
wezterm cli kill-pane
