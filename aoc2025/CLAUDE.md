# Advent of Code Development Context

## Developer Profile
- **Name**: Eric
- **Primary Language**: Python
- **Background**: .NET/C# expert transitioning Python knowledge for AoC
- **Approach**: Methodical, prefers understanding root causes over quick hacks

## Python Preferences

### Code Style
- Clear, readable code over clever one-liners (unless explicitly optimizing)
- Descriptive variable names
- Comments explaining the "why" for complex logic
- Type hints where they add clarity

### Libraries & Tools
- Standard library preferred for most tasks
- Common imports: `collections` (Counter, defaultdict, deque), `itertools`, `functools`, `re`
- NumPy/SciPy if the problem clearly benefits from array operations
- `pytest` for testing when needed

### Problem-Solving Approach
1. **Parse carefully** - Don't rush the input parsing, get it right first
2. **Start simple** - Solve Part 1 cleanly before worrying about Part 2 optimizations
3. **Test with examples** - Validate against provided test cases
4. **Iterate on performance** - Only optimize when necessary (Part 2 usually requires rethinking)

## Advent of Code Specific

### Common Patterns to Consider
- **Grid/2D problems**: Use complex numbers for coordinates or dict with (x,y) tuples
- **Graph traversal**: BFS/DFS, consider when shortest path vs all paths matters
- **State space**: When to use sets vs lists for visited tracking
- **Memoization**: `@lru_cache` for recursive solutions with overlapping subproblems
- **Parsing**: Regex for structured input, split/strip for simple cases

We will create one folder per day and should be able to run the solution for a given day by using `python dayXX`. Tests should live in the day folders and be runnable using `pytest dayXX`.
Put common functions, such as parsing the input, in a shared file or folder.

### Code Organization
```python
def parse_input(filename):
    """Parse the puzzle input into usable data structures."""
    pass

def part1(data):
    """Solve part 1 of the puzzle."""
    pass

def part2(data):
    """Solve part 2 of the puzzle."""
    pass

if __name__ == "__main__":
    data = parse_input("input.txt")
    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")
```

### What I Want Help With
- **Algorithm selection**: Suggest appropriate approaches for the problem type
- **Performance insights**: When my solution won't scale to Part 2, explain why
- **Python idioms**: Show me more Pythonic ways to do things
- **Debugging**: Help trace through logic when answers are wrong
- **Optimization**: When brute force won't work, help me see the pattern

### What to Avoid
- Don't immediately jump to the most complex/optimal solution
- Don't use obscure Python tricks without explaining them
- Don't just give me the answer - help me understand the approach
- Don't assume I want to golf the code (unless I specifically ask)

## Communication Preferences
- Explain algorithmic complexity when it matters (O(n) vs O(n²) etc)
- If there's a clever insight needed, walk me through it
- Show me alternative approaches when relevant
- Be direct about when my approach won't scale

## Testing Notes
- I'll provide the example input/output from the puzzle description
- Help me verify solutions work on examples before trying real input
- When debugging, help me add strategic print statements or assertions

---

**Current Goal**: Solve daily puzzles, learn Python patterns, improve algorithm recognition