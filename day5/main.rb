file = File.open("input.txt", "rb")
input = file.read

puts "Part 1"

def react_step(str)
    ("a".."z").each { |letter|
        unit1 = letter + letter.upcase
        unit2 = letter.upcase + letter

        str.gsub! unit1, ""
        str.gsub! unit2, ""
    }

    str
end

def react(str)
    last_length = 0
    while last_length != str.length
        last_length = str.length
        str = react_step(str)
    end

    last_length
end

p1input = input.dup
puts react(p1input)

puts "Part 2"

def clean(str, letter)
    str.gsub! letter, ""
    str.gsub! letter.upcase, ""
    str
end

shortest = Float::INFINITY
p2input = input.dup

("a".."z").each { |letter|
    cleaned = clean(p2input, letter)
    length = react(p2input)

    if length < shortest
        shortest = length
    end

    p2input = input.dup
}

puts shortest