BASE = 5
OFFSET = 2
DIGITS = '=-012'
DIGIT_VALUES = {'=': -2, '-': -1, '0': 0, '1': 1, '2': 2}


def decode(input):
    return sum(
        BASE ** place * DIGIT_VALUES[digit]
        for place, digit in enumerate(reversed(list(input)))
    )


def encode(input):
    quotient, remainder = divmod(input + OFFSET, BASE)
    last_character = DIGITS[remainder]
    if quotient == 0:
        return last_character
    return encode(quotient) + last_character


total_base10 = sum([decode(number.strip()) for number in open("input.txt").readlines()])
print(encode(total_base10))
