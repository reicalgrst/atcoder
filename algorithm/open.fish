#!/usr/bin/env fish

function print_usage
    echo "Usage: fish open.fish <contest-id>" >&2
    echo "Example: fish open.fish abc023" >&2
    echo "         fish open.fish adt_all_20260917_1" >&2
end

function contest_directory --argument-names contest
    if string match --quiet --regex '^past.*$' -- $contest
        echo "contests/past/$contest"
        return
    end

    if string match --quiet --regex '^adt_[a-z0-9]+_[0-9]{8}_[0-9]+$' -- $contest
        set --local parts (string split '_' -- $contest)
        set --local category $parts[2]
        set --local year (string sub --start 1 --length 4 -- $parts[3])

        echo "contests/adt/$category/$year/"(string sub --start 5 --length 2 -- $parts[3])"/$contest"
        return
    end

    if string match --quiet --regex '^(abc|arc|agc|ahc)[0-9]{3}$|^awc[0-9]{4}$' -- $contest
        set --local series (string replace --regex '[0-9]+$' '' -- $contest)
        set --local number (string replace --regex '^[a-z]+0*' '' -- $contest)

        if test -z "$number"
            echo "open.fish: the contest number must be at least 1" >&2
            return 2
        end

        set --local range_start (math "floor($number / 100) * 100")
        set --local range_end (math "$range_start + 99")
        set --local width 3
        if test "$series" = awc
            set width 4
        end

        set --local range (printf "%0*d-%0*d" $width $range_start $width $range_end)
        echo "contests/$series/$range/$contest"
        return
    end

    echo "contests/othres/$contest"
end

function main
    if test (count $argv) -ne 1
        print_usage
        return 2
    end

    set --local contest (string lower -- $argv[1])

    if not string match --quiet --regex '^[a-z0-9][a-z0-9_-]*$' -- $contest
        echo "open.fish: invalid contest ID: $argv[1]" >&2
        print_usage
        return 2
    end

    set --local relative_directory (contest_directory $contest)
    if test $status -ne 0
        return 2
    end

    set --local repository_root (path resolve (status dirname))
    set --local destination "$repository_root/$relative_directory"

    if not test -d "$destination"
        cd "$repository_root"; or return 1
        cargo compete new "$contest"; or return $status
    end

    if not test -d "$destination"
        echo "open.fish: cargo-compete did not create $destination" >&2
        return 1
    end

    cd "$destination"; or return 1
    code .
end

main $argv
