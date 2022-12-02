function height_validation(x)
    height_match = match(r"^(\d*?)(in|cm)$", x)

    if height_match === nothing
        return false
    end

    captures = height_match.captures
    height = parse(Int, string(captures[1]))

    if captures[2] == "in"
        return 59 <= height <= 76
    elseif captures[2] == "cm"
        return 150 <= height <= 193
    else
        return false
    end
end

function is_valid_passport(passport)
    validations = Dict(
        "byr" => x -> 1920 <= parse(Int, x) <= 2002,
        "iyr" => x -> 2010 <= parse(Int, x) <= 2020,
        "eyr" => x -> 2020 <= parse(Int, x) <= 2030,
        "hgt" => x -> height_validation(x),
        "hcl" => x -> match(r"^#[0-9a-f]{6}$", x) !== nothing,
        "ecl" => x -> x ∈ ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
        "pid" => x -> match(r"^[0-9]{9}$", x) !== nothing,
    )

    is_valid = true
    for (key, validator) ∈ pairs(validations)
        if !validator(passport[key])
            is_valid = false
            break
        end
    end

    return is_valid
end

function parse_passports(filename)
    raw_passports = split(strip(read(open(filename), String)), "\n\n")

    return [
        Dict(union([
            [split(kv, ":") for kv in split(property, " ")]
            for property in split(pp, "\n")
        ]...))
        for pp in raw_passports
    ]
end

function main()
    #filename = "valid.txt"
    filename = "problem.txt"

    required_fields = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        #"cid"
    ]

    passports = parse_passports(filename)

    valid_passports = [
        p
        for p in passports
        if length(intersect(keys(p), required_fields)) == length(required_fields)
    ]
    println(sum(length(valid_passports)))
    # 226

    counter = sum([is_valid_passport(p) for p in valid_passports])
    println(counter)
end

main()
