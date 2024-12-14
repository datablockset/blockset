const merge = ([a0, a1, a2, a3]) => a0 | (a1 << 128n) | (a2 << 256n) | (a3 << 384n)

const mask = (1n << 128n) - 1n

const split = a => [a & mask, (a >> 128n) & mask, (a >> 256n) & mask, (a >> 384n) & mask]

const div_rem = (a, b) => {
    const an = merge(a)
    const bn = merge(b)
    console.log('div_rem: ', split(an / bn), ', ', split(an % bn))
}

div_rem([0n, 0n, 0n, 0n], [1n, 0n, 0n, 0n])
div_rem([1n, 2n, 3n, 4n], [1n, 0n, 0n, 0n])
div_rem([1n, 2n, 3n, 4n], [2n, 0n, 0n, 0n])
div_rem([1n, 2n, 3n, 4n], [3n, 0n, 0n, 0n])
div_rem([1n, 2n, 3n, 4n], [3n, 5n, 0n, 0n])
div_rem([1n, 2n, 3n, 4n], [3n, 5n, 7n, 0n])
div_rem([1n, 2n, 3n, 4n], [3n, 5n, 7n, 1n])
div_rem([1n, 2n, 3n, 4n], [3n, 5n, 7n, 11n])
